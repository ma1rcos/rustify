mod env;
mod core;
mod service;
mod config;
mod error;
mod middleware;
mod r#enum;
mod model;
mod log;
mod seed;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::init_env();
    config::setup::logger();
    let database_pool = match config::database::initialize_database().await {
        Ok(pool) => pool,
        Err(error) => return error::handler::startup_error("Database connection: ", error),
    };
    match config::state::initialize(&database_pool).await {
        Ok(_) => (),
        Err(error) => return error::handler::startup_error("App state initialization: ", error),
    }
    seed::init_data().await;
    let server = config::server::start_http_server(database_pool).await;
    config::state::shutdown_app_state().await;
    server
}