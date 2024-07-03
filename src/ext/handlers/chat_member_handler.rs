use async_trait::async_trait;
use std::future::Future;
use std::sync::Arc;

// use crate::ext::filters::ChatMemberFilter;
use crate::ext::{Context, Handler};
use crate::types::Update;
use crate::{error::GroupIteration, error::Result, Bot};

pub struct ChatMemberHandler<F: Future<Output = Result<GroupIteration>> + Send + 'static> {
    pub callback: fn(Arc<Bot>, Context) -> F,
    // pub filter: Box<dyn ChatMemberFilter>,
    pub allow_channel: bool,
}

impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> ChatMemberHandler<F> {
    pub fn new(
        callback: fn(Arc<Bot>, Context) -> F,
        // filter: Box<dyn ChatMemberFilter>,
    ) -> Box<Self> {
        Box::new(Self {
            callback,
            // filter,
            allow_channel: false,
        })
    }
}

impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> Clone for ChatMemberHandler<F> {
    fn clone(&self) -> Self {
        Self {
            callback: self.callback,
            // filter: self.filter.clone(),
            allow_channel: self.allow_channel,
        }
    }
}

#[async_trait]
impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> Handler for ChatMemberHandler<F> {
    async fn check_update(&self, _: Arc<Bot>, update: Arc<Box<Update>>) -> bool {
        if update.chat_member.is_none() {
            return false;
        }
        let cjr = update.chat_member.as_ref().unwrap();
        if !self.allow_channel && cjr.chat.r#type == "channel" {
            return false;
        }
        true
    }
    async fn handle_update(&self, bot: Arc<Bot>, context: &Context) -> Result<GroupIteration> {
        (self.callback)(bot, context.clone()).await
    }
}
