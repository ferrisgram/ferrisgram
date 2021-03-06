// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InputInvoiceMessageContent;

impl InputInvoiceMessageContent {
    /// This function creates an empty struct for the object InputInvoiceMessageContent.
    pub fn new() -> Self {
        Self {
            description: "".to_string(),
            payload: "".to_string(),
            provider_token: "".to_string(),
            currency: "".to_string(),
            prices: Vec::new(),
            max_tip_amount: None,
            suggested_tip_amounts: None,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
        }
    }
}
impl Default for InputInvoiceMessageContent {
    fn default() -> Self {
        Self::new()
    }
}
