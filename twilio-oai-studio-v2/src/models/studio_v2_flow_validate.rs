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
pub struct StudioV2FlowValidate {
    /// Boolean if the flow definition is valid
    #[serde(rename = "valid", skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}

impl StudioV2FlowValidate {
    pub fn new() -> StudioV2FlowValidate {
        StudioV2FlowValidate { valid: None }
    }
}
