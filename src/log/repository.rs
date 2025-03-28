use crate::config::state::with_app_state;

pub async fn create(message: String) -> Result<(), sqlx::Error> {
    with_app_state(|db_pool| async move {
        sqlx::query_as::<_, _>(
            "INSERT INTO log (message) VALUES ($1) RETURNING *"
        )
        .bind(&message)
        .fetch_one(&db_pool)
        .await
    }).await
}