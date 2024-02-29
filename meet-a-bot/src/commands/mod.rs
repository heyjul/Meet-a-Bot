pub mod feedback;

use crate::error::{Error, Result};

use teams_api::{
    client::TeamsBotClient,
    models::{
        activity::Type, attachment::ContentType, responses::ResourceResponse, Activity, Attachment,
    },
};

#[derive(Debug, PartialEq)]
pub enum Commands {
    Feedback,
}

impl TryFrom<&str> for Commands {
    type Error = Error;

    fn try_from(value: &str) -> Result<Commands> {
        match value.trim() {
            "feedback" => Ok(Self::Feedback),
            _ => Err(Error::UnknownCommand(value.to_owned())),
        }
    }
}

impl ToString for Commands {
    fn to_string(&self) -> String {
        match self {
            Self::Feedback => "feedback".to_owned(),
        }
    }
}

#[tracing::instrument(skip_all)]
pub async fn send_message(
    client: &TeamsBotClient,
    activity: &Activity,
    message: &str,
) -> Result<()> {
    let (base_url, mut response) = activity.create_response();
    response.r#type = Some(Type::Message);
    response.text = Some(message.to_owned());

    if let Some(ref conversation) = activity.conversation {
        if let Some(ref conversation_id) = conversation.id {
            client
                .send_to_conversation(base_url, conversation_id, &response)
                .await?;
        }
    }

    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn send_adaptive_card(
    client: &TeamsBotClient,
    activity: &Activity,
    adaptive_card: &serde_json::Value,
) -> Result<Option<ResourceResponse>> {
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
            let result = client
                .send_to_conversation(base_url, conversation_id, &response)
                .await?;

            return Ok(Some(result));
        }
    }

    Ok(None)
}
