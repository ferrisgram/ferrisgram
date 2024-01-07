// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::Chat;
use crate::types::GiveawayWinners;
use crate::types::User;

impl GiveawayWinners {
    /// This function creates an empty struct for the object GiveawayWinners.
    pub fn new(
        chat: Chat,
        giveaway_message_id: i64,
        winners_selection_date: i64,
        winner_count: i64,
        winners: Vec<User>,
    ) -> Self {
        Self {
            chat,
            giveaway_message_id,
            winners_selection_date,
            winner_count,
            winners,
            additional_chat_count: None,
            premium_subscription_month_count: None,
            unclaimed_prize_count: None,
            only_new_members: None,
            was_refunded: None,
            prize_description: None,
        }
    }
}