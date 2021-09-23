/*
 * Twilio - Verify
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListRateLimitResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListVerificationAttemptResponseMeta>>,
    #[serde(rename = "rate_limits", skip_serializing_if = "Option::is_none")]
    pub rate_limits: Option<Vec<crate::models::VerifyV2ServiceRateLimit>>,
}

impl ListRateLimitResponse {
    pub fn new() -> ListRateLimitResponse {
        ListRateLimitResponse {
            meta: None,
            rate_limits: None,
        }
    }
}


