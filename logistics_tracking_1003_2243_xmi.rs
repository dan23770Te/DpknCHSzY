use rocket::get;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde_json::json;
use rocket::State;
use std::collections::HashMap;

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize, Debug)]
struct TrackingInfo {
    tracking_number: String,
    status: String,
    last_updated: String,
    next_step: Option<String>,
}

#[derive(Default)]
struct TrackingData {
    // 模拟的物流信息存储
    tracking: HashMap<String, TrackingInfo>,
}

#[get("/track/<tracking_number>")]
fn track(tracking_number: String, data: &State<TrackingData>) -> Result<json::Json<TrackingInfo>, String> {
    match data.tracking.get(&tracking_number) {
        Some(info) => Ok(json!(info)),
        None => Err("Tracking number not found".to_string()),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(TrackingData::default())
        .mount("/", routes![track])
}

fn main() {
    // 初始化模拟的物流信息
    let tracking_data = TrackingData {
        tracking: HashMap::from([
            ("12345678".to_string(), TrackingInfo {
                tracking_number: "12345678".to_string(),
                status: "In Transit".to_string(),
                last_updated: "2023-04-01T12:00:00Z".to_string(),
                next_step: Some("Delivering".to_string()),
            }),
            ("23456789".to_string(), TrackingInfo {
                tracking_number: "23456789".to_string(),
                status: "Delivered".to_string(),
                last_updated: "2023-04-02T15:00:00Z".to_string(),
                next_step: None,
            }),
        ]),
    };
    
    // 启动ROCKET服务器
    rocket()
        .manage(tracking_data)
        .launch();
}
