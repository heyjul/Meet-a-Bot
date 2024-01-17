use serde::{Deserialize, Serialize};

/// Defines a response that contains a resource ID.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceResponse {
    /// ID that uniquely identifies the resource.
    pub id: String,
}
