// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::ShippingAddress;
use crate::types::ShippingQuery;
use crate::types::User;

impl ShippingQuery {
    /// This function creates an empty struct for the object ShippingQuery.
    pub fn new(
        id: String,
        from: User,
        invoice_payload: String,
        shipping_address: ShippingAddress,
    ) -> Self {
        Self {
            id,
            from,
            invoice_payload,
            shipping_address,
        }
    }
}
