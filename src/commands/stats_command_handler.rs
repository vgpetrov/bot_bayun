use crate::SleepInterval;
use chrono::{DateTime, NaiveDateTime, TimeDelta, Utc};
use dashmap::DashMap;
use std::sync::Arc;
use teloxide::prelude::{ChatId, Message, Requester};

pub struct StatsCommandHandler {
    in_timer: Arc<DashMap<ChatId, bool>>,
    states: Arc<DashMap<ChatId, SleepInterval>>,
}

impl StatsCommandHandler {
    pub fn new(
        in_timer: Arc<DashMap<ChatId, bool>>,
        states: Arc<DashMap<ChatId, SleepInterval>>,
    ) -> Self {
        Self { in_timer, states }
    }

    pub fn handle(&self, msg: &Message) -> String {
        let mut result = String::new();

        let mut total_time: TimeDelta = TimeDelta::milliseconds(0);

        self.states
            .get(&msg.chat.id)
            .unwrap()
            .dates_arr
            .iter()
            .for_each(|d| {
                if d.stopped_at.is_some() {
                    let delta =
                        TimeDelta::milliseconds(d.stopped_at.unwrap() - d.started_at.unwrap());
                    result.push_str(&format!(
                        "Started at {}, Stopped at {}, Time spent {}:{} \n",
                        DateTime::from_timestamp_millis(d.started_at.unwrap()).unwrap(),
                        DateTime::from_timestamp_millis(d.stopped_at.unwrap()).unwrap(),
                        delta.num_hours(),
                        delta.num_minutes()
                    ));
                    total_time += delta;
                } else {
                    result.push_str(&format!(
                        "Started at {} \n",
                        DateTime::from_timestamp_millis(d.started_at.unwrap()).unwrap(),
                    ));
                }
            });

        result.push_str(&format!(
            "\nTotal sleep {}:{}",
            total_time.num_minutes() / 60,
            total_time.num_minutes() % 60
        ));

        format!("📊 Current state:\n{}", result)
    }
}

#[test]
fn tst() {
    let input1 = "30.08.2025 15:45:12";
    let input2 = "30.08.2025 17:45:12";

    let input3 = "30.08.2025 18:20:12";
    let input4 = "30.08.2025 19:45:12";

    let format = "%d.%m.%Y %H:%M:%S";

    // First parse to NaiveDateTime (no timezone)
    let naive1 = NaiveDateTime::parse_from_str(input1, format).unwrap();
    let naive2 = NaiveDateTime::parse_from_str(input2, format).unwrap();

    let naive3 = NaiveDateTime::parse_from_str(input3, format).unwrap();
    let naive4 = NaiveDateTime::parse_from_str(input4, format).unwrap();

    // Attach UTC (or another offset if you know it)
    let datetime1: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive1, Utc);
    let datetime2: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive2, Utc);

    let datetime3: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive3, Utc);
    let datetime4: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive4, Utc);

    // let time = DateTime::from_timestamp_millis(datetime.timestamp_millis()).unwrap();
    // println!("Parsed = {}, time = {}", datetime, time);

    let delta1 =
        TimeDelta::milliseconds(datetime2.timestamp_millis() - datetime1.timestamp_millis());
    let delta2 =
        TimeDelta::milliseconds(datetime4.timestamp_millis() - datetime3.timestamp_millis());
    let delta3 = delta1 + delta2;

    println!("{}:{}", delta1.num_hours(), delta1.num_minutes());
    println!("{}:{}", delta2.num_hours(), delta2.num_minutes());
    println!("{}:{}", delta3.num_hours(), delta3.num_minutes());

    println!(
        "{}:{}",
        delta1.num_minutes() % 60,
        delta1.num_minutes() / 60
    );
    println!(
        "{}:{}",
        delta2.num_minutes() % 60,
        delta2.num_minutes() / 60
    );
    println!(
        "{}:{}",
        delta3.num_minutes() % 60,
        delta3.num_minutes() / 60
    );
}
