use serde::{Deserialize, Serialize};

/// Defines a bot or user account on the channel.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelAccount {
    /// This account's object ID within Microsoft Entra ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aad_object_id: Option<String>,
    /// Unique ID for the user or bot on this channel.
    pub id: String,
    /// Display-friendly name of the bot or user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Role of the entity behind the account. Either user or bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
