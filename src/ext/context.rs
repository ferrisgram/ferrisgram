use crate::{
    helpers_ext::StringPatch,
    types::{Chat, Message, Update, User},
};


pub struct Context<'a> {
    pub effective_user: Option<&'a User>,
    pub effective_chat: Option<&'a Chat>,
    pub effective_message: Option<&'a Message>,
    pub update: &'a Update,
}

impl<'a> Context<'a> {
    pub fn new(update: &'a Update) -> Self {
        let mut ctx = Self {
            effective_user: None,
            effective_chat: None,
            effective_message: None,
            update,
        };
        if let Some(msg) = &update.message {
            ctx.effective_message = Some(msg);
            ctx.effective_user = msg.from.as_ref();
            ctx.effective_chat = Some(&msg.chat);
        } else if let Some(msg) = &update.edited_message {
            ctx.effective_message = Some(msg);
            ctx.effective_user = msg.from.as_ref();
            ctx.effective_chat = Some(&msg.chat);
        } else if let Some(msg) = &update.channel_post {
            ctx.effective_message = Some(msg);
            ctx.effective_chat = Some(&msg.chat);
        } else if let Some(msg) = &update.edited_channel_post {
            ctx.effective_message = Some(msg);
            ctx.effective_chat = Some(&msg.chat);
        } else if let Some(msg) = &update.inline_query {
            ctx.effective_user = Some(&msg.from);
        } else if let Some(msg) = &update.callback_query {
            ctx.effective_message = msg.message.as_ref();
            ctx.effective_user = Some(&msg.from);
        } else if let Some(msg) = &update.chosen_inline_result {
            ctx.effective_user = Some(&msg.from);
        } else if let Some(msg) = &update.shipping_query {
            ctx.effective_user = Some(&msg.from);
        } else if let Some(msg) = &update.pre_checkout_query {
            ctx.effective_user = Some(&msg.from);
        } else if let Some(msg) = &update.my_chat_member {
            ctx.effective_user = Some(&msg.from);
            ctx.effective_chat = Some(&msg.chat);
        } else if let Some(msg) = &update.chat_join_request {
            ctx.effective_user = Some(&msg.from);
            ctx.effective_chat = Some(&msg.chat);
        }
        ctx
    }
    pub fn args(&self) -> Vec<&str> {
        if self.update.callback_query.is_some() {
            let cbq = self.update.callback_query.as_ref().unwrap();
            if cbq.data.is_some() {
                return cbq.data.as_ref().unwrap().get_args();
            }
        } else if self.effective_message.is_some() {
            let m = self.effective_message.as_ref().unwrap();
            if m.text.is_some() {
                return m.text.as_ref().unwrap().get_args();
            } else if m.caption.is_some() {
                return m.caption.as_ref().unwrap().get_args();
            }
        } else if self.update.inline_query.is_some() {
            return self.update.inline_query.as_ref().unwrap().query.get_args();
        }
        Vec::new()
    }
}
