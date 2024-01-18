use serde::Deserialize;

/// Defines a response to Create Conversation.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationResourceResponse {
    /// ID of the activity, if sent.
    pub activity_id: Option<String>,
    /// ID of the resource.
    pub id: Option<String>,
    /// Service endpoint where operations concerning the conversation may be performed.
    pub service_url: Option<String>,
}
