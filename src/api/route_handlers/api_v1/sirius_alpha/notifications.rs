use crate::common::models::app_settings::TelegramEntity;
use crate::helpers::date::get_formatted_date_time;
use crate::services::notification::NotificationServices;
use actix_web::web;
use chrono::DateTime;
use chrono_tz::Tz;
use std::sync::Arc;
use teloxide_core::requests::ResponseResult;
use teloxide_core::types::Message;

pub type SharedPingNotifications = web::Data<Arc<PingNotifications>>;

#[derive(Debug, Clone)]
pub struct PingNotifications {
    service: NotificationServices,
}

impl PingNotifications {
    pub async fn send_a_device_joined_alert(
        &self,
        device_name: &str,
        device_location: &str,
        activity_time: &DateTime<Tz>,
    ) -> ResponseResult<Message> {
        let time = get_formatted_date_time(activity_time);

        self.service
            .send_message(format!(
                "<strong>{}</strong> in the {} was turned on.\n[{}]",
                device_name, device_location, time
            ))
            .await
    }
    pub async fn send_turn_device_off_alert(
        &self,
        device_name: &str,
        device_location: &str,
        activity_time: &DateTime<Tz>,
    ) -> ResponseResult<Message> {
        let time = get_formatted_date_time(activity_time);

        self.service
            .send_message(format!(
                "<strong>SWITCH OFF</strong> the \"{}\" in the {}.\n[{}]",
                device_name, device_location, time
            ))
            .await
    }

    pub fn new(telegram: &TelegramEntity) -> Arc<Self> {
        Arc::new(Self {
            service: NotificationServices::new(telegram),
        })
    }
}
