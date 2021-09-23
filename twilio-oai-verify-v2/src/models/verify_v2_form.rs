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
pub struct VerifyV2Form {
    /// Additional information for the available forms for this type.
    #[serde(rename = "form_meta", skip_serializing_if = "Option::is_none")]
    pub form_meta: Option<serde_json::Value>,
    /// The Type of this Form
    #[serde(rename = "form_type", skip_serializing_if = "Option::is_none")]
    pub form_type: Option<FormType>,
    /// Object that contains the available forms for this type.
    #[serde(rename = "forms", skip_serializing_if = "Option::is_none")]
    pub forms: Option<serde_json::Value>,
    /// The URL to access the forms for this type.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl VerifyV2Form {
    pub fn new() -> VerifyV2Form {
        VerifyV2Form {
            form_meta: None,
            form_type: None,
            forms: None,
            url: None,
        }
    }
}

/// The Type of this Form
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FormType {
    #[serde(rename = "form-push")]
    FormPush,
}

