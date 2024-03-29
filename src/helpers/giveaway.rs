// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::Chat;
use crate::types::Giveaway;

impl Giveaway {
    /// This function creates an empty struct for the object Giveaway.
    pub fn new(chats: Vec<Chat>, winners_selection_date: i64, winner_count: i64) -> Self {
        Self {
            chats,
            winners_selection_date,
            winner_count,
            only_new_members: None,
            has_public_winners: None,
            prize_description: None,
            country_codes: None,
            premium_subscription_month_count: None,
        }
    }
}
