use sqlx::{Acquire, PgPool, Postgres, Transaction};
use tracing::warn;

use crate::{
    database::queries::{self, feedback_query::FeedbackMetadata},
    error::{Error, Result},
    models::{
        self,
        activity::{Activity, Type},
        attachment::ContentType,
        Attachment, ChannelAccount, ConversationParameters,
    },
    services::{graph_client::GraphClient, teams_client::TeamsClient},
};

use super::send_adaptive_card;

const EMPTY_STAR: &str = include_str!("../assets/empty_star");
const HALF_STAR: &str = include_str!("../assets/half_star");
const FULL_STAR: &str = include_str!("../assets/full_star");
const FEEDBACK_CARD: &str = include_str!("../assets/feedback_card.json");
const FEEDBACK_REPORT: &str = include_str!("../assets/feedback_report.json");
const FALLBACK_NAME: &str = "Unknown";

pub async fn send_feedback_card(
    teams_client: &TeamsClient,
    graph_client: &GraphClient,
    pool: &PgPool,
    activity: &Activity,
) -> Result<()> {
    let name = activity.from.name.as_deref().unwrap_or(FALLBACK_NAME);

    let response = send_adaptive_card(
        teams_client,
        activity,
        &serde_json::from_str(&FEEDBACK_CARD.replace("{name}", name))?,
    )
    .await?;

    let user_id = &activity.from.id;
    let chat = graph_client.get_chat(&activity.conversation.id).await;
    let chat_name = match chat {
        Ok(ref chat) => &chat.topic,
        Err(e) => {
            warn!("An error occured while fetching the chat name : {:?}", e);
            FALLBACK_NAME
        }
    };

    let mut conn = pool.acquire().await?;
    let mut tx = conn.begin().await?;

    queries::user_query::create_user(user_id, name, &mut *tx).await?;
    queries::feedback_query::create_feedback(user_id, &response.id, &chat_name, &mut *tx).await?;

    tx.commit().await?;

    Ok(())
}

pub async fn handle_feedback_entry(
    client: &TeamsClient,
    pool: &PgPool,
    activity: &Activity,
    feedback: &models::action::Feedback,
) -> Result<()> {
    let card_id = activity
        .reply_to_id
        .as_ref()
        .ok_or(Error::MissingValue("reply_to_id"))?;

    let user_id = &activity.from.id;

    let (base_url, mut response) = activity.create_response();

    let mut conn = pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let FeedbackMetadata {
        conversation_id,
        owner_id,
        report_id,
    } = queries::feedback_query::get_feedback_by_id(card_id, &mut *tx).await?;

    let conversation_id = get_or_create_conversation(
        client,
        base_url,
        conversation_id,
        activity,
        &owner_id,
        &mut tx,
    )
    .await?;

    queries::feedback_query::create_or_update_feedback_entry(
        card_id,
        user_id,
        feedback.rating,
        feedback.comment.as_deref(),
        &mut *tx,
    )
    .await?;

    let feedbacks = queries::feedback_query::get_feedbacks_by_id(card_id, &mut *tx).await?;

    let content = get_feedback_report_adaptive_card(&feedbacks)?;

    response.recipient = ChannelAccount::default();
    response.r#type = Type::Message;
    response.attachments = Some(vec![Attachment {
        content: Some(content),
        content_type: Some(ContentType::Adaptive),
    }]);

    match report_id {
        Some(report_id) => {
            client
                .update_activity(base_url, &conversation_id, &report_id, &response)
                .await?;
        }
        None => {
            let response = client
                .send_to_conversation(base_url, &conversation_id, &response)
                .await?;

            queries::feedback_query::add_report(card_id, &response.id, &mut *tx).await?;
        }
    }

    tx.commit().await?;

    Ok(())
}

fn get_feedback_report_adaptive_card(
    feedbacks: &[queries::feedback_query::Feedback],
) -> Result<serde_json::Value> {
    let comments: Vec<_> = feedbacks
        .iter()
        .filter_map(|feedback| {
            feedback.comment.as_ref().map(|x| {
                serde_json::json!({
                    "type": "TextBlock",
                    "text": x,
                    "wrap": true
                })
            })
        })
        .collect();

    let average = feedbacks.iter().map(|x| x.rating).sum::<i64>() as f32 / feedbacks.len() as f32;

    let (_, stars) = (0..5).fold(
        (average, Vec::with_capacity(5)),
        |(average, mut stars), _| {
            let star = match average {
                x if x >= 1.0 => FULL_STAR,
                x if x >= 0.5 => HALF_STAR,
                _ => EMPTY_STAR,
            };

            stars.push(serde_json::json!({
                "type": "Column",
                "width": "stretch",
                "items": [
                    {
                        "type": "Image",
                        "url": star,
                        "size": "Small",
                        "horizontalAlignment": "Center"
                    }
                ]
            }));

            (average - 1.0, stars)
        },
    );

    let feedbacks_count = feedbacks.len();
    let comments_count = comments.len();
    let name = &feedbacks
        .first()
        .expect("Feedbacks should not be empty")
        .conversation_name;

    let mut feedback_report: serde_json::Value = serde_json::from_str(
        &FEEDBACK_REPORT
            .replace("{name}", name)
            .replace("{comments_count}", &comments_count.to_string())
            .replace("{feedbacks_count}", &feedbacks_count.to_string()),
    )?;

    feedback_report["body"][3]["items"] = serde_json::Value::Array(comments);
    feedback_report["body"][5]["columns"] = serde_json::Value::Array(stars);

    Ok(feedback_report)
}

async fn get_or_create_conversation(
    client: &TeamsClient,
    base_url: Option<&str>,
    conversation_id: Option<String>,
    activity: &Activity,
    user_id: &str,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<String> {
    match conversation_id {
        Some(conversation_id) => Ok(conversation_id),
        None => {
            let conversation_id = create_conversation(client, base_url, activity, user_id).await?;
            queries::user_query::update_conversation(user_id, &conversation_id, &mut **tx).await?;

            Ok(conversation_id)
        }
    }
}

async fn create_conversation(
    client: &TeamsClient,
    base_url: Option<&str>,
    activity: &Activity,
    user_id: &str,
) -> Result<String> {
    let conversation_response = client
        .create_conversation(
            base_url,
            &ConversationParameters {
                bot: activity.recipient.clone(),
                members: Some(vec![ChannelAccount {
                    id: user_id.to_owned(),
                    ..Default::default()
                }]),
                tenant_id: activity.conversation.tenant_id.clone(),
            },
        )
        .await?;

    Ok(conversation_response.id)
}
