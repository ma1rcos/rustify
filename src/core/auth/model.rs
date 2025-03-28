use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};
use crate::r#enum::role::Role;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthBody {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JwtPayload {
    pub sub: i32,
    pub email: String,
    pub role: Role,
    pub timestamp: DateTime<Utc>,
    pub random_number: u8,
    pub exp: usize 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthOutput {
    pub token: String,
    pub path: String,
}