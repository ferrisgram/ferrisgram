// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::ChatBoostSourceGiveaway;

impl ChatBoostSourceGiveaway {
    /// This function creates an empty struct for the object ChatBoostSourceGiveaway.
    pub fn new(giveaway_message_id: i64) -> Self {
        Self {
            giveaway_message_id,
            user: None,
            is_unclaimed: None,
        }
    }
}
