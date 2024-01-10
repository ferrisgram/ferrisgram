// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::ShippingAddress;
use serde::{Deserialize, Serialize};

/// This object represents information about an order.
/// <https://core.telegram.org/bots/api#orderinfo>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderInfo {
    /// Optional. User name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional. User's phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Optional. User email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Optional. User shipping address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
}
