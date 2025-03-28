use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize, Clone)]
pub struct Administrator {
    pub id: i32,
    pub avatar: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_master: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateAdministrator {
    pub avatar: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_master: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateReceivedAdministrator {
    pub avatar: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateAdministrator {
    pub avatar: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_master: bool,
}