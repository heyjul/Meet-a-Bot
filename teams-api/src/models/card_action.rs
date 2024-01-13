use serde::{Deserialize, Serialize};

/// Defines a clickable action with a button.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardAction {
    /// Channel-specific data associated with this action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_data: Option<String>,
    /// Text to display in the chat feed if the button is clicked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
    /// Image URL that will appear on the button, next to the text label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Text for the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Text description that appears on the button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Type of action to perform. For a list of valid values, see Add rich card attachments to messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Supplementary parameter for the action. The behavior of this property will vary according to the action type. For more information, see Add rich card attachments to messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
