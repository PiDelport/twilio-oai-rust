/*
 * Twilio - Flex
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FlexV1FlexFlow {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The channel type
    #[serde(rename = "channel_type", skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<ChannelType>,
    /// The SID of the chat service
    #[serde(rename = "chat_service_sid", skip_serializing_if = "Option::is_none")]
    pub chat_service_sid: Option<String>,
    /// The channel contact's Identity
    #[serde(rename = "contact_identity", skip_serializing_if = "Option::is_none")]
    pub contact_identity: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// Whether the Flex Flow is enabled
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The string that you assigned to describe the resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// An object that contains specific parameters for the integration
    #[serde(rename = "integration", skip_serializing_if = "Option::is_none")]
    pub integration: Option<serde_json::Value>,
    /// The software that will handle inbound messages.
    #[serde(rename = "integration_type", skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<IntegrationType>,
    /// Remove active Proxy sessions if the corresponding Task is deleted.
    #[serde(rename = "janitor_enabled", skip_serializing_if = "Option::is_none")]
    pub janitor_enabled: Option<bool>,
    /// Re-use this chat channel for future interactions with a contact
    #[serde(rename = "long_lived", skip_serializing_if = "Option::is_none")]
    pub long_lived: Option<bool>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The absolute URL of the Flex Flow resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl FlexV1FlexFlow {
    pub fn new() -> FlexV1FlexFlow {
        FlexV1FlexFlow {
            account_sid: None,
            channel_type: None,
            chat_service_sid: None,
            contact_identity: None,
            date_created: None,
            date_updated: None,
            enabled: None,
            friendly_name: None,
            integration: None,
            integration_type: None,
            janitor_enabled: None,
            long_lived: None,
            sid: None,
            url: None,
        }
    }
}

/// The channel type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChannelType {
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "whatsapp")]
    Whatsapp,
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "custom")]
    Custom,
}
/// The software that will handle inbound messages.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IntegrationType {
    #[serde(rename = "studio")]
    Studio,
    #[serde(rename = "external")]
    External,
    #[serde(rename = "task")]
    Task,
}
