use crate::SleepInterval;
use std::sync::Arc;
use dashmap::DashMap;
use teloxide::prelude::{ChatId, Message, Requester};
use teloxide::{Bot, RequestError};

pub struct StopCommandHandler {
    in_timer: Arc<DashMap<ChatId, bool>>,
    states: Arc<DashMap<ChatId, SleepInterval>>,
}

impl StopCommandHandler {
    pub fn new(
        in_timer: Arc<DashMap<ChatId, bool>>,
        states: Arc<DashMap<ChatId, SleepInterval>>,
    ) -> Self {
        Self { in_timer, states }
    }

    pub async fn handle(&mut self, bot: &Bot, msg: &Message) -> Result<Message, RequestError> {
        self.in_timer
            .entry(msg.chat.id)
            .and_modify(|v| *v = false);

        self.states
            .entry(msg.chat.id)
            .or_insert(SleepInterval::new())
            .stop_timer(msg.date.timestamp_millis());

        bot.send_message(msg.chat.id, format!("Ends at {}", msg.date))
            .await
    }
}
