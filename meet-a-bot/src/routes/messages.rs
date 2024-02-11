pub use axum::{extract::State, Json};
use teams_api::{
    client::TeamsBotClient,
    models::{
        activity::{TextFormat, Type},
        attachment::ContentType,
        Activity, Attachment,
    },
};

use crate::{state::AppState, utils::parse_command};

#[tracing::instrument(skip(client, activity), fields(activity = %activity))]
pub async fn handle(State(AppState { client }): State<AppState>, Json(activity): Json<Activity>) {
    match activity.r#type {
        Some(Type::ConversationUpdate) => send_greetings(&client, &activity).await,
        Some(Type::Message) => {
            if let Some(TextFormat::Plain) = activity.text_format {
                match parse_command(&activity) {
                    Some("end") => send_adaptive_card(
                        &client,
                        &activity,
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
                                    "type": "ActionSet",
                                    "actions": [
                                        {
                                            "type": "Action.Submit",
                                            "iconUrl": "https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fpluspng.com%2Fimg-png%2Fstar-png-star-vector-png-transparent-image-2000.png&f=1&nofb=1&ipt=24e63d9cad1555be8ff2f18a33a1c1cefbe8d77a137ded45ad3125344c92aef0&ipo=images",
                                            "id": "star1",
                                            "data": {
                                                "rating": 1
                                            }
                                        },
                                        {
                                            "type": "Action.Submit",
                                            "iconUrl": "https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fpluspng.com%2Fimg-png%2Fstar-png-star-vector-png-transparent-image-2000.png&f=1&nofb=1&ipt=24e63d9cad1555be8ff2f18a33a1c1cefbe8d77a137ded45ad3125344c92aef0&ipo=images",
                                            "id": "star2",
                                            "data": {
                                                "rating": 2
                                            }
                                        },
                                        {
                                            "type": "Action.Submit",
                                            "iconUrl": "https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fpluspng.com%2Fimg-png%2Fstar-png-star-vector-png-transparent-image-2000.png&f=1&nofb=1&ipt=24e63d9cad1555be8ff2f18a33a1c1cefbe8d77a137ded45ad3125344c92aef0&ipo=images",
                                            "id": "star3",
                                            "data": {
                                                "rating": 3
                                            }
                                        },
                                        {
                                            "type": "Action.Submit",
                                            "iconUrl": "https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fpluspng.com%2Fimg-png%2Fstar-png-star-vector-png-transparent-image-2000.png&f=1&nofb=1&ipt=24e63d9cad1555be8ff2f18a33a1c1cefbe8d77a137ded45ad3125344c92aef0&ipo=images",
                                            "id": "star4",
                                            "data": {
                                                "rating": 4
                                            }
                                        },
                                        {
                                            "type": "Action.Submit",
                                            "iconUrl": "https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fpluspng.com%2Fimg-png%2Fstar-png-star-vector-png-transparent-image-2000.png&f=1&nofb=1&ipt=24e63d9cad1555be8ff2f18a33a1c1cefbe8d77a137ded45ad3125344c92aef0&ipo=images",
                                            "id": "star5",
                                            "data": {
                                                "rating": 5
                                            }
                                        }
                                    ]
                                }
                            ]
                        }),
                    ).await,
                    Some(x) => send_message(
                        &client,
                        &activity,
                        &format!("Command `{x}` does not exist.")).await,
                    None => send_message(
                        &client,
                        &activity,
                        "Failed to parse the command.").await
                }
            } else {
                send_message(
                    &client,
                    &activity,
                    "Other text formats than `Plain` are not handled for now",
                )
                .await;
            }
        }
        _ => (),
    }
    if let Some(Type::ConversationUpdate) = activity.r#type {
        send_greetings(&client, &activity).await;
    }
}

#[tracing::instrument(skip_all)]
pub async fn send_greetings(client: &TeamsBotClient, activity: &Activity) {
    let members_added = &activity.members_added;
    let recipient = &activity.recipient;

    match (members_added, recipient) {
        (Some(ref members_added), Some(ref recipient)) => {
            if let Some(ref id) = recipient.id {
                if !members_added
                    .iter()
                    .any(|x| x.id.as_deref().map_or(false, |x| x == id))
                {
                    return;
                }
            } else {
                return;
            }
        }
        _ => return,
    }

    let message = format!("Salut ! Je suis {name}, prêt à rendre le meeting plus dynamique ! Pour en savoir plus, n'hésitez pas à me demander de l'aide ! (@{name} help)", name = activity.recipient.as_ref().and_then(|x| x.name.as_deref()).unwrap_or("{bot_name}"));
    send_message(client, activity, &message).await;
}

#[tracing::instrument(skip_all)]
pub async fn send_message(client: &TeamsBotClient, activity: &Activity, message: &str) {
    let (base_url, mut response) = activity.create_response();
    response.r#type = Some(Type::Message);
    response.text = Some(message.to_owned());

    if let Some(ref conversation) = activity.conversation {
        if let Some(ref conversation_id) = conversation.id {
            client
                .send_to_conversation(base_url, conversation_id, &response)
                .await;
        }
    }
}

#[tracing::instrument(skip_all)]
pub async fn send_adaptive_card(
    client: &TeamsBotClient,
    activity: &Activity,
    adaptive_card: &serde_json::Value,
) {
    let (base_url, mut response) = activity.create_response();
    response.r#type = Some(Type::Message);
    response.attachments = Some(vec![Attachment {
        content: Some(adaptive_card.to_owned()),
        content_type: Some(ContentType::Adaptive),
        content_url: None,
        name: None,
        thumbnail_url: None,
    }]);

    if let Some(ref conversation) = activity.conversation {
        if let Some(ref conversation_id) = conversation.id {
            client
                .send_to_conversation(base_url, conversation_id, &response)
                .await;
        }
    }
}
