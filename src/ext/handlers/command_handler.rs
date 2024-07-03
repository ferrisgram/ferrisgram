use async_trait::async_trait;
use std::future::Future;
use std::sync::Arc;

use crate::ext::{Context, Handler};
use crate::types::{Message, Update};
use crate::{error::GroupIteration, error::Result, Bot};

pub struct CommandHandler<'a, F: Future<Output = Result<GroupIteration>> + Send + 'static> {
    pub prefix: Vec<char>,
    pub command: &'a str,
    pub callback: fn(Arc<Bot>, Context) -> F,
    pub allow_edited: bool,
    pub allow_channel: bool,
}

impl<'a, F: Future<Output = Result<GroupIteration>> + Send + 'static> CommandHandler<'a, F> {
    pub fn new(command: &'a str, callback: fn(Arc<Bot>, Context) -> F) -> Box<Self> {
        Box::new(Self {
            prefix: Vec::from(['/']),
            command,
            callback,
            allow_channel: false,
            allow_edited: false,
        })
    }
    async fn check_message(&self, bot: Arc<Bot>, msg: &Message) -> bool {
        let mut text = &String::new();
        if msg.text.is_some() {
            text = msg.text.as_ref().unwrap()
        } else if msg.caption.is_some() {
            text = msg.caption.as_ref().unwrap()
        }
        let text = text.split_whitespace().collect::<Vec<&str>>()[0]
            .to_string()
            .to_lowercase();

        let split = text.split('@').collect::<Vec<&str>>();
        if split.len() > 1 && split[1] != bot.user.username.as_ref().unwrap().to_lowercase() {
            return false;
        }
        let mut cmd = "";
        for c in self.prefix.iter() {
            if !split[0].starts_with(&c.to_string()) {
                continue;
            }
            cmd = split[0];
        }
        if cmd.is_empty() {
            return false;
        }
        &cmd[1..cmd.len()] == self.command
    }
}

#[allow(suspicious_double_ref_op)]
impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> Clone for CommandHandler<'_, F> {
    fn clone(&self) -> Self {
        Self {
            prefix: self.prefix.clone(),
            command: self.command,
            callback: self.callback,
            allow_channel: self.allow_channel,
            allow_edited: self.allow_edited,
        }
    }
}

#[async_trait]
impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> Handler
    for CommandHandler<'_, F>
{
    async fn check_update(&self, bot: Arc<Bot>, update: Arc<Box<Update>>) -> bool {
        if update.message.is_some() {
            let msg = update.message.as_ref().unwrap();
            if msg.text.is_none() && msg.caption.is_none() {
                return false;
            }
            return self.check_message(bot, msg).await;
        }
        if self.allow_edited && update.edited_message.is_some() {
            let msg = update.edited_message.as_ref().unwrap();
            if msg.text.is_none() && msg.caption.is_none() {
                return false;
            }
            return self.check_message(bot, msg).await;
        }
        if self.allow_channel && update.channel_post.is_some() {
            let msg = update.channel_post.as_ref().unwrap();
            if msg.text.is_none() && msg.caption.is_none() {
                return false;
            }
            return self.check_message(bot, msg).await;
        }
        if self.allow_channel && self.allow_edited && update.edited_channel_post.is_some() {
            let msg = update.edited_channel_post.as_ref().unwrap();
            if msg.text.is_none() && msg.caption.is_none() {
                return false;
            }
            return self.check_message(bot, msg).await;
        }
        false
    }
    async fn handle_update(&self, bot: Arc<Bot>, context: &Context) -> Result<GroupIteration> {
        (self.callback)(bot, context.clone()).await
    }
}
