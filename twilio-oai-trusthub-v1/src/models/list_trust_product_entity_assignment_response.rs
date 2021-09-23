/*
 * Twilio - Trusthub
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListTrustProductEntityAssignmentResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListCustomerProfileResponseMeta>>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<crate::models::TrusthubV1TrustProductTrustProductEntityAssignment>>,
}

impl ListTrustProductEntityAssignmentResponse {
    pub fn new() -> ListTrustProductEntityAssignmentResponse {
        ListTrustProductEntityAssignmentResponse {
            meta: None,
            results: None,
        }
    }
}


