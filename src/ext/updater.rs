use crate::ext::Dispatcher;
use crate::{error, Bot};

pub struct Updater<'a> {
    pub bot: &'a Bot,
    pub allowed_updates: Option<Vec<String>>,
    pub dispatcher: &'a mut Dispatcher<'a>,
    running: bool,
}

impl<'a> Updater<'a> {
    pub fn new(bot: &'a Bot, dispatcher: &'a mut Dispatcher<'a>) -> Self {
        Self {
            running: false,
            allowed_updates: None,
            dispatcher,
            bot,
        }
    }
    pub async fn start_polling(&mut self, drop_pending_updates: bool) -> error::Result<()> {
        let mut offset = 0;
        if drop_pending_updates {
            offset = -2;
        }
        self.running = true;
        while self.running {
            let mut updates = self.bot.get_updates().offset(offset + 1).timeout(10);
            if self.allowed_updates.is_some() {
                updates =
                    updates.allowed_updates(self.allowed_updates.as_ref().unwrap().to_owned());
            }
            for update in updates.send().await?.iter() {
                offset = update.update_id;
                self.dispatcher.process_update(update).await;
            }
        }
        Ok(())
    }
    pub async fn stop(&mut self) {
        self.running = false;
    }
}
