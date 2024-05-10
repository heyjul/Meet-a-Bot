use sqlx::{Executor, Postgres};

use crate::error::Result;

#[derive(Clone, Debug)]
pub struct Feedback {
    pub conversation_name: String,
    pub comment: Option<String>,
    pub rating: i64,
}

pub async fn create_feedback<'a, E>(
    owner_id: &str,
    card_id: &str,
    conversation_name: &str,
    executor: E,
) -> Result<()>
where
    E: Executor<'a, Database = Postgres>,
{
    sqlx::query!(
        "INSERT INTO feedback (id, owner_id, conversation_name) VALUES ($1, $2, $3)",
        card_id,
        owner_id,
        conversation_name
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn add_report<'a, E>(card_id: &str, report_id: &str, executor: E) -> Result<()>
where
    E: Executor<'a, Database = Postgres>,
{
    sqlx::query!(
        "UPDATE feedback SET report_id = $1 WHERE id = $2",
        report_id,
        card_id
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn create_or_update_feedback_entry<'a, E>(
    feedback_id: &str,
    user_id: &str,
    rating: i32,
    comment: Option<&str>,
    executor: E,
) -> Result<()>
where
    E: Executor<'a, Database = Postgres>,
{
    sqlx::query!(
        "INSERT INTO feedback_entry (feedback_id, user_id, rating, comment) VALUES ($1, $2, $3, $4) ON CONFLICT (feedback_id, user_id) DO UPDATE SET rating = $3, comment = $4",
        feedback_id,
        user_id,
        rating,
        comment,
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn get_feedbacks_by_id<'a, E>(feedback_id: &str, executor: E) -> Result<Vec<Feedback>>
where
    E: Executor<'a, Database = Postgres>,
{
    let result = sqlx::query_as!(
        Feedback,
        r#"SELECT 
            conversation_name, 
            rating, 
            comment 
        FROM 
            feedback f
            JOIN feedback_entry fe ON f.id = fe.feedback_id
        WHERE 
            feedback_id = $1"#,
        feedback_id
    )
    .fetch_all(executor)
    .await?;

    Ok(result)
}

pub struct FeedbackMetadata {
    pub conversation_id: Option<String>,
    pub owner_id: String,
    pub report_id: Option<String>,
}

pub async fn get_feedback_by_id<'a, E>(card_id: &str, executor: E) -> Result<FeedbackMetadata>
where
    E: Executor<'a, Database = Postgres>,
{
    let result = sqlx::query_as!(
        FeedbackMetadata,
        "SELECT 
            \"user\".conversation_id,
            feedback.owner_id,
            feedback.report_id 
        FROM
            feedback 
            JOIN \"user\" ON feedback.owner_id = \"user\".id
        WHERE 
            feedback.id = $1",
        card_id
    )
    .fetch_one(executor)
    .await?;

    Ok(result)
}
