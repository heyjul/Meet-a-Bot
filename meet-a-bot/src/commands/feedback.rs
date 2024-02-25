use serde::Serialize;
use sqlx::{pool::PoolConnection, Sqlite, SqlitePool};
use teams_api::{
    client::TeamsBotClient,
    models::{
        activity::Type, attachment::ContentType, requests::ConversationParameters, Activity,
        Attachment, ChannelAccount,
    },
};

use crate::{
    database::queries::{self, feedback::FeedbackMeta},
    models,
};

use super::send_adaptive_card;

const EMPTY_STAR: &str = include_str!("../assets/empty_star");
const HALF_STAR: &str = include_str!("../assets/half_star");
const FULL_STAR: &str = include_str!("../assets/full_star");

struct ActivityFrom<'a> {
    id: &'a str,
    name: &'a str,
}

fn try_extract_from(activity: &Activity) -> Result<ActivityFrom, ()> {
    match activity.from {
        Some(ChannelAccount {
            id: Some(ref id),
            name: Some(ref name),
            aad_object_id: _,
            role: _,
        }) => Ok(ActivityFrom { id, name }),
        _ => Err(()),
    }
}

pub async fn send_feedback_card(client: &TeamsBotClient, pool: &SqlitePool, activity: &Activity) {
    let response = send_adaptive_card(
        client,
        activity,
        &serde_json::json!({
            "type": "AdaptiveCard",
            "$schema": "http://adaptivecards.io/schemas/adaptive-card.json",
            "version": "1.5",
            "body": [
                {
                    "type": "TextBlock",
                    "text": "Spank me daddy !",
                    "wrap": true,
                    "style": "heading"
                },
                {
                    "type": "Input.Text",
                    "placeholder": "Was I a good boy ?",
                    "id": "comment",
                    "isMultiline": true
                },
                {
                    "type": "ColumnSet",
                    "style": "emphasis",
                    "horizontalAlignment": "Right",
                    "columns": [
                        {
                            "type": "Column",
                            "items": [
                                {
                                    "type": "Image",
                                    "url": FULL_STAR,
                                    "id": "star1",
                                    "size": "Small",
                                    "selectAction": {
                                        "type": "Action.Submit",
                                        "data": {
                                            "rating": 1
                                        }
                                    }
                                }
                            ]
                        },
                       {
                            "type": "Column",
                            "items": [
                                {
                                    "type": "Image",
                                    "url": FULL_STAR,
                                    "id": "star2",
                                    "size": "Small",
                                    "selectAction": {
                                        "type": "Action.Submit",
                                        "data": {
                                            "rating": 2
                                        }
                                    }
                                }
                            ]
                        },
                        {
                            "type": "Column",
                            "items": [
                                {
                                    "type": "Image",
                                    "url": FULL_STAR,
                                    "id": "star3",
                                    "size": "Small",
                                    "selectAction": {
                                        "type": "Action.Submit",
                                        "data": {
                                            "rating": 3
                                        }
                                    }
                                }
                            ]
                        },
                        {
                            "type": "Column",
                            "items": [
                                {
                                    "type": "Image",
                                    "url": FULL_STAR,
                                    "id": "star4",
                                    "size": "Small",
                                    "selectAction": {
                                        "type": "Action.Submit",
                                        "data": {
                                            "rating": 4
                                        }
                                    }
                                }
                            ]
                        },
                        {
                            "type": "Column",
                            "items": [
                                {
                                    "type": "Image",
                                    "url": FULL_STAR,
                                    "id": "star5",
                                    "size": "Small",
                                    "selectAction": {
                                        "type": "Action.Submit",
                                        "data": {
                                            "rating": 5
                                        }
                                    }
                                }
                            ]
                        }
                    ]
                }
            ]
        })
    ).await;

    let ActivityFrom { id: owner_id, name } =
        try_extract_from(activity).expect("Failed to fetch from");

    let card_id = response
        .as_ref()
        .and_then(|x| x.id.as_ref())
        .expect("response.id expected");

    let mut conn = pool
        .acquire()
        .await
        .expect("Failed to acquire database connection");

    queries::user::create_user(owner_id, name, &mut conn).await;
    queries::feedback::create_feedback(owner_id, card_id, &mut conn).await;
}

