use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::response::status;
use rocket::http::Status;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::{self, BufRead};

// 定义一个结构体来存储分析结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalysisResult {
    pub total_count: usize,
    pub mean: f64,
    pub median: f64,
    pub mode: f64,
}

// 定义一个错误类型
#[derive(Debug, Clone)]
pub struct AnalysisError {
    message: String,
}

impl fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AnalysisError {}

impl AnalysisError {
    pub fn new(message: &str) -> AnalysisError {
        AnalysisError { message: message.to_string() }
    }
}

// 定义一个控制器来处理数据分析
#[rocket::main]
mod data_analyzer {
    use super::*;
    use rocket::State;
    use std::fs::File;
    use std::io::BufReader;

    #[get("/analyze")]
    async fn analyze(
        input_file: String,
        state: &State<HashMap<String, AnalysisResult>>,
    ) -> Result<Json<AnalysisResult>, status::Custom<&'static str>> {
        // 尝试打开文件
        let file = File::open(input_file).map_err(|_|
            status::Custom(Status::InternalServerError, "Failed to open file")
        )?;

        // 创建一个 BufReader 来读取文件
        let reader = BufReader::new(file);
        let data: Vec<f64> = reader
            .lines()
            .map(|line| line.map_err(|_|
                status::Custom(Status::InternalServerError, "Failed to read line")
            )?
            .parse::<f64>()
            .map_err(|_|
                status::Custom(Status::BadRequest, "Invalid data format")
            )?)
            .collect::<Result<_, _>>()?;

        // 计算总数量
        let total_count = data.len();
        // 计算平均值
        let mean = data.iter().sum::<f64>() / total_count as f64;
        // 计算中位数
        let median = data.iter().cloned().copied().collect::<Vec<_>>()[median::median(data.iter().cloned().collect::<Vec<_>>()).unwrap_or(0)] as f64;
        // 计算众数
        let mut frequency = HashMap::new();
        for &value in &data {
            *frequency.entry(value).or_insert(0) += 1;
        }
        let max_frequency = *frequency.values().max().unwrap();
        let mode = frequency
            .into_iter()
            .find(|&(_, count)| count == max_frequency)
            .map(|(value, _)| value)
            .unwrap_or(0.0);

        // 将结果存储在共享状态中
        state.insert(input_file.clone(), AnalysisResult {
            total_count,
            mean,
            median,
            mode,
        });

        Ok(Json(AnalysisResult {
            total_count,
            mean,
            median,
            mode,
        }))
    }
}

// 定义一个模块来计算中位数
mod median {
    // 计算中位数的函数
    pub fn median<T: Clone + std::cmp::PartialOrd + Copy>(values: Vec<T>) -> Option<T> {
        let size = values.len();
        if size == 0 {
            return None;
        }
        if size % 2 == 1 {
            Some(*values
                .iter()
                .nth(size / 2)
                .expect("Index out of bounds"))
        } else {
            let mid = size / 2;
            let (left, right) = values.split_at(mid);
            Some(
                left.last()
                    .zip(right.first())
                    .map(|(left, right)| (left + right) / 2)
                    .expect("Index out of bounds"),
            )
        }
    }
}
