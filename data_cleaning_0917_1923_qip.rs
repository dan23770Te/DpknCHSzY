use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};

// 定义数据清洗请求结构体
#[derive(Serialize, Deserialize, Debug)]
struct CleanDataRequest {
    // 待清洗的数据
    data: String,
}

// 定义数据清洗后的结构体
#[derive(Serialize, Deserialize, Debug)]
struct CleanDataResponse {
    // 清洗后的数据
    cleaned_data: String,
}

// 数据清洗函数
fn clean_data(data: &str) -> String {
    // 这里可以添加实际的数据清洗逻辑，例如去除特殊字符、空格等
    // 仅为示例，此处仅返回原始数据
    data.to_string()
}

// 数据清洗接口
#[get("/clean_data")]
fn clean_data_api(request: Json<CleanDataRequest>) -> Json<CleanDataResponse> {
    // 调用数据清洗函数
    let cleaned_data = clean_data(&request.data);
    
    // 返回清洗后的数据
    Json(CleanDataResponse { cleaned_data })
}

// 启动ROCKET服务器
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![clean_data_api])
}
