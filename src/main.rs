mod commands;

use crate::commands::help_command_handler::HelpCommandHandler;
use crate::commands::new_chat_member_handler::NewChatMemberHandler;
use crate::commands::start_command_handler::StartCommandHandler;
use crate::commands::stats_command_handler::StatsCommandHandler;
use crate::commands::stop_command_handler::StopCommandHandler;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use dashmap::DashMap;
use teloxide::{prelude::*, utils::command::BotCommands};
use tokio::sync::Mutex;
use tracing::info;

#[derive(Clone)]
struct SleepDuration {
    started_at: Option<i64>,
    stopped_at: Option<i64>,
}

#[derive(Clone)]
struct SleepInterval {
    dates_arr: Vec<SleepDuration>,
}

impl SleepInterval {
    fn new() -> Self {
        Self {
            dates_arr: Vec::new(),
        }
    }

    fn start_timer(&mut self, time: i64) {
        self.dates_arr.push(SleepDuration {
            started_at: Some(time),
            stopped_at: None,
        });
    }

    fn stop_timer(&mut self, time: i64) {
        self.dates_arr.last_mut().unwrap().stopped_at = Some(time);
    }
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "start the timer")]
    Start(String),
    #[command(description = "stop the timer")]
    Stop(String),
    #[command(description = "show stats")]
    Stats,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter("info") // or RUST_LOG env var
        .init();

    info!("Starting Bot Bayun...");

    let bot = Bot::new(env::var("TELEGRAM_API_KEY")?);
    let states: Arc<DashMap<ChatId, SleepInterval>> = Arc::new(DashMap::new());
    let in_timer: Arc<DashMap<ChatId, bool>> = Arc::new(DashMap::new());

    let help_command_handler = Arc::new(Mutex::new(HelpCommandHandler::new()));
    let start_command_handler = Arc::new(Mutex::new(StartCommandHandler::new(
        Arc::clone(&in_timer),
        Arc::clone(&states),
    )));
    let stop_command_handler = Arc::new(Mutex::new(StopCommandHandler::new(
        Arc::clone(&in_timer),
        Arc::clone(&states),
    )));
    let stats_command_handler = Arc::new(Mutex::new(StatsCommandHandler::new(
        Arc::clone(&in_timer),
        Arc::clone(&states),
    )));

    let new_chat_member_handler = Arc::new(Mutex::new(NewChatMemberHandler::new()));

    Command::repl(bot, move |bot: Bot, msg: Message, cmd: Command| {
        let help_command_handler = Arc::clone(&help_command_handler);
        let start_command_handler = Arc::clone(&start_command_handler);
        let stop_command_handler = Arc::clone(&stop_command_handler);
        let stats_command_handler = Arc::clone(&stats_command_handler);
        let new_chat_member_handler = Arc::clone(&new_chat_member_handler);

        async move {
            if let Some(new_chat_members) = msg.new_chat_members() {
                for user in new_chat_members {
                    new_chat_member_handler
                        .lock()
                        .await
                        .handle(&bot, &msg, &user)
                        .await?;
                }
            }
            match cmd {
                Command::Help => {
                    help_command_handler.lock().await.handle(&bot, &msg).await?;
                }
                Command::Start(time) => {
                    start_command_handler
                        .lock()
                        .await
                        .handle(&bot, &msg)
                        .await?;
                }
                Command::Stop(time) => {
                    stop_command_handler.lock().await.handle(&bot, &msg).await?;
                }
                Command::Stats => {
                    stats_command_handler
                        .lock()
                        .await
                        .handle(&bot, &msg)
                        .await?;
                }
            };
            Ok(())
        }
    })
    .await;

    Ok(())
}
