use crate::api::routes::api_v1::sirius_alpha::controller::{
    salpha_ping, AppState, SAlphaPingRequest,
};
use actix_web::web::Json;
use actix_web::{put, web, HttpResponse};
use std::sync::Mutex;

#[put("/ping")]
/// Example Json:
/// {
//     "device": {
//         "device_type": "water_heater",
//         "details": {
//             "device_name": "Water Heater",
//             "model": "ESP8266EX",
//             "device_id": "water-heater-abc",
//             "device_location": "Kitchen Room",
//             "device_sdk": "2.2.2-dev(xxxx)/xxxx",
//             "app_version": "1.0.0"
//         }
//     }
// }
pub async fn ping(data: web::Data<Mutex<AppState>>, req: Json<SAlphaPingRequest>) -> HttpResponse {
    salpha_ping(data, req)
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(ping);
}
