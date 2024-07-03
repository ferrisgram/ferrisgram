use async_trait::async_trait;
use std::future::Future;
use std::sync::Arc;

use crate::ext::filters::CallbackQueryFilter;
use crate::ext::{Context, Handler};
use crate::types::{MaybeInaccessibleMessage, Update};
use crate::{error::GroupIteration, error::Result, Bot};

pub struct CallbackQueryHandler<F: Future<Output = Result<GroupIteration>> + Send + 'static> {
    pub callback: fn(Arc<Bot>, Context) -> F,
    pub filter: Box<dyn CallbackQueryFilter>,
    pub allow_channel: bool,
}

impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> CallbackQueryHandler<F> {
    pub fn new(
        callback: fn(Arc<Bot>, Context) -> F,
        filter: Box<dyn CallbackQueryFilter>,
    ) -> Box<Self> {
        Box::new(Self {
            callback,
            filter,
            allow_channel: false,
        })
    }
}

impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> Clone
    for CallbackQueryHandler<F>
{
    fn clone(&self) -> Self {
        Self {
            callback: self.callback,
            filter: self.filter.clone(),
            allow_channel: self.allow_channel,
        }
    }
}

#[async_trait]
impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> Handler
    for CallbackQueryHandler<F>
{
    async fn check_update(&self, _: Arc<Bot>, update: Arc<Box<Update>>) -> bool {
        if update.callback_query.is_none() {
            return false;
        }
        let callback_query = update.callback_query.as_ref().unwrap();
        if !self.allow_channel && callback_query.message.is_some() {
            let chat = match callback_query.message.as_ref().unwrap() {
                MaybeInaccessibleMessage::Message(m) => &m.chat,
                MaybeInaccessibleMessage::InaccessibleMessage(m) => &m.chat,
            };
            if chat.r#type == "channel" {
                return false;
            }
        }
        self.filter.check_filter(callback_query)
    }
    async fn handle_update(&self, bot: Arc<Bot>, context: &Context) -> Result<GroupIteration> {
        (self.callback)(bot, context.clone()).await
    }
}
