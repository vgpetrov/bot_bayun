use teloxide::Bot;
use teloxide::payloads::SendMessage;
use teloxide::prelude::{Message, Requester};
use teloxide::requests::JsonRequest;
use teloxide::utils::command::BotCommands;
use crate::Command;

pub struct HelpCommandHandler {

}

impl HelpCommandHandler {

    pub fn new() -> Self {
        Self {}
    }

    pub fn handle(&self, bot: &Bot, msg: &Message) -> JsonRequest<SendMessage> {
        bot.send_message(msg.chat.id, Command::descriptions().to_string())
    }
}