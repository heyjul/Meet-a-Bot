use serde::{Deserialize, Serialize};

use super::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub r#type: Type,
    pub from: ChannelAccount,
    pub recipient: ChannelAccount,
    pub conversation: ConversationAccount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_added: Option<Vec<ChannelAccount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum Type {
    #[default]
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "contactRelationUpdate")]
    ContactRelationUpdate,
    #[serde(rename = "conversationUpdate")]
    ConversationUpdate,
    #[serde(rename = "typing")]
    Typing,
    #[serde(rename = "endOfConversation")]
    EndOfConversation,
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "invoke")]
    Invoke,
    #[serde(rename = "deleteUserData")]
    DeleteUserData,
    #[serde(rename = "messageUpdate")]
    MessageUpdate,
    #[serde(rename = "messageDelete")]
    MessageDelete,
    #[serde(rename = "installationUpdate")]
    InstallationUpdate,
    #[serde(rename = "messageReaction")]
    MessageReaction,
    #[serde(rename = "suggestion")]
    Suggestion,
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "handoff")]
    Handoff,
}

impl Activity {
    pub fn create_response(&self) -> (Option<&str>, Self) {
        (
            self.service_url.as_deref(),
            Activity {
                from: self.recipient.clone(),
                recipient: self.from.clone(),
                ..Default::default()
            },
        )
    }
}
