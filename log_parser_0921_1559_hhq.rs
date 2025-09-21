use rocket::get;
use rocket::response::Content;
use rocket::serde::json::Json;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

// 定义日志文件解析结果的结构体
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

// 定义日志解析器服务
#[get("/log_parser?<file_path>")]
fn log_parser(file_path: PathBuf) -> io::Result<Json<Vec<LogEntry>>> {
    let file = File::open(file_path).map_err(|e| io::Error::new(e.kind(), e.to_string()))?;
    let reader = BufReader::new(file);

    let mut log_entries = Vec::new();
    for line in reader.lines() {
        let line = line?;
        // 假设日志格式为："2023-04-01 12:00:00 INFO Some message"
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue; // 如果不满足基本的日志格式，跳过这行
        }

        let log_entry = LogEntry {
            timestamp: parts[0].to_string(),
            level: parts[1].to_string(),
            message: parts[2..].join(" "),
        };

        log_entries.push(log_entry);
    }

    Ok(Json(log_entries))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![log_parser])
}