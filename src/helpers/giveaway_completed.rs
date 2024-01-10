// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::GiveawayCompleted;

impl GiveawayCompleted {
    /// This function creates an empty struct for the object GiveawayCompleted.
    pub fn new(winner_count: i64) -> Self {
        Self {
            winner_count,
            unclaimed_prize_count: None,
            giveaway_message: None,
        }
    }
}
