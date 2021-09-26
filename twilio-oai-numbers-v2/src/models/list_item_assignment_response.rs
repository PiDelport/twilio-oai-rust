/*
 * Twilio - Numbers
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListItemAssignmentResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListBundleResponseMeta>>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<crate::models::NumbersV2RegulatoryComplianceBundleItemAssignment>>,
}

impl ListItemAssignmentResponse {
    pub fn new() -> ListItemAssignmentResponse {
        ListItemAssignmentResponse {
            meta: None,
            results: None,
        }
    }
}
