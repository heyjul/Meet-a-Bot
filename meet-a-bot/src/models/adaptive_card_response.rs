use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdaptiveCardResponse {
    Feedback(Feedback),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Feedback {
    pub comment: Option<String>,
    pub rating: i64,
}
