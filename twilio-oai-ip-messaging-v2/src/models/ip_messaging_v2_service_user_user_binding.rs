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
pub struct IpMessagingV2ServiceUserUserBinding {
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    #[serde(rename = "binding_type", skip_serializing_if = "Option::is_none")]
    pub binding_type: Option<BindingType>,
    #[serde(rename = "credential_sid", skip_serializing_if = "Option::is_none")]
    pub credential_sid: Option<String>,
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[serde(rename = "message_types", skip_serializing_if = "Option::is_none")]
    pub message_types: Option<Vec<String>>,
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "user_sid", skip_serializing_if = "Option::is_none")]
    pub user_sid: Option<String>,
}

impl IpMessagingV2ServiceUserUserBinding {
    pub fn new() -> IpMessagingV2ServiceUserUserBinding {
        IpMessagingV2ServiceUserUserBinding {
            account_sid: None,
            binding_type: None,
            credential_sid: None,
            date_created: None,
            date_updated: None,
            endpoint: None,
            identity: None,
            message_types: None,
            service_sid: None,
            sid: None,
            url: None,
            user_sid: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BindingType {
    #[serde(rename = "gcm")]
    Gcm,
    #[serde(rename = "apn")]
    Apn,
    #[serde(rename = "fcm")]
    Fcm,
}
