use actix_web::{post, web, HttpRequest, HttpResponse};
use super::model::CreateReceivedAdministrator;
use super::handler;
use crate::middleware::guard;

#[post("/create/{path}")]
pub async fn create(
    request: HttpRequest, 
    body: web::Json<CreateReceivedAdministrator>, 
    path: web::Path<String>
) -> HttpResponse {
    match guard::master_guard(&request, path.into_inner()) {
        Some(response) => response,
        None => handler::create(body.into_inner()).await
    }
}