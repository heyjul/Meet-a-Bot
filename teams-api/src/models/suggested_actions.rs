use serde::{Deserialize, Serialize};

use super::*;

/// Defines the options from which a user can choose.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedActions {
    /// Array of CardAction objects that define the suggested actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<CardAction>>,
    /// Array of strings that contains the IDs of the recipients to whom the suggested actions should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
}
