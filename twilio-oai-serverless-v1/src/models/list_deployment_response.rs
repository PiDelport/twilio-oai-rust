/*
 * Twilio - Serverless
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListDeploymentResponse {
    #[serde(rename = "deployments", skip_serializing_if = "Option::is_none")]
    pub deployments: Option<Vec<crate::models::ServerlessV1ServiceEnvironmentDeployment>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListServiceResponseMeta>>,
}

impl ListDeploymentResponse {
    pub fn new() -> ListDeploymentResponse {
        ListDeploymentResponse {
            deployments: None,
            meta: None,
        }
    }
}
