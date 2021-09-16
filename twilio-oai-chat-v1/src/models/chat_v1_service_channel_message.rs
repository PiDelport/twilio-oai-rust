/*
 * Twilio - Chat
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChatV1ServiceChannelMessage {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The JSON string that stores application-specific data
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    /// The content of the message
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// The unique ID of the Channel the Message resource belongs to
    #[serde(rename = "channel_sid", skip_serializing_if = "Option::is_none")]
    pub channel_sid: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The identity of the message's author
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// The index of the message within the Channel
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// The SID of the Service that the resource is associated with
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The SID of the Channel that the message was sent to
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// The absolute URL of the Message resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Whether the message has been edited since  it was created
    #[serde(rename = "was_edited", skip_serializing_if = "Option::is_none")]
    pub was_edited: Option<bool>,
}

impl ChatV1ServiceChannelMessage {
    pub fn new() -> ChatV1ServiceChannelMessage {
        ChatV1ServiceChannelMessage {
            account_sid: None,
            attributes: None,
            body: None,
            channel_sid: None,
            date_created: None,
            date_updated: None,
            from: None,
            index: None,
            service_sid: None,
            sid: None,
            to: None,
            url: None,
            was_edited: None,
        }
    }
}


