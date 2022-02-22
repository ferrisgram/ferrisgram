use crate::types::{User, Chat, Message, Update, CallbackQuery, InlineQuery};

#[derive(Clone)]
pub struct Context {
    pub effective_user: Option<User>,
    pub effective_chat: Chat,
    pub effective_message: Option<Box<Message>>,
    pub callback_query: Option<CallbackQuery>,
    pub inline_query: Option<InlineQuery>,
    pub raw_update: Update,
}

impl Context {
    pub fn new(update: &Update) -> Self {
        // let chat = ;
        let udp = update.clone();
        let mut ctx = Self {
            effective_user: None,
            effective_chat: Chat{
                id: 0, 
                r#type: String::new(), 
                title: Option::default(), 
                username: Option::default(), 
                first_name: Option::default(), 
                last_name: Option::default(), 
                photo: Option::default(), 
                bio: Option::default(), 
                has_private_forwards: Option::default(), 
                description: Option::default(), 
                invite_link: Option::default(), 
                pinned_message: Option::default(), 
                permissions: Option::default(), 
                slow_mode_delay: Option::default(), 
                message_auto_delete_time: Option::default(), 
                has_protected_content: Option::default(), sticker_set_name: Option::default(), can_set_sticker_set: Option::default(), linked_chat_id: Option::default(), location: Option::default() },
            effective_message: None,
            raw_update: update.clone(),
            callback_query: None,
            inline_query: None,
        };
        if update.message.is_some() {
            let msg = update.clone().message.unwrap();
            ctx.effective_message = udp.message;
            if msg.from.is_some() {
                ctx.effective_user = msg.from
            }
            ctx.effective_chat = msg.chat
        }
        return ctx;
    }
}