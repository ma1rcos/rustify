use crate::core::administrator::handler::create_master;

pub async fn init_data() {
    create_master().await;
}