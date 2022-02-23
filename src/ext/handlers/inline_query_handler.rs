use std::future::Future;
use async_trait::async_trait;

use crate::ext::{Context, Handler};
use crate::types::Update;
use crate::{Bot, error::GroupIteration, error::Result};

pub struct InlineQueryHandler<F: Future<Output = Result<GroupIteration>> + Send + 'static> {
    pub callback: fn(Bot, Context) -> F
}

impl <F: Future<Output = Result<GroupIteration>> + Send + 'static> InlineQueryHandler<F> {
    pub fn new(callback: fn(Bot, Context) -> F) -> Box<Self> {
        Box::new(Self {
            callback
        })
    }
}

impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> Clone for InlineQueryHandler<F> {
    fn clone(&self) -> Self {
        Self {
            callback: self.callback.clone(),
        }
    }
}

#[async_trait]
impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> Handler for InlineQueryHandler<F> {
    async fn check_update(&self, _: &Bot, update: &Update) -> bool {
        if update.inline_query.is_some() {
            return true
        }
        false
    }
    async fn handle_update(&self, bot: &Bot, context: &Context) -> Result<GroupIteration> {
        (self.callback)(bot.clone(), context.clone()).await
    }
}
