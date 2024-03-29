// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::BotShortDescription;
use crate::Bot;

impl Bot {
    /// Use this method to get the current bot short description for the given user language. Returns BotShortDescription on success.
    /// <https://core.telegram.org/bots/api#getmyshortdescription>
    pub fn get_my_short_description(&self) -> GetMyShortDescriptionBuilder {
        GetMyShortDescriptionBuilder::new(self)
    }
}

#[derive(Serialize)]
pub struct GetMyShortDescriptionBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl<'a> GetMyShortDescriptionBuilder<'a> {
    pub fn new(bot: &'a Bot) -> Self {
        Self {
            bot,
            language_code: None,
        }
    }

    pub fn language_code(mut self, language_code: String) -> Self {
        self.language_code = Some(language_code);
        self
    }

    pub async fn send(self) -> Result<BotShortDescription> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("getMyShortDescription", Some(&form)).await
    }
}
