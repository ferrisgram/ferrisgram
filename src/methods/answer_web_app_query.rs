// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::InlineQueryResult;
use crate::types::SentWebAppMessage;
use crate::Bot;

impl Bot {
    /// Use this method to set the result of an interaction with a Web App and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a SentWebAppMessage object is returned.
    /// <https://core.telegram.org/bots/api#answerwebappquery>
    pub fn answer_web_app_query(
        &self,
        web_app_query_id: String,
        result: InlineQueryResult,
    ) -> AnswerWebAppQueryBuilder {
        AnswerWebAppQueryBuilder::new(self, web_app_query_id, result)
    }
}

#[derive(Serialize)]
pub struct AnswerWebAppQueryBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the query to be answered
    pub web_app_query_id: String,
    /// A JSON-serialized object describing the message to be sent
    pub result: InlineQueryResult,
}

impl<'a> AnswerWebAppQueryBuilder<'a> {
    pub fn new(bot: &'a Bot, web_app_query_id: String, result: InlineQueryResult) -> Self {
        Self {
            bot,
            web_app_query_id,
            result,
        }
    }

    pub fn web_app_query_id(mut self, web_app_query_id: String) -> Self {
        self.web_app_query_id = web_app_query_id;
        self
    }

    pub fn result(mut self, result: InlineQueryResult) -> Self {
        self.result = result;
        self
    }

    pub async fn send(self) -> Result<SentWebAppMessage> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("answerWebAppQuery", Some(&form)).await
    }
}
