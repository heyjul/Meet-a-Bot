use serde::{Deserialize, Serialize};

use super::*;

/// Defines a particular point in a conversation.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationReference {
    /// ID that uniquely identifies the activity that this object references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    /// A ChannelAccount object that identifies the bot in the conversation that this object references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<ChannelAccount>,
    /// An ID that uniquely identifies the channel in the conversation that this object references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// A ConversationAccount object that defines the conversation that this object references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ConversationAccount>,
    /// URL that specifies the channel's service endpoint in the conversation that this object references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_url: Option<String>,
    /// A ChannelAccount object that identifies the user in the conversation that this object references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<ChannelAccount>,
}
