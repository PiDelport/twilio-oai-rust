/*
 * Twilio - Pricing
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

/// PricingV1VoiceVoiceNumberOutboundCallPrice : The OutboundCallPrice record



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PricingV1VoiceVoiceNumberOutboundCallPrice {
    #[serde(rename = "base_price", skip_serializing_if = "Option::is_none")]
    pub base_price: Option<f32>,
    #[serde(rename = "current_price", skip_serializing_if = "Option::is_none")]
    pub current_price: Option<f32>,
}

impl PricingV1VoiceVoiceNumberOutboundCallPrice {
    /// The OutboundCallPrice record
    pub fn new() -> PricingV1VoiceVoiceNumberOutboundCallPrice {
        PricingV1VoiceVoiceNumberOutboundCallPrice {
            base_price: None,
            current_price: None,
        }
    }
}

