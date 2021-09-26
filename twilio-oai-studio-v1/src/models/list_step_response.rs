/*
 * Twilio - Studio
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListStepResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListFlowResponseMeta>>,
    #[serde(rename = "steps", skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<crate::models::StudioV1FlowEngagementStep>>,
}

impl ListStepResponse {
    pub fn new() -> ListStepResponse {
        ListStepResponse {
            meta: None,
            steps: None,
        }
    }
}
