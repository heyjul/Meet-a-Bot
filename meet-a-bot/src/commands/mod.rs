use teams_api::{
    client::TeamsBotClient,
    models::{
        activity::Type, attachment::ContentType, responses::ResourceResponse, Activity, Attachment,
    },
};

pub mod feedback;

#[derive(Debug, PartialEq)]
pub enum Commands {
    Feedback,
}

impl TryFrom<&str> for Commands {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "feedback" => Ok(Self::Feedback),
            _ => Err(()),
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
) -> Option<ResourceResponse> {
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
            return Some(
                client
                    .send_to_conversation(base_url, conversation_id, &response)
                    .await,
            );
        }
    }

    None
}
