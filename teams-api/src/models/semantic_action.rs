use serde::{Deserialize, Serialize};

/// Defines a reference to a programmatic action.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SemanticAction {
    /// An object where the value of each property is an Entity object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<serde_json::Value>,
    /// ID of this action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// State of this action. Allowed values: start, continue, done.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "done")]
    Done,
}
