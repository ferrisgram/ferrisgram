// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::OrderInfo;

impl OrderInfo {
    /// This function creates an empty struct for the object OrderInfo.
    pub fn new() -> Self {
        Self {
            name: None,
            phone_number: None,
            email: None,
            shipping_address: None,
        }
    }
}
impl Default for OrderInfo {
    fn default() -> Self {
        Self::new()
    }
}
