// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]use serde::Serialize;

use crate::Bot;
use crate::error::Result;
use crate::types::PassportElementError;

impl Bot {
    /// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns True on success.
    /// Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues.
    /// <https://core.telegram.org/bots/api#setpassportdataerrors>
    pub fn set_passport_data_errors(&self, user_id: i64, errors: Vec<PassportElementError>) -> SetPassportDataErrorsBuilder {
        SetPassportDataErrorsBuilder::new(self, user_id, errors)
    }
}

#[derive(Serialize)]
pub struct SetPassportDataErrorsBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// User identifier
    pub user_id: i64,
    /// A JSON-serialized array describing the errors
    pub errors: Vec<PassportElementError>,
}


impl <'a> SetPassportDataErrorsBuilder<'a> {
    pub fn new(bot: &'a Bot, user_id: i64, errors: Vec<PassportElementError>) -> Self {
        Self{
            bot,
            user_id,
            errors,
        }
    }

    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
                
    pub fn errors(mut self, errors: Vec<PassportElementError>) -> Self {
        self.errors = errors;
        self
    }
                
    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("setPassportDataErrors", Some(&form)).await
    }

}