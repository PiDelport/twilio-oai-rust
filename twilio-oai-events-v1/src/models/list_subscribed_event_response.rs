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
pub struct ListSubscribedEventResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListSchemaVersionResponseMeta>>,
    #[serde(rename = "types", skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<crate::models::EventsV1SubscriptionSubscribedEvent>>,
}

impl ListSubscribedEventResponse {
    pub fn new() -> ListSubscribedEventResponse {
        ListSubscribedEventResponse {
            meta: None,
            types: None,
        }
    }
}

