use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::*;

/// Defines a message that is exchanged between bot and user.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    /// The action to apply or that was applied. Use the type property to determine context for the action. For example, if type is contactRelationUpdate, the value of the action property would be add if the user added your bot to their contacts list, or remove if they removed your bot from their contacts list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Layout of the rich card attachments that the message includes. One of these values: carousel, list. For more information about rich card attachments, see Add rich card attachments to messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_layout: Option<AttachmentLayout>,
    /// Array of Attachment objects that defines additional information to include in the message. Each attachment may be either a file (for example, audio, video, image) or a rich card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    /// A string containing an IRI identifying the caller of a bot. This field isn't intended to be transmitted over the wire, but is instead populated by bots and clients based on cryptographically verifiable data that asserts the identity of the callers (for example, tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_id: Option<String>,
    /// An object that contains channel-specific content. Some channels provide features that require additional information that can't be represented using the attachment schema. For those cases, set this property to the channel-specific content as defined in the channel's documentation. For more information, see Implement channel-specific functionality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_data: Option<serde_json::Value>,
    /// An ID that uniquely identifies the channel. Set by the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// Code indicating why the conversation has ended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// A ConversationAccount object that defines the conversation to which the activity belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ConversationAccount>,
    /// A delivery hint to signal to the recipient alternate delivery paths for the activity. One of these values: normal, notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<DeliveryMode>,
    /// Array of objects that represents the entities that were mentioned in the message. Objects in this array may be any Schema.org object. For example, the array may include Mention objects that identify someone who was mentioned in the conversation and Place objects that identify a place that was mentioned in the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<serde_json::Value>>,
    /// The time at which the activity should be considered to be "expired" and shouldn't be presented to the recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// A ChannelAccount object that specifies the sender of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<ChannelAccount>,
    /// Flag that indicates whether or not history is disclosed. Default value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_disclosed: Option<bool>,
    /// ID that uniquely identifies the activity on the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Defines the importance of an Activity. One of these values: low, normal, high.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importance: Option<Importance>,
    /// Value that indicates whether your bot is accepting, expecting, or ignoring user input after the message is delivered to the client. One of these values: acceptingInput, expectingInput, ignoringInput.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_hint: Option<InputHint>,
    /// A descriptive label for the activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// List of phrases and references that speech and language priming systems should listen for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_for: Option<Vec<String>>,
    /// Locale of the language that should be used to display text within the message, in the format <language>-<country>. The channel uses this property to indicate the user's language, so that your bot may specify display strings in that language. Default value is en-US.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Date and time that the message was sent in the local time zone, expressed in ISO-8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_timestamp: Option<String>,
    /// Contains the name of the local timezone of the message, expressed in IANA Time Zone database format. For example, America/Los_Angeles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_timezone: Option<String>,
    /// Array of ChannelAccount objects that represents the list of users that joined the conversation. Present only if activity type is "conversationUpdate" and users joined the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_added: Option<Vec<ChannelAccount>>,
    /// Array of ChannelAccount objects that represents the list of users that left the conversation. Present only if activity type is "conversationUpdate" and users left the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_removed: Option<Vec<ChannelAccount>>,
    /// Name of the operation to invoke or the name of the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The collection of reactions added to the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions_added: Option<Vec<MessageReaction>>,
    /// The collection of reactions removed from the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions_removed: Option<Vec<MessageReaction>>,
    /// A ChannelAccount object that specifies the recipient of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<ChannelAccount>,
    /// A ConversationReference object that defines a particular point in a conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relates_to: Option<ConversationReference>,
    /// The ID of the message to which this message replies. To reply to a message that the user sent, set this property to the ID of the user's message. Not all channels support threaded replies. In these cases, the channel will ignore this property and use time ordered semantics (timestamp) to append the message to the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_id: Option<String>,
    /// A SemanticAction object that represents a reference to a programmatic action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_action: Option<SemanticAction>,
    /// URL that specifies the channel's service endpoint. Set by the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_url: Option<String>,
    /// Text to be spoken by your bot on a speech-enabled channel. To control various characteristics of your bot's speech such as voice, rate, volume, pronunciation, and pitch, specify this property in Speech Synthesis Markup Language (SSML) format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speak: Option<String>,
    /// A SuggestedActions object that defines the options from which the user can choose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_actions: Option<SuggestedActions>,
    /// Summary of the information that the message contains. For example, for a message that is sent on an email channel, this property may specify the first 50 characters of the email message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Text of the message that is sent from user to bot or bot to user. See the channel's documentation for limits imposed upon the contents of this property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Format of the message's text. One of these values: markdown, plain, xml. For details about text format, see Create messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_format: Option<TextFormat>,
    /// The collection of text fragments to highlight when the activity contains a replyToId value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_highlights: Option<Vec<TextHighlight>>,
    /// Date and time that the message was sent in the UTC time zone, expressed in ISO-8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Topic of the conversation to which the activity belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// Type of activity. One of these values: message, contactRelationUpdate, conversationUpdate, typing, endOfConversation, event, invoke, deleteUserData, messageUpdate, messageDelete, installationUpdate, messageReaction, suggestion, trace, handoff. For details about activity types, see Activities overview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Open-ended value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    /// The type of the activity's value object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AttachmentLayout {
    #[serde(rename = "carousel")]
    Carousel,
    #[serde(rename = "list")]
    List,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DeliveryMode {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "notification")]
    Notification,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Importance {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "high")]
    High,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InputHint {
    #[serde(rename = "acceptingInput")]
    AcceptingInput,
    #[serde(rename = "expectingInput")]
    ExpectingInput,
    #[serde(rename = "ignoringInput")]
    IgnoringInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TextFormat {
    #[serde(rename = "markdown")]
    Markdown,
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "xml")]
    Xml,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "contactRelationUpdate")]
    ContactRelationUpdate,
    #[serde(rename = "conversationUpdate")]
    ConversationUpdate,
    #[serde(rename = "typing")]
    Typing,
    #[serde(rename = "endOfConversation")]
    EndOfConversation,
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "invoke")]
    Invoke,
    #[serde(rename = "deleteUserData")]
    DeleteUserData,
    #[serde(rename = "messageUpdate")]
    MessageUpdate,
    #[serde(rename = "messageDelete")]
    MessageDelete,
    #[serde(rename = "installationUpdate")]
    InstallationUpdate,
    #[serde(rename = "messageReaction")]
    MessageReaction,
    #[serde(rename = "suggestion")]
    Suggestion,
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "handoff")]
    Handoff,
}

impl Activity {
    pub fn create_response(&self) -> (Option<&str>, Self) {
        (
            self.service_url.as_deref(),
            Activity {
                from: self.recipient.clone(),
                conversation: self.conversation.clone(),
                recipient: self.from.clone(),
                ..Default::default()
            },
        )
    }
}

impl Display for Activity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}
