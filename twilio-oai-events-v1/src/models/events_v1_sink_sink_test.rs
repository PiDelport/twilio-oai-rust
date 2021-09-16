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
pub struct EventsV1SinkSinkTest {
    /// Feedback indicating whether the test event was generated.
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

impl EventsV1SinkSinkTest {
    pub fn new() -> EventsV1SinkSinkTest {
        EventsV1SinkSinkTest {
            result: None,
        }
    }
}

