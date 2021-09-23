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
pub struct ChatV1ServiceUser {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The JSON string that stores application-specific data
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The string that you assigned to describe the resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The string that identifies the resource's User
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// Whether the User has a potentially valid Push Notification registration for the Service instance
    #[serde(rename = "is_notifiable", skip_serializing_if = "Option::is_none")]
    pub is_notifiable: Option<bool>,
    /// Whether the User is actively connected to the Service instance and online
    #[serde(rename = "is_online", skip_serializing_if = "Option::is_none")]
    pub is_online: Option<bool>,
    /// The number of Channels this User is a Member of
    #[serde(rename = "joined_channels_count", skip_serializing_if = "Option::is_none")]
    pub joined_channels_count: Option<i32>,
    /// The absolute URLs of the Channel and Binding resources related to the user
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The SID of the assigned to the user
    #[serde(rename = "role_sid", skip_serializing_if = "Option::is_none")]
    pub role_sid: Option<String>,
    /// The SID of the Service that the resource is associated with
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The absolute URL of the User resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ChatV1ServiceUser {
    pub fn new() -> ChatV1ServiceUser {
        ChatV1ServiceUser {
            account_sid: None,
            attributes: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            identity: None,
            is_notifiable: None,
            is_online: None,
            joined_channels_count: None,
            links: None,
            role_sid: None,
            service_sid: None,
            sid: None,
            url: None,
        }
    }
}


