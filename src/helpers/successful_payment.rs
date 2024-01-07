// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::SuccessfulPayment;

impl SuccessfulPayment {
    /// This function creates an empty struct for the object SuccessfulPayment.
    pub fn new(
        currency: String,
        total_amount: i64,
        invoice_payload: String,
        telegram_payment_charge_id: String,
        provider_payment_charge_id: String,
    ) -> Self {
        Self {
            currency,
            total_amount,
            invoice_payload,
            shipping_option_id: None,
            order_info: None,
            telegram_payment_charge_id,
            provider_payment_charge_id,
        }
    }
}
