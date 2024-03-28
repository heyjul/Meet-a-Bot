use serde::{Deserialize, Serialize};

/// Defines additional information to include in the message. An attachment may be a file (such as an image, audio, or video) or a rich card.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    /// The content of the attachment. If the attachment is a rich card, set this property to the rich card object. This property and the contentUrl property are mutually exclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
    /// The media type of the content in the attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ContentType>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    /// A rich card that can contain any combination of text, speech, images, buttons, and input fields. Set the content property to an AdaptiveCard object.
    #[serde(rename = "application/vnd.microsoft.card.adaptive")]
    Adaptive,
    /// A rich card that plays animation. Set the content property to an AnimationCard object.
    #[serde(rename = "application/vnd.microsoft.card.animation")]
    Animation,
    /// A rich card that plays audio files. Set the content property to an AudioCard object.
    #[serde(rename = "application/vnd.microsoft.card.audio")]
    Audio,
    /// A Hero card. Set the content property to a HeroCard object.
    #[serde(rename = "application/vnd.microsoft.card.hero")]
    Hero,
    /// A Receipt card. Set the content property to a ReceiptCard object.
    #[serde(rename = "application/vnd.microsoft.card.receipt")]
    Receipt,
    /// A user Sign In card. Set the content property to a SignInCard object.
    #[serde(rename = "application/vnd.microsoft.card.signin")]
    Signin,
    /// A Thumbnail card. Set the content property to a ThumbnailCard object.
    #[serde(rename = "application/vnd.microsoft.card.thumbnail")]
    Thumbnail,
    /// A rich card that plays videos. Set the content property to a VideoCard object.
    #[serde(rename = "application/vnd.microsoft.card.video")]
    Video,
    /// A media files. Set this property to known media types such as image/png, audio/wav, and video/mp4
    #[serde(untagged)]
    Media(String),
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::ContentType;

    #[rstest]
    #[case(ContentType::Adaptive, "\"application/vnd.microsoft.card.adaptive\"")]
    #[case(ContentType::Animation, "\"application/vnd.microsoft.card.animation\"")]
    #[case(ContentType::Audio, "\"application/vnd.microsoft.card.audio\"")]
    #[case(ContentType::Hero, "\"application/vnd.microsoft.card.hero\"")]
    #[case(ContentType::Receipt, "\"application/vnd.microsoft.card.receipt\"")]
    #[case(ContentType::Signin, "\"application/vnd.microsoft.card.signin\"")]
    #[case(ContentType::Thumbnail, "\"application/vnd.microsoft.card.thumbnail\"")]
    #[case(ContentType::Video, "\"application/vnd.microsoft.card.video\"")]
    #[case(ContentType::Media(String::from("image/png")), "\"image/png\"")]
    fn test_content_type_serialize(#[case] content_type: ContentType, #[case] expected: &str) {
        // Act
        let result = serde_json::to_string(&content_type).unwrap();

        // Assert
        assert_eq!(expected, result.to_owned());
    }

    #[rstest]
    #[case("\"application/vnd.microsoft.card.adaptive\"", ContentType::Adaptive)]
    #[case("\"application/vnd.microsoft.card.animation\"", ContentType::Animation)]
    #[case("\"application/vnd.microsoft.card.audio\"", ContentType::Audio)]
    #[case("\"application/vnd.microsoft.card.hero\"", ContentType::Hero)]
    #[case("\"application/vnd.microsoft.card.receipt\"", ContentType::Receipt)]
    #[case("\"application/vnd.microsoft.card.signin\"", ContentType::Signin)]
    #[case("\"application/vnd.microsoft.card.thumbnail\"", ContentType::Thumbnail)]
    #[case("\"application/vnd.microsoft.card.video\"", ContentType::Video)]
    #[case("\"image/png\"", ContentType::Media(String::from("image/png")))]
    fn test_content_type_deserialize(#[case] content_type: &str, #[case] expected: ContentType) {
        // Act
        let result = serde_json::from_str(content_type).unwrap();

        // Assert
        assert_eq!(expected, result);
    }
}
