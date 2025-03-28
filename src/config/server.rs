use actix_web::{web, App, HttpServer};
use std::time::Duration;
use crate::config::state::AppState;
use crate::core;

pub async fn start_http_server(
    database_pool: sqlx::Pool<sqlx::Postgres>,
) -> std::io::Result<()> {
    let workers = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db: database_pool.clone(),
            }))
            .configure(core::administrator::route::config)
            .configure(core::auth::route::config)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(workers)
    .backlog(1024) 
    .keep_alive(Duration::from_secs(75))
    .client_request_timeout(Duration::from_secs(5))
    .run()
    .await
}