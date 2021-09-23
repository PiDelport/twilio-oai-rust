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
pub struct ChatV2ServiceChannelInvite {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The SID of the Channel the new resource belongs to
    #[serde(rename = "channel_sid", skip_serializing_if = "Option::is_none")]
    pub channel_sid: Option<String>,
    /// The identity of the User that created the invite
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The string that identifies the resource's User
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// The SID of the Role assigned to the member
    #[serde(rename = "role_sid", skip_serializing_if = "Option::is_none")]
    pub role_sid: Option<String>,
    /// The SID of the Service that the resource is associated with
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The absolute URL of the Invite resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ChatV2ServiceChannelInvite {
    pub fn new() -> ChatV2ServiceChannelInvite {
        ChatV2ServiceChannelInvite {
            account_sid: None,
            channel_sid: None,
            created_by: None,
            date_created: None,
            date_updated: None,
            identity: None,
            role_sid: None,
            service_sid: None,
            sid: None,
            url: None,
        }
    }
}


