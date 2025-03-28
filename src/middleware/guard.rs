use actix_web::{HttpRequest, HttpResponse};
use jsonwebtoken::{decode, DecodingKey, Algorithm, Validation};
use crate::core::auth::model::JwtPayload;
use crate::r#enum::message::Message;
use crate::model::response;
use crate::env::{
    MASTER_KEY, 
    ADMINISTRATOR_KEY, 
    CUSTOMER_KEY, 
    PHOTOGRAPHER_KEY,
    MASTER_PATH,
    ADMINISTRATOR_PATH,
    CUSTOMER_PATH,
    PHOTOGRAPHER_PATH
};

fn extract_bearer_token(request: &HttpRequest) -> Option<String> {
    request.headers()
        .get("Authorization")
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_str| {
            match auth_str.starts_with("Bearer ") {
                true => Some(auth_str.trim_start_matches("Bearer ").to_string()),
                false => None,
            }
        })
}

fn authorize_token(token: &str, secret: &str) -> Result<JwtPayload, ()> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);
    decode::<JwtPayload>(token, &decoding_key, &validation)
        .map(|token_data| token_data.claims)
        .map_err(|_| ())
}

trait Authorizer {
    fn secret_key() -> &'static str;
    fn allowed_paths() -> Vec<&'static str>;
    
    fn authorize(request: &HttpRequest) -> Option<HttpResponse> {
        extract_bearer_token(request)
            .and_then(|token| authorize_token(&token, Self::secret_key()).ok())
            .map_or_else(
                || Some(response::Response::get_forbidden(Message::NotAuthorized.get_message())),
                |_| None
            )
    }
    
    fn is_path_allowed(path: &str) -> bool {
        Self::allowed_paths().contains(&path)
    }
}

struct MasterAuthorizer;

impl Authorizer for MasterAuthorizer {
    fn secret_key() -> &'static str { &MASTER_KEY }
    fn allowed_paths() -> Vec<&'static str> { vec![&MASTER_PATH] }
}

struct AdministratorAuthorizer;

impl Authorizer for AdministratorAuthorizer {
    fn secret_key() -> &'static str { &ADMINISTRATOR_KEY }
    fn allowed_paths() -> Vec<&'static str> { vec![&ADMINISTRATOR_PATH, &MASTER_PATH] }
}

struct PhotographerAuthorizer;

impl Authorizer for PhotographerAuthorizer {
    fn secret_key() -> &'static str { &PHOTOGRAPHER_KEY }
    fn allowed_paths() -> Vec<&'static str> { vec![&PHOTOGRAPHER_PATH, &ADMINISTRATOR_PATH, &MASTER_PATH] }
}

struct CustomerAuthorizer;

impl Authorizer for CustomerAuthorizer {
    fn secret_key() -> &'static str { &CUSTOMER_KEY }
    fn allowed_paths() -> Vec<&'static str> { vec![&CUSTOMER_PATH, &ADMINISTRATOR_PATH, &MASTER_PATH] }
}

pub fn master_guard(request: &HttpRequest, path: String) -> Option<HttpResponse> {
    match MasterAuthorizer::is_path_allowed(&path) {
        true => MasterAuthorizer::authorize(request),
        false => Some(response::Response::get_forbidden(Message::NotAuthorized.get_message())),
    }
}

pub fn administrator_guard(request: &HttpRequest, path: String) -> Option<HttpResponse> {
    match AdministratorAuthorizer::is_path_allowed(&path) {
        true => AdministratorAuthorizer::authorize(request)
            .or_else(|| MasterAuthorizer::authorize(request)),
        false => Some(response::Response::get_forbidden(Message::NotAuthorized.get_message())),
    }
}

pub fn photographer_guard(request: &HttpRequest, path: String) -> Option<HttpResponse> {
    match PhotographerAuthorizer::is_path_allowed(&path) {
        true => PhotographerAuthorizer::authorize(request)
            .or_else(|| AdministratorAuthorizer::authorize(request))
            .or_else(|| MasterAuthorizer::authorize(request)),
        false => Some(response::Response::get_forbidden(Message::NotAuthorized.get_message())),
    }
}

pub fn customer_guard(request: &HttpRequest, path: String) -> Option<HttpResponse> {
    match CustomerAuthorizer::is_path_allowed(&path) {
        true => CustomerAuthorizer::authorize(request)
            .or_else(|| AdministratorAuthorizer::authorize(request))
            .or_else(|| MasterAuthorizer::authorize(request)),
        false => Some(response::Response::get_forbidden(Message::NotAuthorized.get_message())),
    }
}

pub fn any_guard(request: &HttpRequest, path: String) -> Option<HttpResponse> {
    match path.as_str() {
        path if path == *MASTER_PATH => MasterAuthorizer::authorize(request),
        path if path == *ADMINISTRATOR_PATH => 
            AdministratorAuthorizer::authorize(request)
                .or_else(|| MasterAuthorizer::authorize(request)),
        path if path == *PHOTOGRAPHER_PATH => 
            PhotographerAuthorizer::authorize(request)
                .or_else(|| AdministratorAuthorizer::authorize(request))
                .or_else(|| MasterAuthorizer::authorize(request)),
        path if path == *CUSTOMER_PATH => 
            CustomerAuthorizer::authorize(request)
                .or_else(|| AdministratorAuthorizer::authorize(request))
                .or_else(|| MasterAuthorizer::authorize(request)),
        _ => Some(response::Response::get_forbidden(Message::NotAuthorized.get_message())),
    }
}