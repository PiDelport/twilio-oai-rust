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
pub struct ChatV2ServiceChannel {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The JSON string that stores application-specific data
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    /// The identity of the User that created the channel
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The string that you assigned to describe the resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// Absolute URLs to access the Members, Messages , Invites and, if it exists, the last Message for the Channel
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The number of Members in the Channel
    #[serde(rename = "members_count", skip_serializing_if = "Option::is_none")]
    pub members_count: Option<i32>,
    /// The number of Messages that have been passed in the Channel
    #[serde(rename = "messages_count", skip_serializing_if = "Option::is_none")]
    pub messages_count: Option<i32>,
    /// The SID of the Service that the resource is associated with
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The visibility of the channel. Can be: `public` or `private`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// An application-defined string that uniquely identifies the resource
    #[serde(rename = "unique_name", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    /// The absolute URL of the Channel resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ChatV2ServiceChannel {
    pub fn new() -> ChatV2ServiceChannel {
        ChatV2ServiceChannel {
            account_sid: None,
            attributes: None,
            created_by: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            links: None,
            members_count: None,
            messages_count: None,
            service_sid: None,
            sid: None,
            _type: None,
            unique_name: None,
            url: None,
        }
    }
}

/// The visibility of the channel. Can be: `public` or `private`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}
