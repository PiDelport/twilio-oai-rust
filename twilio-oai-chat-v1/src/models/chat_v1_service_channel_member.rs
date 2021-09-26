/*
 * Twilio - Chat
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChatV1ServiceChannelMember {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The unique ID of the Channel for the member
    #[serde(rename = "channel_sid", skip_serializing_if = "Option::is_none")]
    pub channel_sid: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The string that identifies the resource's User
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// The index of the last Message that the Member has read within the Channel
    #[serde(
        rename = "last_consumed_message_index",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_consumed_message_index: Option<i32>,
    /// The ISO 8601 based timestamp string that represents the date-time of the last Message read event for the Member within the Channel
    #[serde(
        rename = "last_consumption_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_consumption_timestamp: Option<String>,
    /// The SID of the Role assigned to the member
    #[serde(rename = "role_sid", skip_serializing_if = "Option::is_none")]
    pub role_sid: Option<String>,
    /// The SID of the Service that the resource is associated with
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The absolute URL of the Member resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ChatV1ServiceChannelMember {
    pub fn new() -> ChatV1ServiceChannelMember {
        ChatV1ServiceChannelMember {
            account_sid: None,
            channel_sid: None,
            date_created: None,
            date_updated: None,
            identity: None,
            last_consumed_message_index: None,
            last_consumption_timestamp: None,
            role_sid: None,
            service_sid: None,
            sid: None,
            url: None,
        }
    }
}
