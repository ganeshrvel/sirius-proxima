use crate::api::helpers::responses::success;
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SAlphaPingResponse {
    turn_buzzer_on: bool,
}

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    let res = SAlphaPingResponse {
        turn_buzzer_on: false,
    };

    let de = serde_value::to_value(res).unwrap();

    success(de)
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(ping);
}
