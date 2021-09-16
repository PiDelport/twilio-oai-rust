/*
 * Twilio - Events
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListSinkResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListSchemaVersionResponseMeta>>,
    #[serde(rename = "sinks", skip_serializing_if = "Option::is_none")]
    pub sinks: Option<Vec<crate::models::EventsV1Sink>>,
}

impl ListSinkResponse {
    pub fn new() -> ListSinkResponse {
        ListSinkResponse {
            meta: None,
            sinks: None,
        }
    }
}

