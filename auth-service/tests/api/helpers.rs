use core::panic;
use reqwest::cookie::Jar;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Connection, Executor, PgConnection, PgPool,
};
use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    app_state::{AppState, BannedTokenStoreType, TwoFACodeStoreType},
    get_postgres_pool, get_redis_client,
    services::{
        data_stores::{PostgresUserStore, RedisBannedTokenStore, RedisTwoFACodeStore},
        mock_email_client::MockEmailClient,
    },
    utils::constants::{test, DATABASE_URL, DEFAULT_REDIS_HOSTNAME},
    Application,
};

use std::str::FromStr;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
    pub http_client: reqwest::Client,
    pub db_name: String,
    pub clean_up_called: bool,
}

impl TestApp {
    pub async fn new() -> Self {
        let db_name = Uuid::new_v4().to_string();
        let pg_pool = configure_postgresql(&db_name).await;
        let redis_connection = Arc::new(RwLock::new(configure_redis()));

        let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
        let banned_token_store = Arc::new(RwLock::new(RedisBannedTokenStore::new(
            redis_connection.clone(),
        )));
        let two_fa_code_store = Arc::new(RwLock::new(RedisTwoFACodeStore::new(redis_connection)));

        let email_client = Arc::new(MockEmailClient);

        let app_state = AppState::new(
            user_store,
            banned_token_store.clone(),
            two_fa_code_store.clone(),
            email_client,
        );

        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        Self {
            address,
            cookie_jar,
            banned_token_store,
            two_fa_code_store,
            http_client,
            db_name,
            clean_up_called: false,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_2fa<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/verify-2fa", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn clean_up(&mut self) {
        if self.clean_up_called {
            return;
        }

        delete_database(&self.db_name).await;

        self.clean_up_called = true;
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        if !self.clean_up_called {
            panic!("TestApp::clean_up was not called before dropping TestApp");
        }
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}

async fn configure_postgresql(db_name: &str) -> PgPool {
    let postgresql_conn_url = DATABASE_URL.to_owned();

    configure_database(&postgresql_conn_url, db_name).await;

    let postgresql_conn_url_with_db = format!("{}/{}", postgresql_conn_url, db_name);

    get_postgres_pool(&postgresql_conn_url_with_db)
        .await
        .expect("Failed to create Postgres connection pool!")
}

async fn delete_database(db_name: &str) {
    let postgresql_conn_url = DATABASE_URL.to_owned();

    let connection_options = PgConnectOptions::from_str(&postgresql_conn_url)
        .expect("Failed to parse PostgreSQL connection string");

    let mut connection = PgConnection::connect_with(&connection_options)
        .await
        .expect("Failed to connect to Postgres");

    // Kill active connections to the database
    connection
        .execute(
            format!(
                r#"
                SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE pg_stat_activity.datname = '{}'
                  AND pid <> pg_backend_pid();
        "#,
                db_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to drop the database.");

    // Drop the database
    connection
        .execute(format!(r#"DROP DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to drop the database.");
}

async fn configure_database(db_conn_string: &str, db_name: &str) {
    let connection = PgPoolOptions::new()
        .connect(db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool.");

    // Create database
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to create database.");

    let db_conn_string = format!("{}/{}", db_conn_string, db_name);

    let connection = PgPoolOptions::new()
        .connect(&db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool.");

    // Migrate database
    sqlx::migrate!()
        .run(&connection)
        .await
        .expect("Failed to migrate the database");
}

fn configure_redis() -> redis::Connection {
    let redis_hostname = DEFAULT_REDIS_HOSTNAME.to_owned();

    get_redis_client(redis_hostname)
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis connection")
}
