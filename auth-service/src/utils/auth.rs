use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::{app_state::BannedTokenStoreType, domain::email::Email};

use super::constants::{JWT_COOKIE_NAME, JWT_SECRET};

pub fn generate_auth_cookie(email: &Email) -> Result<Cookie<'static>, GenerateTokenError> {
    let token = generate_auth_token(email)?;
    Ok(create_auth_cookie(token))
}

fn create_auth_cookie(token: String) -> Cookie<'static> {
    let cookie = Cookie::build((JWT_COOKIE_NAME, token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    cookie
}

#[derive(Debug)]
pub enum GenerateTokenError {
    TokenError(jsonwebtoken::errors::Error),
    UnexpectedError,
}

pub const TOKEN_TTL_SECONDS: i64 = 600;

fn generate_auth_token(email: &Email) -> Result<String, GenerateTokenError> {
    let delta = chrono::Duration::try_seconds(TOKEN_TTL_SECONDS)
        .ok_or(GenerateTokenError::UnexpectedError)?;

    let exp = Utc::now()
        .checked_add_signed(delta)
        .ok_or(GenerateTokenError::UnexpectedError)?
        .timestamp();

    let exp: usize = exp
        .try_into()
        .map_err(|_| GenerateTokenError::UnexpectedError)?;

    let sub = email.as_ref().to_owned();

    let claims = Claims { sub, exp };

    create_token(&claims).map_err(GenerateTokenError::TokenError)
}

pub async fn validate_token(
    token: &str,
    banned_token_store: BannedTokenStoreType,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    match banned_token_store.read().await.contains_token(token).await {
        Ok(value) => {
            if value {
                return Err(jsonwebtoken::errors::Error::from(
                    jsonwebtoken::errors::ErrorKind::InvalidToken,
                ));
            }
        }
        Err(_) => {
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidToken,
            ));
        }
    }

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

fn create_token(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
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

    use crate::{
        domain::BannedTokenStore, services::hashset_banned_token_store::HashsetBannedTokenStore,
    };

    use super::*;

    #[tokio::test]
    async fn test_generate_auth_cookie() {
        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let cookie = generate_auth_cookie(&email).unwrap();
        assert_eq!(cookie.name(), JWT_COOKIE_NAME);
        assert_eq!(cookie.value().split('.').count(), 3);
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    }

    #[tokio::test]
    async fn test_create_auth_cookie() {
        let token = "test_token".to_owned();
        let cookie = create_auth_cookie(token.clone());
        assert_eq!(cookie.name(), JWT_COOKIE_NAME);
        assert_eq!(cookie.value(), token);
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    }

    #[tokio::test]
    async fn test_generate_auth_token() {
        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let result = generate_auth_token(&email).unwrap();
        assert_eq!(result.split('.').count(), 3);
    }

    #[tokio::test]
    async fn test_validate_token_with_valid_token() {
        let email = Email::parse("test@example.com".to_owned()).unwrap();
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
        let token = "invalid_token".to_owned();
        let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
        let result = validate_token(&token, banned_token_store).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_token_with_banned_token() {
        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let token = generate_auth_token(&email).unwrap();
        let mut hs = HashsetBannedTokenStore::default();
        hs.add_token(token.clone()).await.unwrap();
        let banned_token_store = Arc::new(RwLock::new(hs));
        let result = validate_token(&token, banned_token_store).await;
        assert!(result.is_err());
    }
}
