use dotenvy::dotenv;
use lazy_static::lazy_static;
use secrecy::Secret;
use std::env as std_env;

lazy_static! {
    pub static ref JWT_SECRET: Secret<String> = set_token();
    pub static ref DATABASE_URL: Secret<String> = set_db_url();
    pub static ref REDIS_HOST_NAME: String = set_redis_host();
    pub static ref POSTMARK_AUTH_TOKEN: Secret<String> = set_postmark_auth_token();
}

fn set_token() -> Secret<String> {
    dotenv().ok();
    let secret = std_env::var(env::JWT_SECRET_ENV_VAR).expect("JWT_SECRET must be set.");
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }
    Secret::new(secret)
}

fn set_db_url() -> Secret<String> {
    dotenv().ok();
    Secret::new(std_env::var(env::DATABASE_URL_ENV_VAR).expect("DATABASE_URL must be set."))
}

fn set_redis_host() -> String {
    dotenv().ok();
    std_env::var(env::REDIS_HOST_NAME_ENV_VAR).unwrap_or(DEFAULT_REDIS_HOSTNAME.to_owned())
}

fn set_postmark_auth_token() -> Secret<String> {
    dotenv().ok();
    Secret::new(
        std_env::var(env::POSTMARK_AUTH_TOKEN_ENV_VAR).expect("POSTMARK_AUTH_TOKEN must be set."),
    )
}

pub mod env {
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const REDIS_HOST_NAME_ENV_VAR: &str = "REDIS_HOST_NAME";
    pub const POSTMARK_AUTH_TOKEN_ENV_VAR: &str = "POSTMARK_AUTH_TOKEN";
}

pub const JWT_COOKIE_NAME: &str = "jwt";
pub const DEFAULT_REDIS_HOSTNAME: &str = "127.0.0.1";

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
    pub mod email_client {
        use std::time::Duration;

        pub const BASE_URL: &str = "https://api.postmarkapp.com/email";
        pub const SENDER: &str = "bogdan@codeiron.io";
        pub const TIMEOUT: Duration = std::time::Duration::from_secs(10);
    }
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
    pub mod email_client {
        use std::time::Duration;

        pub const SENDER: &str = "test@email.com";
        pub const TIMEOUT: Duration = std::time::Duration::from_millis(200);
    }
}
