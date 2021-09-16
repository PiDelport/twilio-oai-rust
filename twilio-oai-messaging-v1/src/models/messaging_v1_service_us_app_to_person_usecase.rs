/*
 * Twilio - Messaging
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MessagingV1ServiceUsAppToPersonUsecase {
    /// Human readable A2P Use Case details
    #[serde(rename = "us_app_to_person_usecases", skip_serializing_if = "Option::is_none")]
    pub us_app_to_person_usecases: Option<Vec<serde_json::Value>>,
}

impl MessagingV1ServiceUsAppToPersonUsecase {
    pub fn new() -> MessagingV1ServiceUsAppToPersonUsecase {
        MessagingV1ServiceUsAppToPersonUsecase {
            us_app_to_person_usecases: None,
        }
    }
}

