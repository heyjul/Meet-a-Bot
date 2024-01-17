use serde::{Deserialize, Serialize};

use crate::models::*;

/// Defines parameters for creating a new conversation.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationParameters {
    /// The initial message to send to the conversation when it's created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<Activity>,
    /// Channel account information needed to route a message to the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<ChannelAccount>,
    /// Channel-specific payload for creating the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_data: Option<serde_json::Value>,
    /// Indicates whether this is a group conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_group: Option<bool>,
    /// Channel account information needed to route a message to each user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<ChannelAccount>>,
    /// The tenant ID in which the conversation should be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// Topic of the conversation. This property is only used if a channel supports it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}
