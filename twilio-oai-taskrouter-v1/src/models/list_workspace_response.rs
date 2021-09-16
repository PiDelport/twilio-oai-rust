/*
 * Twilio - Taskrouter
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListWorkspaceResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListWorkspaceResponseMeta>>,
    #[serde(rename = "workspaces", skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<Vec<crate::models::TaskrouterV1Workspace>>,
}

impl ListWorkspaceResponse {
    pub fn new() -> ListWorkspaceResponse {
        ListWorkspaceResponse {
            meta: None,
            workspaces: None,
        }
    }
}


