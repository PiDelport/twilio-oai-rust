/*
 * Twilio - Pricing
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PricingV1VoiceVoiceNumber {
    /// The name of the country
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "inbound_call_price", skip_serializing_if = "Option::is_none")]
    pub inbound_call_price: Option<Box<crate::models::PricingV1VoiceVoiceNumberInboundCallPrice>>,
    /// The ISO country code
    #[serde(rename = "iso_country", skip_serializing_if = "Option::is_none")]
    pub iso_country: Option<String>,
    /// The phone number
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "outbound_call_price", skip_serializing_if = "Option::is_none")]
    pub outbound_call_price: Option<Box<crate::models::PricingV1VoiceVoiceNumberOutboundCallPrice>>,
    /// The currency in which prices are measured, in ISO 4127 format (e.g. usd, eur, jpy)
    #[serde(rename = "price_unit", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<String>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl PricingV1VoiceVoiceNumber {
    pub fn new() -> PricingV1VoiceVoiceNumber {
        PricingV1VoiceVoiceNumber {
            country: None,
            inbound_call_price: None,
            iso_country: None,
            number: None,
            outbound_call_price: None,
            price_unit: None,
            url: None,
        }
    }
}


