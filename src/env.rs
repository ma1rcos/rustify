use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;

pub fn init_env() {
    dotenv().ok();
}

fn get_env_var(key: &str) -> String {
    env::var(key).expect(&format!("{} must be set.", key))
}

pub static MASTER_USERNAME: Lazy<String> = Lazy::new(|| get_env_var("MASTER_USERNAME"));
pub static MASTER_EMAIL: Lazy<String> = Lazy::new(|| get_env_var("MASTER_EMAIL"));
pub static MASTER_PASSWORD: Lazy<String> = Lazy::new(|| get_env_var("MASTER_PASSWORD"));
pub static MASTER_PATH: Lazy<String> = Lazy::new(|| get_env_var("MASTER_PATH"));
pub static ADMINISTRATOR_PATH: Lazy<String> = Lazy::new(|| get_env_var("ADMINISTRATOR_PATH"));
pub static ADMINISTRATOR_KEY: Lazy<String> = Lazy::new(|| get_env_var("ADMINISTRATOR_KEY"));
pub static MASTER_KEY: Lazy<String> = Lazy::new(|| get_env_var("MASTER_KEY"));
pub static DATABASE_URL: Lazy<String> = Lazy::new(|| get_env_var("DATABASE_URL"));