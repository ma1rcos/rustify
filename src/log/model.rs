use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct CreateLog {
    pub message: String,
    pub timestamp: DateTime<Utc>,
}