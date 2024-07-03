use async_trait::async_trait;
use std::future::Future;
use std::sync::Arc;

use crate::ext::filters::ChatJoinRequestFilter;
use crate::ext::{Context, Handler};
use crate::types::Update;
use crate::{error::GroupIteration, error::Result, Bot};

pub struct ChatJoinRequestHandler<F: Future<Output = Result<GroupIteration>> + Send + 'static> {
    pub callback: fn(Arc<Bot>, Context) -> F,
    pub filter: Box<dyn ChatJoinRequestFilter>,
    pub allow_channel: bool,
}

impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> ChatJoinRequestHandler<F> {
    pub fn new(
        callback: fn(Arc<Bot>, Context) -> F,
        filter: Box<dyn ChatJoinRequestFilter>,
    ) -> Box<Self> {
        Box::new(Self {
            callback,
            filter,
            allow_channel: false,
        })
    }
}

impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> Clone
    for ChatJoinRequestHandler<F>
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
    for ChatJoinRequestHandler<F>
{
    async fn check_update(&self, _: Arc<Bot>, update: Arc<Box<Update>>) -> bool {
        if update.chat_join_request.is_none() {
            return false;
        }
        let cjr = update.chat_join_request.as_ref().unwrap();
        if !self.allow_channel && cjr.chat.r#type == "channel" {
            return false;
        }
        self.filter.check_filter(cjr)
    }
    async fn handle_update(&self, bot: Arc<Bot>, context: &Context) -> Result<GroupIteration> {
        (self.callback)(bot, context.clone()).await
    }
}
