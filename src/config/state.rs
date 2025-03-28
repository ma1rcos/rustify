use sqlx::postgres::PgPool;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use std::future::Future;

pub struct AppState {
    pub db: PgPool,
}

impl AppState {

    pub fn new(db: PgPool) -> Self {
        AppState { db }
    }
    pub async fn close_connection(&self) {
        self.db.close().await;
    }

}

pub static APP_STATE: Lazy<RwLock<Option<AppState>>> = Lazy::new(|| RwLock::new(None));

pub async fn initialize(
    database_pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), String> {
    match initialize_application_state(database_pool.clone()).await {
        Ok(_) => Ok(()),
        Err(error) => {
            eprintln!("Failed to initialize app state: {}", error);
            Err(error.to_string())
        }
    }
}

pub async fn initialize_application_state(db: PgPool) -> Result<(), String> {
    let mut app_state_guard = APP_STATE.write().await;
    *app_state_guard = Some(AppState::new(db));
    Ok(())
}

pub async fn shutdown_app_state() {
    let mut app_state_guard = APP_STATE.write().await;
    match app_state_guard.take() {
        Some(app_state) => {
            app_state.close_connection().await;
        },
        None => {
            eprintln!("AppState was already None during shutdown");
        }
    }
}

pub async fn with_app_state<F, Fut, T>(callback: F) -> T
where
    F: FnOnce(PgPool) -> Fut,
    Fut: Future<Output = T>,
{
    let app_state_guard = APP_STATE.read().await;
    match app_state_guard.as_ref() {
        Some(app_state) => {
            let db_pool = app_state.db.clone();
            callback(db_pool).await
        },
        None => panic!("AppState is not initialized"),
    }
}