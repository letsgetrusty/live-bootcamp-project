use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::Utc;
use color_eyre::eyre::{eyre, Context, ContextCompat, Result};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Validation};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

use crate::{app_state::BannedTokenStoreType, domain::email::Email};

use super::constants::{JWT_COOKIE_NAME, JWT_SECRET};

#[tracing::instrument(name = "Generate auth cookie", skip_all)]
pub fn generate_auth_cookie(email: &Email) -> Result<Cookie<'static>> {
    let token = generate_auth_token(email)?;
    Ok(create_auth_cookie(token))
}

#[tracing::instrument(name = "Create auth cookie", skip_all)]
fn create_auth_cookie(token: Secret<String>) -> Cookie<'static> {
    let cookie = Cookie::build((JWT_COOKIE_NAME, token.expose_secret().to_owned()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    cookie
}

pub const TOKEN_TTL_SECONDS: i64 = 600;

#[tracing::instrument(name = "Generate auth token", skip_all)]
fn generate_auth_token(email: &Email) -> Result<Secret<String>> {
    let delta = chrono::Duration::try_seconds(TOKEN_TTL_SECONDS)
        .wrap_err("failed to create 10 minute time delta")?;

    let exp = Utc::now()
        .checked_add_signed(delta)
        .ok_or(eyre!("failed to add 10 minutes to current time"))?
        .timestamp();

    let exp: usize = exp.try_into().wrap_err(format!(
        "failed to cast exp time to usize. exp time: {}",
        exp
    ))?;

    let sub = email.as_ref().expose_secret().to_owned();

    let claims = Claims { sub, exp };

    create_token(&claims)
}

#[tracing::instrument(name = "Validate token", skip_all)]
pub async fn validate_token(
    token: &Secret<String>,
    banned_token_store: BannedTokenStoreType,
) -> Result<Claims> {
    match banned_token_store.read().await.contains_token(token).await {
        Ok(value) => {
            if value {
                return Err(eyre!("token is banned"));
            }
        }
        Err(e) => return Err(e.into()),
    }

    decode::<Claims>(
        token.expose_secret(),
        &DecodingKey::from_secret(JWT_SECRET.expose_secret().as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .wrap_err("failed to decode token")
}

#[tracing::instrument(name = "Create token", skip_all)]
fn create_token(claims: &Claims) -> Result<Secret<String>> {
    encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.expose_secret().as_bytes()),
    )
    .map(Secret::new)
    .wrap_err("failed to create token")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use tokio::sync::RwLock;

    use crate::{domain::BannedTokenStore, services::data_stores::HashsetBannedTokenStore};

    use super::*;

    #[tokio::test]
    async fn test_generate_auth_cookie() {
        let email = Email::parse(Secret::new("test@example.com".to_owned())).unwrap();
        let cookie = generate_auth_cookie(&email).unwrap();
        assert_eq!(cookie.name(), JWT_COOKIE_NAME);
        assert_eq!(cookie.value().split('.').count(), 3);
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    }

    #[tokio::test]
    async fn test_create_auth_cookie() {
        let token = Secret::new("test_token".to_owned());
        let cookie = create_auth_cookie(token.clone());
        assert_eq!(cookie.name(), JWT_COOKIE_NAME);
        assert_eq!(cookie.value(), token.expose_secret());
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    }

    #[tokio::test]
    async fn test_generate_auth_token() {
        let email = Email::parse(Secret::new("test@example.com".to_owned())).unwrap();
        let result = generate_auth_token(&email).unwrap();
        assert_eq!(result.expose_secret().split('.').count(), 3);
    }

    #[tokio::test]
    async fn test_validate_token_with_valid_token() {
        let email = Email::parse(Secret::new("test@example.com".to_owned())).unwrap();
        let token = generate_auth_token(&email).unwrap();
        let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
        let result = validate_token(&token, banned_token_store).await.unwrap();
        assert_eq!(result.sub, "test@example.com");

        let exp = Utc::now()
            .checked_add_signed(chrono::Duration::try_minutes(9).expect("valid duration"))
            .expect("valid timestamp")
            .timestamp();

        assert!(result.exp > exp as usize);
    }

    #[tokio::test]
    async fn test_validate_token_with_invalid_token() {
        let token = Secret::new("invalid_token".to_owned());
        let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
        let result = validate_token(&token, banned_token_store).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_token_with_banned_token() {
        let email = Email::parse(Secret::new("test@example.com".to_owned())).unwrap();
        let token = generate_auth_token(&email).unwrap();
        let mut hs = HashsetBannedTokenStore::default();
        hs.add_token(token.clone()).await.unwrap();
        let banned_token_store = Arc::new(RwLock::new(hs));
        let result = validate_token(&token, banned_token_store).await;
        assert!(result.is_err());
    }
}
