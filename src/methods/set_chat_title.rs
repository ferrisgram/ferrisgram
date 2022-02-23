// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::Serialize;

use crate::Bot;
use crate::error::Result;

impl Bot {
    /// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    /// <https://core.telegram.org/bots/api#setchattitle>
    pub fn set_chat_title(&self, chat_id: i64, title: String) -> SetChatTitleBuilder {
        SetChatTitleBuilder::new(&self, chat_id, title)
    }
}

#[derive(Serialize)]
pub struct SetChatTitleBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// New chat title, 1-255 characters
    pub title: String,
}


impl <'a> SetChatTitleBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, title: String) -> Self {
        Self{
            bot,
            chat_id,
            title,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
                
    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }
                
    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("setChatTitle", Some(&form)).await
    }

}