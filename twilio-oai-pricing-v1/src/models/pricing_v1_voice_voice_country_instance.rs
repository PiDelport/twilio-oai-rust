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
pub struct PricingV1VoiceVoiceCountryInstance {
    /// The name of the country
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The list of InboundCallPrice records
    #[serde(
        rename = "inbound_call_prices",
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_call_prices:
        Option<Vec<crate::models::PricingV1MessagingMessagingCountryInstanceInboundSmsPrices>>,
    /// The ISO country code
    #[serde(rename = "iso_country", skip_serializing_if = "Option::is_none")]
    pub iso_country: Option<String>,
    /// The list of OutboundPrefixPrice records
    #[serde(
        rename = "outbound_prefix_prices",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_prefix_prices:
        Option<Vec<crate::models::PricingV1VoiceVoiceCountryInstanceOutboundPrefixPrices>>,
    /// The currency in which prices are measured, in ISO 4127 format (e.g. usd, eur, jpy)
    #[serde(rename = "price_unit", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<String>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl PricingV1VoiceVoiceCountryInstance {
    pub fn new() -> PricingV1VoiceVoiceCountryInstance {
        PricingV1VoiceVoiceCountryInstance {
            country: None,
            inbound_call_prices: None,
            iso_country: None,
            outbound_prefix_prices: None,
            price_unit: None,
            url: None,
        }
    }
}
