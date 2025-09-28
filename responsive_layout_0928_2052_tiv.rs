use rocket::get;
use rocket::response::status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[macro_use] extern crate rocket;

// 定义一个简单的结构体来模拟响应式布局的配置
#[derive(Serialize, Deserialize)]
struct LayoutConfig {
    width: String,
    height: String,
}

// 响应式布局的路由处理函数
#[get(