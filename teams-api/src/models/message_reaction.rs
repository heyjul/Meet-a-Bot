use serde::{Deserialize, Serialize};

/// Defines a reaction to a message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageReaction {
    /// Type of reaction. Either like or plusOne.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "like")]
    Like,
    #[serde(rename = "plusOne")]
    PlusOne,
}
