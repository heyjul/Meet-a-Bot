use serde::{Deserialize, Serialize};

/// Defines a response to Create Conversation.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationResourceResponse {
    ///	ID of the activity, if sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    ///	ID of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///	Service endpoint where operations concerning the conversation may be performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_url: Option<String>,
}
