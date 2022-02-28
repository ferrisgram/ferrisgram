// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]use serde::Serialize;

use crate::Bot;
use crate::error::Result;

impl Bot {
    /// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless unbanned first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    /// <https://core.telegram.org/bots/api#banchatmember>
    pub fn ban_chat_member(&self, chat_id: i64, user_id: i64) -> BanChatMemberBuilder {
        BanChatMemberBuilder::new(self, chat_id, user_id)
    }
}

#[derive(Serialize)]
pub struct BanChatMemberBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: i64,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Date when the user will be unbanned, unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever. Applied for supergroups and channels only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
    /// Pass True to delete all messages from the chat for the user that is being removed. If False, the user will be able to see messages in the group that were sent before the user was removed. Always True for supergroups and channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}


impl <'a> BanChatMemberBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, user_id: i64) -> Self {
        Self{
            bot,
            chat_id,
            user_id,
            until_date: None,
            revoke_messages: None,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
                
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
                
    pub fn until_date(mut self, until_date: i64) -> Self {
        self.until_date = Some(until_date);
        self
    }
                
    pub fn revoke_messages(mut self, revoke_messages: bool) -> Self {
        self.revoke_messages = Some(revoke_messages);
        self
    }
                
    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("banChatMember", Some(&form)).await
    }

}