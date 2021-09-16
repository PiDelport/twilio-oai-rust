/*
 * Twilio - Serverless
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListFunctionVersionResponse {
    #[serde(rename = "function_versions", skip_serializing_if = "Option::is_none")]
    pub function_versions: Option<Vec<crate::models::ServerlessV1ServiceFunctionFunctionVersion>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListServiceResponseMeta>>,
}

impl ListFunctionVersionResponse {
    pub fn new() -> ListFunctionVersionResponse {
        ListFunctionVersionResponse {
            function_versions: None,
            meta: None,
        }
    }
}


