use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Error;
use crate::env;

pub async fn establish_connection_database() -> Result<PgPool, Error> {
    let database_url: &str = &env::DATABASE_URL;
    PgPoolOptions::new()
        .max_connections(100)
        .min_connections(5) 
        .connect(database_url)
        .await
}

pub async fn initialize_database() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    establish_connection_database().await
}