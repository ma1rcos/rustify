use actix_web::{web, post, Responder};

use super::model::AuthBody;
use super::handler;

#[post("/administrator")]
pub async fn auth_administrator(body: web::Json<AuthBody>) -> impl Responder {
    handler::auth_administrator(body.into_inner()).await
}