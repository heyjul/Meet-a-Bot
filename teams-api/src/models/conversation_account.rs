use serde::{Deserialize, Serialize};

/// Defines a conversation in a channel
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationAccount {
    /// This account's object ID within Microsoft Entra ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aad_object_id: Option<String>,
    /// Indicates the type of the conversation in channels that distinguish between conversation types (for example, group or personal).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_type: Option<String>,
    /// The ID that identifies the conversation. The ID is unique per channel. If the channel starts the conversation, it sets this ID; otherwise, the bot sets this property to the ID that it gets back in the response when it starts the conversation (see Create Conversation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Flag to indicate whether the conversation contains more than two participants at the time the activity was generated. Set to true if this is a group conversation; otherwise, false. The default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_group: Option<bool>,
    /// A display name that can be used to identify the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Role of the entity behind the account. Either user or bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// This conversation's tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
