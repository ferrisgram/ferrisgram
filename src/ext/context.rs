use crate::{
    helpers_ext::StringPatch,
    types::{Chat, Message, Update, User},
};

#[derive(Clone)]
pub struct Context {
    pub effective_user: Option<User>,
    pub effective_chat: Option<Chat>,
    pub effective_message: Option<Box<Message>>,
    pub update: Update,
}

impl Context {
    pub fn new(update: &Update) -> Self {
        let udp = update.clone();
        let mut ctx = Self {
            effective_user: None,
            effective_chat: None,
            effective_message: None,
            update: update.clone(),
        };
        if update.message.is_some() {
            let msg = update.clone().message.unwrap();
            ctx.effective_message = udp.message;
            ctx.effective_user = msg.from;
            ctx.effective_chat = Some(msg.chat)
        } else if update.edited_message.is_some() {
            let msg = update.clone().edited_message.unwrap();
            ctx.effective_message = udp.edited_message;
            ctx.effective_user = msg.from;
            ctx.effective_chat = Some(msg.chat)
        } else if update.channel_post.is_some() {
            let msg = update.clone().channel_post.unwrap();
            ctx.effective_message = udp.channel_post;
            ctx.effective_chat = Some(msg.chat)
        } else if update.edited_channel_post.is_some() {
            let msg = update.clone().edited_channel_post.unwrap();
            ctx.effective_message = udp.edited_channel_post;
            ctx.effective_chat = Some(msg.chat)
        } else if update.inline_query.is_some() {
            let msg = update.clone().inline_query.unwrap();
            ctx.effective_user = Some(msg.from)
        } else if update.callback_query.is_some() {
            let msg = update.clone().callback_query.unwrap();
            ctx.effective_message = Some(Box::new(msg.message.unwrap()));
            ctx.effective_user = Some(msg.from)
        } else if update.chosen_inline_result.is_some() {
            ctx.effective_user = Some(udp.chosen_inline_result.unwrap().from)
        } else if update.shipping_query.is_some() {
            ctx.effective_user = Some(udp.shipping_query.unwrap().from)
        } else if update.pre_checkout_query.is_some() {
            ctx.effective_user = Some(udp.pre_checkout_query.unwrap().from)
        } else if update.my_chat_member.is_some() {
            let msg = update.clone().my_chat_member.unwrap();
            ctx.effective_user = Some(msg.from);
            ctx.effective_chat = Some(msg.chat)
        } else if update.chat_join_request.is_some() {
            let msg = update.clone().chat_join_request.unwrap();
            ctx.effective_user = Some(msg.from);
            ctx.effective_chat = Some(msg.chat)
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
