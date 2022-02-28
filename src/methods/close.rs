// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::Bot;
use crate::error::Result;

impl Bot {
    /// Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns True on success. Requires no parameters.
    /// <https://core.telegram.org/bots/api#close>
    pub fn close(&self) -> CloseBuilder {
        CloseBuilder::new(self)
    }
}

#[derive(Serialize)]
pub struct CloseBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
}


impl <'a> CloseBuilder<'a> {
    pub fn new(bot: &'a Bot) -> Self {
        Self{
            bot,
        }
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("close", Some(&form)).await
    }

}