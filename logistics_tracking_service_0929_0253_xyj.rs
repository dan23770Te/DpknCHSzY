use rocket::get;
# FIXME: 处理边界情况
use rocket::serde::{Deserialize, Serialize};
use rocket::serde_json::json;
# 扩展功能模块
use rocket::http::Status;
use std::collections::HashMap;
use std::sync::Mutex;
# NOTE: 重要实现细节
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
# 添加错误处理

// 定义一个包裹状态枚举
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
enum PackageStatus {
    Pending,
    InTransit,
    Delivered,
# 扩展功能模块
}
# 扩展功能模块

// 定义一个包裹跟踪信息结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
struct PackageTrackingInfo {
    package_id: String,
    status: PackageStatus,
    last_updated: String, // ISO 8601 datetime format
}

// 使用Mutex保护共享数据
# 改进用户体验
lazy_static! {
# FIXME: 处理边界情况
    static ref PACKAGES: Mutex<HashMap<String, PackageTrackingInfo>> = Lazy::new(|| {
        Mutex::new(HashMap::new())
    });
}
# NOTE: 重要实现细节

// 跟踪系统的服务结构体
#[derive(Default)]
struct TrackingService;

// 实现服务方法
impl TrackingService {
    // 获取包裹跟踪信息
    #[get("/track/<package_id>")]
    fn track(package_id: String) -> Result<json::Json<PackageTrackingInfo>, Status> {
# 优化算法效率
        // 检查包裹ID是否为空
        if package_id.trim().is_empty() {
# 扩展功能模块
            return Err(Status::BadRequest);
        }

        let packages = PACKAGES.lock().unwrap();

        // 查找包裹跟踪信息
        match packages.get(&package_id) {
            Some(info) => Ok(json!(info)),
            None => Err(Status::NotFound),
        }
    }
}

// 启动Rocket服务器
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![TrackingService::track])
}

// 测试数据
fn main() {
# 添加错误处理
    let mut packages = PACKAGES.lock().unwrap();
    packages.insert(
        "1234567890".to_string(),
        PackageTrackingInfo {
            package_id: "1234567890".to_string(),
            status: PackageStatus::InTransit,
            last_updated: "2023-04-01T12:00:00Z".to_string(),
        },
    );

    // 启动Rocket服务器
    rocket().launch();
}

// 解释：
// 1. 使用ROCKET框架的宏定义路由和处理函数。
// 2. 使用Mutex保护全局HashMap，确保线程安全。
// 3. 定义PackageStatus枚举和PackageTrackingInfo结构体，用于表示包裹状态和跟踪信息。
// 4. 实现TrackingService服务，提供跟踪接口。
// 5. 在main函数中初始化测试数据，启动Rocket服务器。