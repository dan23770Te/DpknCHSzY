use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::response::status;
use rocket::Request;
use rocket::outcome::IntoOutcome;
use rocket::http::Status;
use std::collections::HashMap;

// 定义智能家居设备状态
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SmartHomeDevice {
    pub device_id: String,
    pub is_on: bool,
    pub temperature: Option<f32>,  // 可选的温度值
}

// 智能家居控制器
#[derive(Default)]
pub struct SmartHomeController {
    devices: HashMap<String, SmartHomeDevice>,
}

impl SmartHomeController {
    // 创建新的智能家居控制器
    pub fn new() -> Self {
        SmartHomeController {
            devices: HashMap::new(),
        }
    }

    // 添加设备
    pub fn add_device(&mut self, device: SmartHomeDevice) {
        self.devices.insert(device.device_id.clone(), device);
    }

    // 激活或关闭设备
    pub fn toggle_device(&mut self, device_id: &str) -> Result<(), String> {
        match self.devices.get_mut(device_id) {
            Some(device) => {
                device.is_on = !device.is_on;
                Ok(())
            },
            None => Err(format!("Device with ID '{}' not found.", device_id)),
        }
    }

    // 获取设备状态
    pub fn get_device_status(&self, device_id: &str) -> Result<SmartHomeDevice, String> {
        match self.devices.get(device_id) {
            Some(device) => Ok(device.clone()),
            None => Err(format!("Device with ID '{}' not found.", device_id)),
        }
    }
}

// 定义智能设备状态响应结构
#[derive(Serialize, Deserialize)]
pub struct DeviceStatusResponse {
    pub status: String,
    pub device: SmartHomeDevice,
}

// 火箭的请求处理器
#[rocket::get("/device/<device_id>/status")]
pub fn device_status(mut controller: rocket::State<SmartHomeController>, device_id: String) -> Json<DeviceStatusResponse> {
    let response = controller.get_device_status(&device_id)
        .map(|device| DeviceStatusResponse {
            status: "success".to_string(),
            device,
        })
        .map_err(|error| DeviceStatusResponse {
            status: "error".to_string(),
            device: SmartHomeDevice {
                device_id: device_id.clone(),
                is_on: false,
                temperature: None,
            },
        });
    Json(response.unwrap_or_else(|e| e))
}

#[rocket::get("/device/<device_id>/toggle")]
pub fn toggle_device(mut controller: rocket::State<SmartHomeController>, device_id: String) -> status::Accepted<&'static str> {
    if controller.toggle_device(&device_id).is_ok() {
        status::Accepted::new("Device toggled successfully.")
    } else {
        status::Accepted::new("Error toggling device.")
    }
}

// 启动Rocket应用
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(SmartHomeController::new())
        .mount("/api", routes![device_status, toggle_device])
}

// 注释和文档
/// 该程序是一个简单的智能家居控制器，使用RUST和ROCKET框架。
/// 它允许用户通过HTTP GET请求来获取设备状态和切换设备的开关状态。
/// 设备状态以JSON格式返回，包含设备是否开启和可选的温度信息。
/// 该程序遵循RUST最佳实践，包括错误处理和代码的可维护性。
