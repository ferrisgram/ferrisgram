// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::Serialize;

use crate::Bot;
use crate::error::Result;

impl Bot {
    /// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is unbanned, the owner of the banned chat won't be able to send messages on behalf of any of their channels. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns True on success.
    /// <https://core.telegram.org/bots/api#banchatsenderchat>
    pub fn ban_chat_sender_chat(&self, chat_id: i64, sender_chat_id: i64) -> BanChatSenderChatBuilder {
        BanChatSenderChatBuilder::new(&self, chat_id, sender_chat_id)
    }
}

#[derive(Serialize)]
pub struct BanChatSenderChatBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Unique identifier of the target sender chat
    pub sender_chat_id: i64,
}


impl <'a> BanChatSenderChatBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, sender_chat_id: i64) -> Self {
        Self{
            bot,
            chat_id,
            sender_chat_id,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
                
    pub fn sender_chat_id(mut self, sender_chat_id: i64) -> Self {
        self.sender_chat_id = sender_chat_id;
        self
    }
                
    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("banChatSenderChat", Some(&form)).await
    }

}