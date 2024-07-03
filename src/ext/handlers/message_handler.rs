use async_trait::async_trait;
use std::future::Future;
use std::sync::Arc;

use crate::ext::filters::MessageFilter;
use crate::ext::{Context, Handler};
use crate::types::Update;
use crate::{error::GroupIteration, error::Result, Bot};

pub struct MessageHandler<F: Future<Output = Result<GroupIteration>> + Send + 'static> {
    pub callback: fn(Arc<Bot>, Context) -> F,
    pub filter: Box<dyn MessageFilter>,
    pub allow_edited: bool,
    pub allow_channel: bool,
}

impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> MessageHandler<F> {
    pub fn new(callback: fn(Arc<Bot>, Context) -> F, filter: Box<dyn MessageFilter>) -> Box<Self> {
        Box::new(Self {
            callback,
            filter,
            allow_channel: false,
            allow_edited: false,
        })
    }
}

impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> Clone for MessageHandler<F> {
    fn clone(&self) -> Self {
        Self {
            callback: self.callback,
            filter: self.filter.clone(),
            allow_channel: self.allow_channel,
            allow_edited: self.allow_edited,
        }
    }
}

#[async_trait]
impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> Handler for MessageHandler<F> {
    async fn check_update(&self, _: Arc<Bot>, update: Arc<Box<Update>>) -> bool {
        if update.message.is_some() {
            let msg = update.message.as_ref().unwrap();
            return self.filter.check_filter(msg);
        }
        if self.allow_edited && update.edited_message.is_some() {
            let msg = update.edited_message.as_ref().unwrap();
            if msg.text.is_none() && msg.caption.is_none() {
                return false;
            }
            return self.filter.check_filter(msg);
        }
        if self.allow_channel && update.channel_post.is_some() {
            let msg = update.channel_post.as_ref().unwrap();
            if msg.text.is_none() && msg.caption.is_none() {
                return false;
            }
            return self.filter.check_filter(msg);
        }
        if self.allow_channel && self.allow_edited && update.edited_channel_post.is_some() {
            let msg = update.edited_channel_post.as_ref().unwrap();
            if msg.text.is_none() && msg.caption.is_none() {
                return false;
            }
            return self.filter.check_filter(msg);
        }
        false
    }
    async fn handle_update(&self, bot: Arc<Bot>, context: &Context) -> Result<GroupIteration> {
        (self.callback)(bot, context.clone()).await
    }
}
