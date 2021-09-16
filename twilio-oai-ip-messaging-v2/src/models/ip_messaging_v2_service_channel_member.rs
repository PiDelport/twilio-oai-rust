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
pub struct IpMessagingV2ServiceChannelMember {
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    #[serde(rename = "channel_sid", skip_serializing_if = "Option::is_none")]
    pub channel_sid: Option<String>,
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[serde(rename = "last_consumed_message_index", skip_serializing_if = "Option::is_none")]
    pub last_consumed_message_index: Option<i32>,
    #[serde(rename = "last_consumption_timestamp", skip_serializing_if = "Option::is_none")]
    pub last_consumption_timestamp: Option<String>,
    #[serde(rename = "role_sid", skip_serializing_if = "Option::is_none")]
    pub role_sid: Option<String>,
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl IpMessagingV2ServiceChannelMember {
    pub fn new() -> IpMessagingV2ServiceChannelMember {
        IpMessagingV2ServiceChannelMember {
            account_sid: None,
            attributes: None,
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


