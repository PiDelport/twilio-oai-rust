/*
 * Twilio - Ip_messaging
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IpMessagingV2ServiceUser {
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[serde(rename = "is_notifiable", skip_serializing_if = "Option::is_none")]
    pub is_notifiable: Option<bool>,
    #[serde(rename = "is_online", skip_serializing_if = "Option::is_none")]
    pub is_online: Option<bool>,
    #[serde(rename = "joined_channels_count", skip_serializing_if = "Option::is_none")]
    pub joined_channels_count: Option<i32>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(rename = "role_sid", skip_serializing_if = "Option::is_none")]
    pub role_sid: Option<String>,
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl IpMessagingV2ServiceUser {
    pub fn new() -> IpMessagingV2ServiceUser {
        IpMessagingV2ServiceUser {
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

