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
pub struct ListFlexFlowResponse {
    #[serde(rename = "flex_flows", skip_serializing_if = "Option::is_none")]
    pub flex_flows: Option<Vec<crate::models::FlexV1FlexFlow>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListChannelResponseMeta>>,
}

impl ListFlexFlowResponse {
    pub fn new() -> ListFlexFlowResponse {
        ListFlexFlowResponse {
            flex_flows: None,
            meta: None,
        }
    }
}

