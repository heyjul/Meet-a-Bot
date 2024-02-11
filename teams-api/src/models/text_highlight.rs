use serde::{Deserialize, Serialize};

/// Refers to a substring of content within another field.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TextHighlight {
    /// Occurrence of the text field within the referenced text, if multiple exist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrence: Option<usize>,
    /// Defines the snippet of text to highlight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
