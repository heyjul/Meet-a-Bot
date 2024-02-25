use axum::{extract::State, Json};
use teams_api::{
    client::TeamsBotClient,
    models::{activity::Type, Activity},
};

use crate::{
    commands::{
        feedback::{handle_feedback_entry, send_feedback_card},
        send_message, Commands,
    },
    state::AppState,
    utils::parse_command,
};
//, fields(activity = %activity)
#[tracing::instrument(skip(client, pool, activity))]
pub async fn handle(
    State(AppState { client, pool }): State<AppState>,
    Json(activity): Json<Activity>,
) {
    match activity.r#type {
        Some(Type::ConversationUpdate) => send_greetings(&client, &activity).await,
        Some(Type::Message) => {
            if activity.text.is_some() {
                match parse_command(&activity) {
                    Some(Commands::Feedback) => send_feedback_card(&client, &pool, &activity).await,
                    None => send_message(&client, &activity, "Failed to parse the command.").await,
                }
            }
            if let Some(ref value) = activity.value {
                if let Ok(crate::models::adaptive_card_response::AdaptiveCardResponse::Feedback(
                    feedback,
                )) = serde_json::from_value::<
                    crate::models::adaptive_card_response::AdaptiveCardResponse,
                >(value.clone())
                {
                    handle_feedback_entry(&client, &pool, &activity, &feedback).await;
                }
            }
        }
        _ => (),
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
