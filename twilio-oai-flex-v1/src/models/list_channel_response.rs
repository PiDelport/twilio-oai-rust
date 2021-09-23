/*
 * Twilio - Flex
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListChannelResponse {
    #[serde(rename = "flex_chat_channels", skip_serializing_if = "Option::is_none")]
    pub flex_chat_channels: Option<Vec<crate::models::FlexV1Channel>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListChannelResponseMeta>>,
}

impl ListChannelResponse {
    pub fn new() -> ListChannelResponse {
        ListChannelResponse {
            flex_chat_channels: None,
            meta: None,
        }
    }
}


