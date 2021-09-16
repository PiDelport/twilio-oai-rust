/*
 * Twilio - Notify
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NotifyV1Service {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// Deprecated
    #[serde(rename = "alexa_skill_id", skip_serializing_if = "Option::is_none")]
    pub alexa_skill_id: Option<String>,
    /// The SID of the Credential to use for APN Bindings
    #[serde(rename = "apn_credential_sid", skip_serializing_if = "Option::is_none")]
    pub apn_credential_sid: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// Deprecated
    #[serde(rename = "default_alexa_notification_protocol_version", skip_serializing_if = "Option::is_none")]
    pub default_alexa_notification_protocol_version: Option<String>,
    /// The protocol version to use for sending APNS notifications
    #[serde(rename = "default_apn_notification_protocol_version", skip_serializing_if = "Option::is_none")]
    pub default_apn_notification_protocol_version: Option<String>,
    /// The protocol version to use for sending FCM notifications
    #[serde(rename = "default_fcm_notification_protocol_version", skip_serializing_if = "Option::is_none")]
    pub default_fcm_notification_protocol_version: Option<String>,
    /// The protocol version to use for sending GCM notifications
    #[serde(rename = "default_gcm_notification_protocol_version", skip_serializing_if = "Option::is_none")]
    pub default_gcm_notification_protocol_version: Option<String>,
    /// Enable delivery callbacks
    #[serde(rename = "delivery_callback_enabled", skip_serializing_if = "Option::is_none")]
    pub delivery_callback_enabled: Option<bool>,
    /// Webhook URL
    #[serde(rename = "delivery_callback_url", skip_serializing_if = "Option::is_none")]
    pub delivery_callback_url: Option<String>,
    /// Deprecated
    #[serde(rename = "facebook_messenger_page_id", skip_serializing_if = "Option::is_none")]
    pub facebook_messenger_page_id: Option<String>,
    /// The SID of the Credential to use for FCM Bindings
    #[serde(rename = "fcm_credential_sid", skip_serializing_if = "Option::is_none")]
    pub fcm_credential_sid: Option<String>,
    /// The string that you assigned to describe the resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The SID of the Credential to use for GCM Bindings
    #[serde(rename = "gcm_credential_sid", skip_serializing_if = "Option::is_none")]
    pub gcm_credential_sid: Option<String>,
    /// The URLs of the resources related to the service
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// Whether to log notifications
    #[serde(rename = "log_enabled", skip_serializing_if = "Option::is_none")]
    pub log_enabled: Option<bool>,
    /// The SID of the Messaging Service to use for SMS Bindings
    #[serde(rename = "messaging_service_sid", skip_serializing_if = "Option::is_none")]
    pub messaging_service_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The absolute URL of the Service resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl NotifyV1Service {
    pub fn new() -> NotifyV1Service {
        NotifyV1Service {
            account_sid: None,
            alexa_skill_id: None,
            apn_credential_sid: None,
            date_created: None,
            date_updated: None,
            default_alexa_notification_protocol_version: None,
            default_apn_notification_protocol_version: None,
            default_fcm_notification_protocol_version: None,
            default_gcm_notification_protocol_version: None,
            delivery_callback_enabled: None,
            delivery_callback_url: None,
            facebook_messenger_page_id: None,
            fcm_credential_sid: None,
            friendly_name: None,
            gcm_credential_sid: None,
            links: None,
            log_enabled: None,
            messaging_service_sid: None,
            sid: None,
            url: None,
        }
    }
}

