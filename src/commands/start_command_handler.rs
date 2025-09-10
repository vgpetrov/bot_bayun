use crate::{SleepInterval};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::sync::Arc;
use dashmap::DashMap;
use teloxide::prelude::{ChatId, Message, Requester};
use teloxide::{Bot, RequestError};

pub struct StartCommandHandler {
    in_timer: Arc<DashMap<ChatId, bool>>,
    states: Arc<DashMap<ChatId, SleepInterval>>,
}

impl StartCommandHandler {
    pub fn new(
        in_timer: Arc<DashMap<ChatId, bool>>,
        states: Arc<DashMap<ChatId, SleepInterval>>,
    ) -> Self {
        Self { in_timer, states }
    }

    pub async fn handle(&mut self, bot: &Bot, msg: &Message) -> Result<Message, RequestError> {
        let mut timer_started_already = false;

        self.in_timer
            .entry(msg.chat.id)
            .and_modify(|v| {
                if *v {
                    timer_started_already = true;
                } else {
                    *v = true;
                }
            })
            .or_insert(true);

        if timer_started_already {
            let started_at = self.states
                .get(&msg.chat.id)
                .unwrap()
                .dates_arr
                .last()
                .unwrap()
                .started_at
                .unwrap();

            bot.send_message(
                msg.chat.id,
                format!("Timer already started at {}", DateTime::from_timestamp_millis(started_at).unwrap()),
            )
            .await
        } else {
            self.states
                .entry(msg.chat.id)
                .or_insert(SleepInterval::new())
                .start_timer(msg.date.timestamp_millis());

            bot.send_message(msg.chat.id, format!("Started at {}", msg.date))
                .await
        }
    }
}

#[test]
fn tst() {
    let input = "30.08.2025 15:45:12";
    let format = "%d.%m.%Y %H:%M:%S";

    // First parse to NaiveDateTime (no timezone)
    let naive = NaiveDateTime::parse_from_str(input, format).unwrap();

    // Attach UTC (or another offset if you know it)
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);

    let time = DateTime::from_timestamp_millis(datetime.timestamp_millis()).unwrap();

    println!("Parsed = {}, time = {}", datetime, time);
}
