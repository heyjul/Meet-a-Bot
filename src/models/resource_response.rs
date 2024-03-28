use serde::Deserialize;

/// Defines a response that contains a resource ID.
#[derive(Clone, Debug, Deserialize)]
pub struct ResourceResponse {
    /// ID that uniquely identifies the resource.
    pub id: String,
}
