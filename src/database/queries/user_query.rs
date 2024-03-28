use sqlx::{Executor, Sqlite};

use crate::error::Result;

pub async fn create_user<'a, E>(user_id: &str, name: &str, executor: E) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    sqlx::query!("INSERT INTO user (id, name) SELECT ?, ? WHERE NOT EXISTS (SELECT * FROM user WHERE id = ?)",
        user_id,
        name,
        user_id
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn create_user_with_conversation<'a, E>(
    user_id: &str,
    name: &str,
    conversation_id: &str,
    executor: E,
) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    sqlx::query!(
        "INSERT INTO user (id, name, conversation_id) VALUES (?, ?, ?)",
        user_id,
        name,
        conversation_id
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn update_conversation<'a, E>(
    user_id: &str,
    conversation_id: &str,
    executor: E,
) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    sqlx::query!(
        "UPDATE user SET conversation_id = ? WHERE id = ?",
        conversation_id,
        user_id,
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn get_conversation_by_id<'a, E>(id: &str, executor: E) -> Result<Option<Option<String>>>
where
    E: Executor<'a, Database = Sqlite>,
{
    let result = sqlx::query_scalar!("SELECT conversation_id FROM user WHERE id = ?", id)
        .fetch_optional(executor)
        .await?;

    Ok(result)
}
