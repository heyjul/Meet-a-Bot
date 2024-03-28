use serde::Deserialize;

/// Defines a response to Create Conversation.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationResourceResponse {
    /// ID of the resource.
    pub id: String,
}
