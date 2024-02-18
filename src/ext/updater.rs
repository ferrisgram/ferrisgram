use crate::ext::Dispatcher;
use crate::{error, Bot};
use std::collections::HashMap;

pub struct Updater<'a> {
    pub bot: &'a Bot,
    pub dispatcher: &'a mut Dispatcher<'a>,
    pub allowed_updates: Option<Vec<&'a str>>,
    running: HashMap<&'a str,bool>,
}
 
impl<'a> Updater<'a> {
    fn checker(&mut self) -> bool {
        if *self.running.get("polling").unwrap() || *self.running.get("webhook").unwrap() {
            self.running.get("polling").unwrap() ^ self.running.get("webhook").unwrap()
        } else {
            true
        }
    }
    pub fn new(bot: &'a Bot, dispatcher: &'a mut Dispatcher<'a>) -> Self {
        let run=HashMap::from([
            ("polling",false),
            ("webhook",false)
        ]);
        Self {
            running: run,
            allowed_updates: None,
            dispatcher,
            bot,
        }
    }
    pub async fn start_polling(&mut self, drop_pending_updates: bool) -> error::Result<()> {
        let mut offset = if drop_pending_updates {-2} else {0};
        self.running.insert("polling",true);
        let allowed_updates: Option<Vec<String>> = if self.allowed_updates.is_some() {
            Some(
                self.allowed_updates
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            )
        } else {
            None
        };
        if self.checker() {
            while *self.running.get("polling").unwrap() {
                let mut updates = self.bot.get_updates().offset(offset + 1).timeout(10);
                if allowed_updates.is_some() {
                    updates = updates.allowed_updates(allowed_updates.clone().unwrap());
                }
                for update in updates.send().await?.iter() {
                    offset = update.update_id;
                    self.dispatcher.process_update(update).await;
                }
            }
            Ok(())
        } else {
            Err(error::Error::Conflict("start_polling and start_webhook cannot be called at the same time".to_string()))
        }
    }
    // THis is where i work 
    /* A breifing ... 
     *
     * A server which will wait for telegram to send a request. Can be in actix or hyper. Still not decided
     * (Optional) Both polling and webhook function cannot be running together
     * The recieved request will be verified with a telegram secret token
     * The recieved data will be phrased to Update data type ...
     * (I guess this much for now)
    */
    pub async fn start_webhook(&mut self) -> error::Result<()> {
        self.running.insert("webhook",true);
        if self.checker() {
            Ok(())
        } else {
            Err(error::Error::Conflict("start_polling and start_webhook cannot be used at the same time".to_string()))
        }
    }
    pub async fn stop(&mut self) {
        self.running.insert("polling",false);
        self.running.insert("webhook",false);
    }
}
