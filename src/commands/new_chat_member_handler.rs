use teloxide::{Bot, RequestError};
use teloxide::prelude::{Message, Requester};
use teloxide::types::User;

pub struct NewChatMemberHandler {

}

impl NewChatMemberHandler {

    pub fn new() -> Self {
        Self {}
    }

    pub async fn handle(&mut self, bot: &Bot, msg: &Message, user: &User) -> Result<Message, RequestError> {
        bot.send_message(
            msg.chat.id,
            format!("👋 Welcome, {}!", user.first_name)
        ).await
    }
}