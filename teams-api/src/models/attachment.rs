use serde::{Deserialize, Serialize};

/// Defines additional information to include in the message. An attachment may be a file (such as an image, audio, or video) or a rich card.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    ///	The content of the attachment. If the attachment is a rich card, set this property to the rich card object. This property and the contentUrl property are mutually exclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
    ///	The media type of the content in the attachment. For media files, set this property to known media types such as image/png, audio/wav, and video/mp4. For rich cards, set this property to one of these vendor-specific types:
    /// - application/vnd.microsoft.card.adaptive: A rich card that can contain any combination of text, speech, images, buttons, and input fields. Set the content property to an AdaptiveCard object.
    /// - application/vnd.microsoft.card.animation: A rich card that plays animation. Set the content property to an AnimationCard object.
    /// - application/vnd.microsoft.card.audio: A rich card that plays audio files. Set the content property to an AudioCard object.
    /// - application/vnd.microsoft.card.hero: A Hero card. Set the content property to a HeroCard object.
    /// - application/vnd.microsoft.card.receipt: A Receipt card. Set the content property to a ReceiptCard object.
    /// - application/vnd.microsoft.card.signin: A user Sign In card. Set the content property to a SignInCard object.
    /// - application/vnd.microsoft.card.thumbnail: A Thumbnail card. Set the content property to a ThumbnailCard object.
    /// - application/vnd.microsoft.card.video: A rich card that plays videos. Set the content property to a VideoCard object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    ///	URL for the content of the attachment. For example, if the attachment is an image, you can set contentUrl to the URL that represents the location of the image. Supported protocols are: HTTP, HTTPS, File, and Data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
    ///	Name of the attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///	URL to a thumbnail image that the channel can use if it supports using an alternative, smaller form of content or contentUrl. For example, if you set contentType to application/word and set contentUrl to the location of the Word document, you might include a thumbnail image that represents the document. The channel could display the thumbnail image instead of the document. When the user clicks the image, the channel would open the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
}
