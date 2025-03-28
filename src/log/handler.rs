use super::repository;
use std::fs::OpenOptions;
use std::io::{self, Write};
use chrono::Utc;
use super::model::CreateLog;

fn write_to_file(file_path: &str, data: &str) -> io::Result<()> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?
        .write_all(data.as_bytes())
}

fn log_error_to_file(file_path: &str, message: &str) {
    let data = CreateLog {
        message: message.to_string(),
        timestamp: Utc::now(),
    };
    match serde_json::to_string(&data) {
        Ok(log_string) => {
            let _ = match write_to_file(file_path, &log_string) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            };
        }
        Err(_) => {}
    }
}

pub async fn create(message: String) {
    match repository::create(message.clone()).await {
        Ok(_) => {}
        Err(_) => {
            let log_file_path = "log.txt";
            log_error_to_file(log_file_path, &message);
        }
    }
}