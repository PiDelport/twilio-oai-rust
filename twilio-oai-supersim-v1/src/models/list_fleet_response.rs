/*
 * Twilio - Supersim
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListFleetResponse {
    #[serde(rename = "fleets", skip_serializing_if = "Option::is_none")]
    pub fleets: Option<Vec<crate::models::SupersimV1Fleet>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListCommandResponseMeta>>,
}

impl ListFleetResponse {
    pub fn new() -> ListFleetResponse {
        ListFleetResponse {
            fleets: None,
            meta: None,
        }
    }
}


