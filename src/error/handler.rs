use std::error::Error;
use actix_web::HttpResponse;
use crate::r#enum::message::Message;
use crate::model::response;
use crate::log::handler::create as create_log;

pub fn startup_error(context: &str, error: impl std::fmt::Display) -> std::io::Result<()> {
    eprintln!("Failed to {}: {}", context, error);
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("Failed to {}", context),
    ))
}

pub async fn save_and_return_error(error: &dyn Error) -> HttpResponse {
    create_log(serde_json::to_string(&error.to_string()).unwrap()).await;
    response::Response::get_internal_server_error(Message::UnknowError.get_message())
}

pub async fn save_error(error: &dyn Error) {
    create_log(serde_json::to_string(&error.to_string()).unwrap()).await;
}