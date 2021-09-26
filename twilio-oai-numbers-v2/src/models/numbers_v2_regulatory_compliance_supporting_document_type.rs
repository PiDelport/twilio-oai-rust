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
pub struct NumbersV2RegulatoryComplianceSupportingDocumentType {
    /// The required information for creating a Supporting Document
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<serde_json::Value>>,
    /// A human-readable description of the Supporting Document Type resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The machine-readable description of the Supporting Document Type resource
    #[serde(rename = "machine_name", skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    /// The unique string that identifies the Supporting Document Type resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The absolute URL of the Supporting Document Type resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl NumbersV2RegulatoryComplianceSupportingDocumentType {
    pub fn new() -> NumbersV2RegulatoryComplianceSupportingDocumentType {
        NumbersV2RegulatoryComplianceSupportingDocumentType {
            fields: None,
            friendly_name: None,
            machine_name: None,
            sid: None,
            url: None,
        }
    }
}
