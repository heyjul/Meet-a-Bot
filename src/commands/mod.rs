pub mod feedback_command;

use crate::{
    error::{Error, Result},
    models::{
        activity::{Activity, Type},
        attachment::ContentType,
        Attachment, ResourceResponse,
    },
    services::teams_client::TeamsClient,
};

#[derive(Debug, PartialEq)]
pub enum Commands {
    Feedback,
    Help,
}

impl TryFrom<&str> for Commands {
    type Error = Error;

    fn try_from(value: &str) -> Result<Commands> {
        match value.trim() {
            "feedback" => Ok(Self::Feedback),
            "help" => Ok(Self::Help),
            _ => Err(Error::UnknownCommand(value.to_owned())),
        }
    }
}

#[tracing::instrument(skip_all)]
pub async fn send_message(client: &TeamsClient, activity: &Activity, message: &str) -> Result<()> {
    let (base_url, mut response) = activity.create_response();
    response.r#type = Type::Message;
    response.text = Some(message.to_owned());

    client
        .send_to_conversation(base_url, &activity.conversation.id, &response)
        .await?;

    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn send_adaptive_card(
    client: &TeamsClient,
    activity: &Activity,
    adaptive_card: &serde_json::Value,
) -> Result<ResourceResponse> {
    let (base_url, mut response) = activity.create_response();
    response.r#type = Type::Message;
    response.attachments = Some(vec![Attachment {
        content: Some(adaptive_card.to_owned()),
        content_type: Some(ContentType::Adaptive),
    }]);

    let result = client
        .send_to_conversation(base_url, &activity.conversation.id, &response)
        .await?;

    Ok(result)
}
