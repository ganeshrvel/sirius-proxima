use crate::common::models::app_settings::TelegramEntity;
use teloxide::requests::{Request, Requester};
use teloxide::types::{ChatId, Recipient};
use teloxide::Bot;
use teloxide_core::requests::{RequesterExt, ResponseResult};
use teloxide_core::types::{Message, ParseMode};

#[derive(Debug, Clone)]
pub struct NotificationServices {
    pub bot: Bot,
    telegram: TelegramEntity,
}

impl NotificationServices {
    pub const fn get_chat_id(&self) -> Recipient {
        let chat_id = self.telegram.chat_id;
        Recipient::Id(ChatId(chat_id))
    }

    pub async fn send_message<T>(&self, text: T) -> ResponseResult<Message>
    where
        T: Into<String>,
    {
        self.bot
            .clone()
            .parse_mode(ParseMode::Html)
            .send_message(self.get_chat_id(), text)
            .send()
            .await
    }

    pub fn new(telegram: &TelegramEntity) -> Self {
        let telegram_token = &telegram.token;
        let b = Bot::new(telegram_token);

        Self {
            bot: b,
            telegram: telegram.clone(),
        }
    }
}
