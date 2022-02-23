// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::Serialize;

use crate::Bot;
use crate::error::Result;
use crate::types::InlineKeyboardMarkup;
use crate::types::Message;

impl Bot {
    /// Use this method to edit live location messages. A location can be edited until its live_period expires or editing is explicitly disabled by a call to stopMessageLiveLocation. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned.
    /// <https://core.telegram.org/bots/api#editmessagelivelocation>
    pub fn edit_message_live_location(&self, latitude: f64, longitude: f64) -> EditMessageLiveLocationBuilder {
        EditMessageLiveLocationBuilder::new(&self, latitude, longitude)
    }
}

#[derive(Serialize)]
pub struct EditMessageLiveLocationBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Latitude of new location
    pub latitude: f64,
    /// Longitude of new location
    pub longitude: f64,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    /// Maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
    /// A JSON-serialized object for a new inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}


impl <'a> EditMessageLiveLocationBuilder<'a> {
    pub fn new(bot: &'a Bot, latitude: f64, longitude: f64) -> Self {
        Self{
            bot,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            latitude,
            longitude,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }
                
    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }
                
    pub fn inline_message_id(mut self, inline_message_id: String) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }
                
    pub fn latitude(mut self, latitude: f64) -> Self {
        self.latitude = latitude;
        self
    }
                
    pub fn longitude(mut self, longitude: f64) -> Self {
        self.longitude = longitude;
        self
    }
                
    pub fn horizontal_accuracy(mut self, horizontal_accuracy: f64) -> Self {
        self.horizontal_accuracy = Some(horizontal_accuracy);
        self
    }
                
    pub fn heading(mut self, heading: i64) -> Self {
        self.heading = Some(heading);
        self
    }
                
    pub fn proximity_alert_radius(mut self, proximity_alert_radius: i64) -> Self {
        self.proximity_alert_radius = Some(proximity_alert_radius);
        self
    }
                
    pub fn reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
                
    pub async fn send(self) -> Result<Message> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<Message>("editMessageLiveLocation", Some(&form)).await
    }

}