use sqlx::{pool::PoolConnection, Sqlite};

use crate::error::Result;

#[derive(Clone, Debug)]
pub struct Feedback {
    pub comment: Option<String>,
    pub rating: i64,
}

pub async fn create_feedback(
    owner_id: &str,
    card_id: &str,
    conn: &mut PoolConnection<Sqlite>,
) -> Result<()> {
    sqlx::query!(
        "INSERT INTO feedback (id, owner_id) VALUES (?, ?)",
        card_id,
        owner_id,
    )
    .execute(&mut **conn)
    .await?;

    Ok(())
}

pub async fn add_report(
    card_id: &str,
    report_id: &str,
    conn: &mut PoolConnection<Sqlite>,
) -> Result<()> {
    sqlx::query!(
        "UPDATE feedback SET report_id = ? WHERE id = ?",
        report_id,
        card_id
    )
    .execute(&mut **conn)
    .await?;

    Ok(())
}

pub async fn create_or_update_feedback_entry(
    feedback_id: &str,
    user_id: &str,
    rating: i64,
    comment: Option<&str>,
    conn: &mut PoolConnection<Sqlite>,
) -> Result<()> {
    sqlx::query!(
        "INSERT INTO feedback_entry (feedback_id, user_id, rating, comment) VALUES (?, ?, ?, ?) ON CONFLICT (feedback_id, user_id) DO UPDATE SET rating = ?, comment = ?",
        feedback_id,
        user_id,
        rating,
        comment,
        rating,
        comment,
    )
    .execute(&mut **conn)
    .await?;

    Ok(())
}

pub async fn get_feedbacks_by_id(
    feedback_id: &str,
    conn: &mut PoolConnection<Sqlite>,
) -> Result<Vec<Feedback>> {
    let result = sqlx::query_as!(
        Feedback,
        "SELECT rating, comment FROM feedback_entry WHERE feedback_id = ?",
        feedback_id
    )
    .fetch_all(&mut **conn)
    .await?;

    Ok(result)
}

pub struct FeedbackMeta {
    pub conversation_id: Option<String>,
    pub owner_id: String,
    pub report_id: Option<String>,
}

pub async fn get_feedback_by_id(
    card_id: &str,
    conn: &mut PoolConnection<Sqlite>,
) -> Result<FeedbackMeta> {
    let result = sqlx::query_as!(
        FeedbackMeta,
        "SELECT 
            user.conversation_id,
            feedback.owner_id,
            feedback.report_id 
        FROM
            feedback 
            JOIN user ON feedback.owner_id = user.id
        WHERE 
            feedback.id = ?",
        card_id
    )
    .fetch_one(&mut **conn)
    .await?;

    Ok(result)
}
