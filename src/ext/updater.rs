use crate::{error, Bot};
use crate::types::Update;

pub struct Updater<'a> {
    pub bot: &'a Bot,
    pub allowed_updates: Option<Vec<String>>,
    running: bool
}

impl <'a> Updater<'a> {
    pub fn new(bot: &'a Bot) -> Self {
        Self{
            running: false,
            allowed_updates: None,
            bot
        }
    }
    pub async fn start_polling(&mut self, drop_pending_updates: bool) -> error::Result<()> {
        let mut offset = 0;
        if drop_pending_updates {
            offset = -2;
        }
        self.running = true;
        while self.running {
            let mut updates = self.bot.get_updates()
            .offset(offset+1)
            .timeout(10);
            if self.allowed_updates.is_some() {
                updates = updates.allowed_updates(self.allowed_updates.as_ref().unwrap().to_owned());
            }
            for update in updates.send().await?.iter() {
                offset = update.update_id;
                self.process_update(&self.bot, update).await;
            }
        };
        Ok(())
    }
    pub async fn stop(&mut self) {
        self.running = false;
    }
    pub async fn process_update(&self, bot: &Bot, update: &Update) {
        if update.message.is_some() {
        }
        if update.callback_query.is_some() {
        }
    }
}
