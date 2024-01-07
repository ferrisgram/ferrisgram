// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::PreCheckoutQuery;
use crate::types::User;

impl PreCheckoutQuery {
    /// This function creates an empty struct for the object PreCheckoutQuery.
    pub fn new(
        id: String,
        from: User,
        currency: String,
        total_amount: i64,
        invoice_payload: String,
    ) -> Self {
        Self {
            id,
            from,
            currency,
            total_amount,
            invoice_payload,
            shipping_option_id: None,
            order_info: None,
        }
    }
}
