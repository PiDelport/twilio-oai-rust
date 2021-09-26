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
pub struct ListPhoneNumberCountryResponse {
    #[serde(rename = "countries", skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<crate::models::PricingV1PhoneNumberPhoneNumberCountry>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListMessagingCountryResponseMeta>>,
}

impl ListPhoneNumberCountryResponse {
    pub fn new() -> ListPhoneNumberCountryResponse {
        ListPhoneNumberCountryResponse {
            countries: None,
            meta: None,
        }
    }
}