pub async fn handle_feedback_entry(
    client: &TeamsBotClient,
    pool: &SqlitePool,
    activity: &Activity,
    feedback: &models::adaptive_card_response::Feedback,
) {
    let card_id = activity
        .reply_to_id
        .as_ref()
        .expect("Reply to id should be set");

    let user_id = activity
        .from
        .as_ref()
        .and_then(|x| x.id.as_ref())
        .expect("from.id expected");

    let (base_url, mut response) = activity.create_response();

    let mut conn = pool
        .acquire()
        .await
        .expect("Failed to acquire database connection");

    let FeedbackMeta {
        conversation_id,
        owner_id,
        report_id,
    } = queries::feedback::get_feedback_by_id(card_id, &mut conn).await;

    let conversation_id = get_or_create_conversation(
        client,
        base_url,
        conversation_id,
        activity,
        &owner_id,
        &mut conn,
    )
    .await;

    queries::feedback::create_or_update_feedback_entry(
        card_id,
        user_id,
        feedback.rating,
        feedback.comment.as_deref(),
        &mut conn,
    )
    .await;

    let feedbacks = queries::feedback::get_feedbacks_by_id(card_id, &mut conn).await;

    let content = get_feedback_report_adaptive_card(feedbacks);

    response.recipient = None;
    response.r#type = Some(Type::Message);
    response.attachments = Some(vec![Attachment {
        content: Some(content),
        content_type: Some(ContentType::Adaptive),
        content_url: None,
        name: None,
        thumbnail_url: None,
    }]);

    match report_id {
        Some(report_id) => {
            client
                .update_activity(base_url, &conversation_id, &report_id, &response)
                .await;
        }
        None => {
            let response = client
                .send_to_conversation(base_url, &conversation_id, &response)
                .await;

            queries::feedback::add_report(
                card_id,
                &response.id.expect("Id sould be set"),
                &mut conn,
            )
            .await;
        }
    }
}

fn get_feedback_report_adaptive_card(
    feedbacks: Vec<queries::feedback::Feedback>,
) -> serde_json::Value {
    let mut comments: Vec<_> = feedbacks
        .iter()
        .filter_map(|x| {
            x.comment.as_ref().map(|comment| {
                AdaptiveCardElements::TextBlock(TextBlock {
                    r#type: "TextBlock".to_owned(),
                    text: comment.clone(),
                    wrap: true,
                    separator: true,
                })
            })
        })
        .collect();

    let mut average =
        feedbacks.iter().map(|x| x.rating).sum::<i64>() as f32 / feedbacks.len() as f32;

    let stars: Vec<_> = (0..5)
        .map(move |_| {
            let star = match average {
                x if x >= 1.0 => FULL_STAR,
                x if x >= 0.5 => HALF_STAR,
                _ => EMPTY_STAR,
            };

            average -= 1.0;

            Column {
                r#type: "Column".to_owned(),
                width: "stretch".to_owned(),
                items: vec![AdaptiveCardElements::Image(Image {
                    r#type: "Image".to_owned(),
                    url: star.to_owned(),
                    width: "32px".to_owned(),
                    horizontal_alignment: "Center".to_owned(),
                })],
            }
        })
        .collect();

    comments.push(AdaptiveCardElements::ColumnSet(ColumnSet {
        r#type: "ColumnSet".to_owned(),
        columns: stars,
        separator: true,
        spacing: "ExtraLarge".to_owned(),
    }));

    serde_json::json!({
        "type": "AdaptiveCard",
        "$schema": "http://adaptivecards.io/schemas/adaptive-card.json",
        "version": "1.5",
        "body": comments
    })
}

async fn get_or_create_conversation(
    client: &TeamsBotClient,
    base_url: Option<&str>,
    conversation_id: Option<String>,
    activity: &Activity,
    user_id: &str,
    conn: &mut PoolConnection<Sqlite>,
) -> String {
    match conversation_id {
        Some(conversation_id) => conversation_id,
        None => {
            let conversation_id = create_conversation(client, base_url, activity, user_id).await;
            queries::user::update_conversation(user_id, &conversation_id, conn).await;

            conversation_id
        }
    }
}

async fn create_conversation(
    client: &TeamsBotClient,
    base_url: Option<&str>,
    activity: &Activity,
    user_id: &str,
) -> String {
    let conversation_response = client
        .create_conversation(
            base_url,
            &ConversationParameters {
                bot: activity.recipient.clone(),
                is_group: Some(false),
                members: Some(vec![ChannelAccount {
                    id: Some(user_id.to_owned()),
                    ..Default::default()
                }]),
                tenant_id: activity.conversation.clone().and_then(|x| x.tenant_id),
                ..Default::default()
            },
        )
        .await;

    conversation_response.id.expect("Missing conversation id")
}

#[derive(Clone, Debug, Serialize)]
pub struct AdaptiveCard {
    #[serde(rename = "type")]
    pub r#type: String,
    pub schema: String,
    pub version: String,
    pub body: Vec<AdaptiveCardElements>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum AdaptiveCardElements {
    TextBlock(TextBlock),
    ColumnSet(ColumnSet),
    Column(Column),
    Image(Image),
}

#[derive(Clone, Debug, Serialize)]
pub struct TextBlock {
    #[serde(rename = "type")]
    pub r#type: String,
    pub text: String,
    pub wrap: bool,
    pub separator: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct ColumnSet {
    #[serde(rename = "type")]
    pub r#type: String,
    pub columns: Vec<Column>,
    pub separator: bool,
    pub spacing: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Column {
    #[serde(rename = "type")]
    pub r#type: String,
    pub width: String,
    pub items: Vec<AdaptiveCardElements>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Image {
    #[serde(rename = "type")]
    pub r#type: String,
    pub url: String,
    pub width: String,
    #[serde(rename = "horizontalAlignment")]
    pub horizontal_alignment: String,
}
