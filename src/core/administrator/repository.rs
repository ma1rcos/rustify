use sqlx::Error;
use super::model::{CreateAdministrator, Administrator, UpdateAdministrator};
use crate::config::state::with_app_state;
use crate::error::handler::save_error;

pub async fn create(data: CreateAdministrator) -> Result<Administrator, Error> {
    with_app_state(|db_pool| async move {
        match sqlx::query_as::<_, Administrator>(
            r#"
            INSERT INTO administrator 
            (avatar, username, email, password, is_master) 
            VALUES ($1, $2, $3, $4, $5) 
            RETURNING *
            "#
        )
        .bind(&data.avatar)
        .bind(&data.username)
        .bind(&data.email)
        .bind(&data.password)
        .bind(&data.is_master)
        .fetch_one(&db_pool)
        .await
        {
            Ok(admin) => Ok(admin),
            Err(error) => {
                save_error(&error).await;
                Err(error)
            }
        }
    }).await
}

pub async fn _update(data: UpdateAdministrator, id: i32) -> Result<Administrator, Error> {
    with_app_state(|db_pool| async move {
        match sqlx::query_as::<_, Administrator>(
            r#"
            UPDATE administrator 
            SET avatar = $1, username = $2, email = $3, 
                password = $4, is_master = $5, 
                updated_at = NOW() 
            WHERE id = $6 
            RETURNING *
            "#
        )
        .bind(&data.avatar)
        .bind(&data.username)
        .bind(&data.email)
        .bind(&data.password)
        .bind(data.is_master)
        .bind(id)
        .fetch_one(&db_pool)
        .await
        {
            Ok(admin) => Ok(admin),
            Err(error) => {
                save_error(&error).await;
                Err(error)
            }
        }
    }).await
}

pub async fn _get_by_id(id: i32) -> Result<Option<Administrator>, Error> {
    with_app_state(|db_pool| async move {
        match sqlx::query_as::<_, Administrator>(
            r#"
            SELECT * FROM administrator 
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&db_pool)
        .await
        {
            Ok(admin) => Ok(admin),
            Err(error) => {
                save_error(&error).await;
                Err(error)
            }
        }
    }).await
}

pub async fn get_by_email(email: String) -> Result<Option<Administrator>, Error> {
    with_app_state(|db_pool| async move {
        match sqlx::query_as::<_, Administrator>(
            r#"
            SELECT * FROM administrator 
            WHERE email = $1
            "#
        )
        .bind(email)
        .fetch_optional(&db_pool)
        .await
        {
            Ok(admin) => Ok(admin),
            Err(error) => {
                save_error(&error).await;
                Err(error)
            }
        }
    }).await
}

pub async fn _delete(id: i32) -> Result<bool, Error> {
    with_app_state(|db_pool| async move {
        match sqlx::query(
            r#"
            DELETE FROM administrator 
            WHERE id = $1
            "#
        )
        .bind(id)
        .execute(&db_pool)
        .await
        {
            Ok(result) => Ok(result.rows_affected() > 0),
            Err(error) => {
                save_error(&error).await;
                Err(error)
            }
        }
    }).await
}