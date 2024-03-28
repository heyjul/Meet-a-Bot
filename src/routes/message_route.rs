use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    commands::{
        feedback_command::{handle_feedback_entry, send_feedback_card},
        send_message, Commands,
    },
    models::activity::{Activity, Type},
    services::teams_client::TeamsClient,
    state::AppState,
    utils::parse_command,
};

use crate::error::Result;

#[tracing::instrument(skip_all)]
pub async fn handle(
    State(AppState {
        teams_client,
        graph_client,
        pool,
    }): State<AppState>,
    Json(activity): Json<Activity>,
) -> Result<impl IntoResponse> {
    match activity.r#type {
        Type::ConversationUpdate => send_greetings(&teams_client, &activity).await?,
        Type::Message => {
            if activity.text.is_some() {
                match parse_command(&activity) {
                    Some(Commands::Feedback) => {
                        send_feedback_card(&teams_client, &graph_client, &pool, &activity).await?
                    }
                    Some(Commands::Help) => {
                        send_message(
                            &teams_client,
                            &activity,
                            "Franchement ? De l'aide ? Gros y a que 2 commandes ...",
                        )
                        .await?
                    }
                    None => {
                        send_message(&teams_client, &activity, "Failed to parse the command.")
                            .await?
                    }
                }
            }
            if let Some(ref value) = activity.value {
                if let Ok(crate::models::action::Action::Feedback(feedback)) =
                    serde_json::from_value::<crate::models::action::Action>(value.clone())
                {
                    handle_feedback_entry(&teams_client, &pool, &activity, &feedback).await?;
                }
            }
        }
        _ => (),
    }

    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn send_greetings(client: &TeamsClient, activity: &Activity) -> Result<()> {
    let members_added = &activity.members_added;
    let recipient = &activity.recipient;

    match (members_added, recipient) {
        (Some(ref members_added), ref recipient) => {
            let id = &recipient.id;
            if !members_added.iter().any(|x| x.id == *id) {
                return Ok(());
            }
        }
        _ => return Ok(()),
    }

    let message = format!("Salut ! Je suis {name}, prêt à rendre le meeting plus dynamique ! Pour en savoir plus, n'hésitez pas à me demander de l'aide ! (@{name} help)", name = activity.recipient.name.as_deref().unwrap_or("{bot_name}"));
    send_message(client, activity, &message).await?;

    Ok(())
}
