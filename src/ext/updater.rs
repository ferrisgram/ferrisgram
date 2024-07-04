use crate::{error, Bot};
use std::sync::Arc;
use tokio::task::AbortHandle;

use super::dispatcher;

pub struct Updater<'a> {
    pub bot: Arc<Bot>,
    pub dispatcher: &'a mut dispatcher::Dispatcher,
    pub allowed_updates: Option<Vec<&'a str>>,
    pub(super) running: bool,
    pub(super) abort_handle: Option<AbortHandle>,
}

impl<'a> Updater<'a> {
    pub fn new(bot: Arc<Bot>, dispatcher: &'a mut dispatcher::Dispatcher) -> Self {
        Self {
            running: false,
            allowed_updates: None,
            dispatcher,
            bot,
            abort_handle: None,
        }
    }
    pub async fn start_polling(&mut self, drop_pending_updates: bool) -> error::Result<()> {
        let mut offset = 0;
        if drop_pending_updates {
            offset = -2;
        }
        self.running = true;
        let mut allowed_updates: Option<Vec<String>> = None;
        if self.allowed_updates.is_some() {
            allowed_updates = Some(
                self.allowed_updates
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            );
        }
        let mut pending_tasks = Vec::new();
        loop {
            if !self.running {
                break;
            }
            let mut updates = self.bot.get_updates().offset(offset + 1).timeout(10);
            if allowed_updates.is_some() {
                updates = updates.allowed_updates(allowed_updates.clone().unwrap());
            }
            for update in updates.send().await?.iter() {
                offset = update.update_id;
                let task = self.dispatcher.process_update(update).await;
                pending_tasks.push(task);
            }
        }
        for task in pending_tasks {
            let _ = task.await;
        }
        Ok(())
    }
    pub async fn stop(mut self) {
        self.running = false;
        if let Some(task) = self.abort_handle {
            task.abort();
        }
    }
}
