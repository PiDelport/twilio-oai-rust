/*
 * Twilio - Conversations
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConversationsV1ServiceServiceConversationServiceConversationMessage {
    /// The unique ID of the Account responsible for this message.
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// A string metadata field you can use to store any data you wish.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    /// The channel specific identifier of the message's author.
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// The content of the message.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// The SID of the Conversation Service that the resource is associated with.
    #[serde(rename = "chat_service_sid", skip_serializing_if = "Option::is_none")]
    pub chat_service_sid: Option<String>,
    /// The unique ID of the Conversation for this message.
    #[serde(rename = "conversation_sid", skip_serializing_if = "Option::is_none")]
    pub conversation_sid: Option<String>,
    /// The date that this resource was created.
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The date that this resource was last updated.
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// An object that contains the summary of delivery statuses for the message to non-chat participants.
    #[serde(rename = "delivery", skip_serializing_if = "Option::is_none")]
    pub delivery: Option<serde_json::Value>,
    /// The index of the message within the Conversation.
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// Absolute URL to access the receipts of this message.
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// An array of objects that describe the Message's media if attached, otherwise, null.
    #[serde(rename = "media", skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<serde_json::Value>>,
    /// The unique ID of messages's author participant.
    #[serde(rename = "participant_sid", skip_serializing_if = "Option::is_none")]
    pub participant_sid: Option<String>,
    /// A 34 character string that uniquely identifies this resource.
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// An absolute URL for this message.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ConversationsV1ServiceServiceConversationServiceConversationMessage {
    pub fn new() -> ConversationsV1ServiceServiceConversationServiceConversationMessage {
        ConversationsV1ServiceServiceConversationServiceConversationMessage {
            account_sid: None,
            attributes: None,
            author: None,
            body: None,
            chat_service_sid: None,
            conversation_sid: None,
            date_created: None,
            date_updated: None,
            delivery: None,
            index: None,
            links: None,
            media: None,
            participant_sid: None,
            sid: None,
            url: None,
        }
    }
}


