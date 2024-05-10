use sqlx::{Executor, Postgres};

use crate::error::Result;

pub async fn create_user<'a, E>(user_id: &str, name: &str, executor: E) -> Result<()>
where
    E: Executor<'a, Database = Postgres>,
{
    sqlx::query!("INSERT INTO \"user\" (id, name) SELECT $1, $2 WHERE NOT EXISTS (SELECT * FROM \"user\" WHERE id = $1)",
        user_id,
        name,
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
    E: Executor<'a, Database = Postgres>,
{
    sqlx::query!(
        "INSERT INTO \"user\" (id, name, conversation_id) VALUES ($1, $2, $3)",
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
    E: Executor<'a, Database = Postgres>,
{
    sqlx::query!(
        "UPDATE \"user\" SET conversation_id = $1 WHERE id = $2",
        conversation_id,
        user_id,
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn get_conversation_by_id<'a, E>(id: &str, executor: E) -> Result<Option<Option<String>>>
where
    E: Executor<'a, Database = Postgres>,
{
    let result = sqlx::query_scalar!("SELECT conversation_id FROM \"user\" WHERE id = $1", id)
        .fetch_optional(executor)
        .await?;

    Ok(result)
}
