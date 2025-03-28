use actix_web::Responder;
use chrono::Utc;
use dotenv::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm, errors::Error as JwtError};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::{
    r#enum::{role::Role, message::Message},
    error::handler::save_and_return_error,
    model::response,
    core::administrator::handler::get_by_email as get_administrator_by_email,
    service::bcrypt::is_match,
    log::handler::create as create_log,
    env,
};
use super::model::{AuthBody, JwtPayload, AuthOutput};

const TOKEN_EXPIRATION_SECONDS: u64 = 604800;

struct AuthService;

impl AuthService {
    
    fn generate_authorization(sub: i32, email: String, role: Role) -> Result<String, JwtError> {
        let exp = Self::calculate_expiration();
        let key = Self::get_secret_key(&role);
        let encoding_key = EncodingKey::from_secret(key.as_ref());
        let payload = JwtPayload {
            sub,
            email: email.clone(),
            role,
            timestamp: Utc::now(),
            random_number: email.len() as u8,
            exp,
        };
        let token = encode(&Header::new(Algorithm::HS256), &payload, &encoding_key)?;
        let path = Self::get_role_path(&role);
        let auth_output = AuthOutput { token, path };
        serde_json::to_string(&auth_output).map_err(|e| JwtError::from(e))
    }

    fn calculate_expiration() -> usize {
        SystemTime::now()
            .checked_add(std::time::Duration::new(TOKEN_EXPIRATION_SECONDS, 0))
            .unwrap_or(UNIX_EPOCH)
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::new(0, 0))
            .as_secs() as usize
    }

    fn get_secret_key(role: &Role) -> String {
        dotenv().ok();
        match role {
            Role::Master => env::MASTER_KEY.to_string(),
            Role::Administrator => env::ADMINISTRATOR_KEY.to_string(),
            Role::Photographer => env::PHOTOGRAPHER_KEY.to_string(),
            Role::Customer => env::CUSTOMER_KEY.to_string(),
        }
    }

    fn get_role_path(role: &Role) -> String {
        match role {
            Role::Master => env::MASTER_PATH.to_string(),
            Role::Administrator => env::ADMINISTRATOR_PATH.to_string(),
            Role::Photographer => env::PHOTOGRAPHER_PATH.to_string(),
            Role::Customer => env::CUSTOMER_PATH.to_string(),
        }
    }

}

pub async fn auth_administrator(body: AuthBody) -> impl Responder {
    
    let administrator = match get_administrator_by_email(body.email.clone()).await {
        Some(admin) => admin,
        None => return response::Response::get_not_found(Message::UserNotFound.get_message()),
    };

    match is_match(&body.password, &administrator.password) {
        false => return response::Response::get_unauthorized(Message::InvalidCredentials.get_message()),
        true => (),
    }

    let role = match administrator.is_master {
        true => Role::Master,
        false => Role::Administrator,
    };

    match AuthService::generate_authorization(administrator.id, administrator.email, role) {
        Ok(authorization) => {
            response::Response::get_success(
                Message::UserAuthenticatedSuccessfully.get_message(),
                authorization,
            )
        }
        Err(error) => {
            create_log(serde_json::to_string(&body).unwrap()).await;
            save_and_return_error(&error).await
        }
    }
    
}