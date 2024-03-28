use serde::Serialize;

use crate::models::*;

/// Defines parameters for creating a new conversation.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationParameters {
    /// Channel account information needed to route a message to the bot.
    pub bot: ChannelAccount,
    /// Channel account information needed to route a message to each user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<ChannelAccount>>,
    /// The tenant ID in which the conversation should be created.
    pub tenant_id: String,
}
