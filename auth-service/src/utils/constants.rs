use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;


pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}


pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}


// Define a lazily evaluated static. lazy_static is needed because std_env::var is not a const function.
lazy_static! {
    pub static ref JWT_SECRET: String = set_token();
    pub static ref DATABASE_URL: String = set_db_url();
}


fn set_db_url() -> String {
    dotenv().ok();
    std_env::var(env::DATABASE_URL_ENV_VAR).expect("DATABASE_URL must be set.")
}


fn set_token() -> String {
    dotenv().ok(); // Load environment variables
    let secret = std_env::var(env::JWT_SECRET_ENV_VAR).expect("JWT_SECRET must be set.");
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }

    secret
}

pub mod env {
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
}

pub const JWT_COOKIE_NAME: &str = "jwt";
