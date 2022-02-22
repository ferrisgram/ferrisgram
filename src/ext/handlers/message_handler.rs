use std::future::Future;
use async_trait::async_trait;

use crate::ext::{Context, Handler};
use crate::types::Update;
use crate::{Bot, error::GroupIteration, error::Result};

pub struct MessageHandler<F: Future<Output = Result<GroupIteration>> + Send + 'static> {
    pub allow_edited: bool,
    pub allow_channel: bool,
    pub callback: fn(Bot, Context) -> F
}

impl <F: Future<Output = Result<GroupIteration>> + Send + 'static> MessageHandler<F> {
    pub fn new(callback: fn(Bot, Context) -> F) -> Box<Self> {
        Box::new(Self {
            allow_channel: false,
            allow_edited: false,
            callback
        })
    }
}

impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> Clone for MessageHandler<F> {
    fn clone(&self) -> Self {
        Self {
            allow_channel: self.allow_channel.clone(),
            allow_edited: self.allow_edited.clone(),
            callback: self.callback.clone(),
        }
    }
}

#[async_trait]
impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> Handler for MessageHandler<F> {
    async fn check_update(&self, _: &Bot, update: &Update) -> bool {
        if update.message.is_some() {
            return true
        }
        let e_string = Some(String::new());
        if self.allow_edited && update.edited_message.is_some() {
            let msg = update.edited_message.as_ref().unwrap();
            if msg.text == e_string && msg.caption == e_string {
                return false
            }
            return true
        }
        if self.allow_channel && update.channel_post.is_some() {
            let msg = update.channel_post.as_ref().unwrap();
            if msg.text == e_string && msg.caption == e_string {
                return false
            }
            return true
        }
        if self.allow_channel && self.allow_edited && update.edited_channel_post.is_some() {
            let msg = update.edited_channel_post.as_ref().unwrap();
            if msg.text == e_string && msg.caption == e_string {
                return false
            }
            return true
        }
        false
    }
    async fn handle_update(&self, bot: &Bot, context: &Context) -> Result<GroupIteration> {
        (self.callback)(bot.clone(), context.clone()).await
    }
}
