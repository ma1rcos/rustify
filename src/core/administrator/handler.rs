use actix_web::HttpResponse;
use crate::{
    r#enum::message::Message,
    env::{MASTER_USERNAME, MASTER_EMAIL, MASTER_PASSWORD},
    model::response,
    service::bcrypt,
    error::handler::{save_and_return_error, save_error}
};
use super::{
    model::{Administrator, CreateAdministrator, CreateReceivedAdministrator},
    repository
};

struct AdministratorService;

impl AdministratorService {
    
    fn prepare_administrator_data(data: CreateReceivedAdministrator, is_master: bool) -> Result<CreateAdministrator, bcrypt::BcryptError> {
        let password_hash = bcrypt::generate_hash(data.password.trim())?;
        Ok(CreateAdministrator {
            avatar: data.avatar.map(|avatar| avatar.trim().to_string()),
            username: data.username.trim().to_string(),
            email: data.email.trim().to_string(),
            password: password_hash,
            is_master,
        })
    }

    async fn create_administrator(data: CreateAdministrator) -> Result<HttpResponse, HttpResponse> {
        match repository::create(data).await {
            Ok(created) => {
                let created_json = serde_json::to_string(&created)
                    .map_err(|e| save_and_return_error(&e).await)?;
                
                Ok(response::Response::get_created(
                    Message::UserCreatedSuccessfully.get_message(),
                    created_json
                ))
            },
            Err(error) => Err(save_and_return_error(&error).await)
        }
    }

    async fn is_email_available(email: &str) -> Result<bool, HttpResponse> {
        match repository::get_by_email(email.to_string()).await {
            Ok(Some(_)) => Ok(false),
            Ok(None) => Ok(true),
            Err(error) => Err(save_and_return_error(&error).await)
        }
    }

    pub async fn get_by_email(email: String) -> Option<Administrator> {
        repository::get_by_email(email)
            .await
            .unwrap_or(None)
    }

    pub async fn create(body: CreateReceivedAdministrator) -> HttpResponse {
        match Self::prepare_administrator_data(body, false) {
            Ok(data) => {
                match Self::is_email_available(&data.email).await {
                    Ok(true) => {
                        match Self::create_administrator(data).await {
                            Ok(response) => response,
                            Err(response) => response
                        }
                    },
                    Ok(false) => response::Response::get_conflict(Message::EmailAlreadyUsed.get_message()),
                    Err(response) => response
                }
            },
            Err(error) => save_and_return_error(&error).await
        }
    }

    pub async fn create_master() {
        let body = CreateReceivedAdministrator {
            avatar: None,
            username: MASTER_USERNAME.to_string(),
            email: MASTER_EMAIL.to_string(),
            password: MASTER_PASSWORD.to_string(),
        };

        match Self::prepare_administrator_data(body, true) {
            Ok(data) => {
                match Self::is_email_available(&data.email).await {
                    Ok(true) => {
                        if let Err(error) = repository::create(data).await {
                            save_error(&error).await;
                        } else {
                            println!("Master admin created successfully!");
                        }
                    },
                    Ok(false) => (),
                    Err(error_response) => error_response
                }
            },
            Err(error) => save_error(&error).await
        }
    }
}

pub async fn get_by_email(email: String) -> Option<Administrator> {
    AdministratorService::get_by_email(email).await
}

pub async fn create(body: CreateReceivedAdministrator) -> HttpResponse {
    AdministratorService::create(body).await
}

pub async fn create_master() {
    AdministratorService::create_master().await
}