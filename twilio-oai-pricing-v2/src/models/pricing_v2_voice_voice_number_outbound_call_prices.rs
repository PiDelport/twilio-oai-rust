/*
 * Twilio - Pricing
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PricingV2VoiceVoiceNumberOutboundCallPrices {
    #[serde(rename = "base_price", skip_serializing_if = "Option::is_none")]
    pub base_price: Option<f32>,
    #[serde(rename = "current_price", skip_serializing_if = "Option::is_none")]
    pub current_price: Option<f32>,
    #[serde(rename = "origination_prefixes", skip_serializing_if = "Option::is_none")]
    pub origination_prefixes: Option<Vec<String>>,
}

impl PricingV2VoiceVoiceNumberOutboundCallPrices {
    pub fn new() -> PricingV2VoiceVoiceNumberOutboundCallPrices {
        PricingV2VoiceVoiceNumberOutboundCallPrices {
            base_price: None,
            current_price: None,
            origination_prefixes: None,
        }
    }
}

