use std::sync::Arc;

use crate::{
    helpers_ext::StringPatch,
    types::{Chat, MaybeInaccessibleMessage, Message, Update, User},
};

#[derive(Clone)]
pub struct Context {
    pub effective_user: Option<User>,
    pub effective_chat: Option<Chat>,
    pub effective_message: Option<Message>,
    pub update: Arc<Box<Update>>,
}

impl Context {
    pub fn new(update: Arc<Box<Update>>) -> Arc<Self> {
        let mut ctx = Self {
            effective_user: None,
            effective_chat: None,
            effective_message: None,
            update: update.clone(),
        };
        if update.message.is_some() {
            let msg = update.message.clone().unwrap();
            ctx.effective_message = update.message.clone();
            ctx.effective_user = msg.from;
            ctx.effective_chat = Some(msg.chat)
        } else if update.edited_message.is_some() {
            let msg = update.edited_message.clone().unwrap();
            ctx.effective_message = update.edited_message.clone();
            ctx.effective_user = msg.from;
            ctx.effective_chat = Some(msg.chat)
        } else if update.channel_post.is_some() {
            let msg = update.channel_post.clone().unwrap();
            ctx.effective_message = update.channel_post.clone();
            ctx.effective_chat = Some(msg.chat)
        } else if update.edited_channel_post.is_some() {
            let msg = update.edited_channel_post.clone().unwrap();
            ctx.effective_message = update.edited_channel_post.clone();
            ctx.effective_chat = Some(msg.chat)
        } else if update.inline_query.is_some() {
            let msg = update.inline_query.clone().unwrap();
            ctx.effective_user = Some(msg.from)
        } else if update.callback_query.is_some() {
            let msg = update.callback_query.clone().unwrap();
            if let Some(rmsg) = msg.message {
                match rmsg {
                    MaybeInaccessibleMessage::Message(m) => {
                        ctx.effective_message = Some(m.clone());
                        ctx.effective_chat = Some(m.chat)
                    }
                    MaybeInaccessibleMessage::InaccessibleMessage(m) => {
                        ctx.effective_chat = Some(m.chat)
                    }
                }
            }
            ctx.effective_user = Some(msg.from)
        } else if update.chosen_inline_result.is_some() {
            ctx.effective_user = Some(update.chosen_inline_result.clone().unwrap().from)
        } else if update.shipping_query.is_some() {
            ctx.effective_user = Some(update.shipping_query.clone().unwrap().from)
        } else if update.pre_checkout_query.is_some() {
            ctx.effective_user = Some(update.pre_checkout_query.clone().unwrap().from)
        } else if update.my_chat_member.is_some() {
            let msg = update.my_chat_member.clone().unwrap();
            ctx.effective_user = Some(msg.from);
            ctx.effective_chat = Some(msg.chat)
        } else if update.chat_member.is_some() {
            let msg = update.chat_member.clone().unwrap();
            ctx.effective_user = Some(msg.from);
            ctx.effective_chat = Some(msg.chat)
        } else if update.chat_join_request.is_some() {
            let msg = update.chat_join_request.clone().unwrap();
            ctx.effective_user = Some(msg.from);
            ctx.effective_chat = Some(msg.chat)
        }
        Arc::from(ctx)
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
