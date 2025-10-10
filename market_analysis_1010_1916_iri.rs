use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::http::Status;
use std::collections::HashMap;

// 定义市场数据分析的结构体
#[derive(Serialize, Deserialize, Debug)]
struct MarketData {
    // 市场数据的字段
    symbol: String,
    price: f64,
    volume: u64,
    timestamp: u64,
}

// 定义市场数据分析服务
#[derive(Default)]
struct MarketAnalysisService {
    // 服务的内部数据
    data: HashMap<String, MarketData>,
}

// 实现MarketAnalysisService的方法
impl MarketAnalysisService {
    // 获取市场数据
    #[get("/market/<symbol>/")]
    fn get_market_data(&self, symbol: String) -> Result<Json<MarketData>, Status> {
        // 检查数据是否存在
        match self.data.get(&symbol) {
            Some(data) => Ok(Json(data.clone())),
            None => Err(Status::NotFound),
        }
    }

    // 添加市场数据
    #[post("/market/", format = "json", data = "<market_data>")]
    fn add_market_data(&mut self, market_data: Json<MarketData>) -> String {
        // 将市场数据添加到内部数据结构
        self.data.insert(market_data.into_inner().symbol.clone(), market_data.into_inner());
        "Market data added successfully".to_string()
    }
}

// 启动Rocket应用
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(MarketAnalysisService::default())
        .mount("/", routes![
            MarketAnalysisService::get_market_data,
            MarketAnalysisService::add_market_data,
        ])
}
