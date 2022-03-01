use crate::methods::SendMessageBuilder;
use crate::types::Message;
use crate::Bot;

impl Message {
    /// This is a helper method to easily build the SendMessageBuilder with current message's chat id and message id.
    pub fn reply<'a>(&self, bot: &'a Bot, text: &'a str) -> SendMessageBuilder<'a> {
        SendMessageBuilder::new(bot, self.chat.id, text.to_string())
            .reply_to_message_id(self.message_id)
    }
    /// This is a helper method to get hyperlink of a message.
    /// It will return an empty string in case of private and group chat type.
    pub fn get_link(&self) -> String {
        if self.chat.r#type == "private" || self.chat.r#type == "group" {
            return String::new();
        }
        if self.chat.username.is_some() {
            return format!(
                "https://t.me/{}/{}",
                self.chat.username.as_ref().unwrap(),
                self.message_id
            );
        }
        format!(
            "https://t.me/c/{}/{}",
            self.chat.id.to_string().trim_start_matches("-100"),
            self.message_id
        )
    }
}
