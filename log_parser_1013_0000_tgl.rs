use rocket::get;
use rocket::Route;
use rocket::serde::json::Json;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use serde::Deserialize;
use regex::Regex;
use lazy_static::lazy_static;
use rocket_contrib::json::JsonValue;

#[macro_use]
extern crate rocket;
extern crate serde;
extern crate serde_json;
extern crate lazy_static;
extern crate regex;

#[derive(Deserialize)]
#[serde(crate = "serde")]
struct ParseLogConfig {
    file_path: String,
    pattern: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(crate = "serde")]
struct ParsedLogEntry {
    line_number: usize,
    log_entry: String,
}

// 定义一个错误类型，用于处理解析日志时的错误
#[derive(Debug)]
enum ParseLogError {
    FileNotFound,
    IoError(io::Error),
    RegexError(regex::Error),
}

impl From<io::Error> for ParseLogError {
    fn from(err: io::Error) -> Self {
        ParseLogError::IoError(err)
    }
}

impl From<regex::Error> for ParseLogError {
    fn from(err: regex::Error) -> Self {
        ParseLogError::RegexError(err)
    }
}

// 配置Rocket路由
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![parse_log])
}

// Rocket路由处理函数
#[get("/parse")]
fn parse_log(config: Json<ParseLogConfig>) -> Result<JsonValue, ParseLogError> {
    let file_path = &config.file_path;
    let pattern = &config.pattern;
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return Err(ParseLogError::FileNotFound),
    };

    let reader = BufReader::new(file);
    let regex = lazy_static! {
        static ref RE: Regex = Regex::new(pattern).unwrap();
    };

    let mut log_entries = Vec::new();
    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        if regex.is_match(&line) {
            log_entries.push(ParsedLogEntry {
                line_number,
                log_entry: line,
            });
        }
    }

    Ok(serde_json::json!(log_entries))
}

// 定义Rocket的路由
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log() {
        let config = ParseLogConfig {
            file_path: "example.log".to_string(),
            pattern: r"^\[INFO\]".to_string(),
        };
        let result = parse_log(Json(config));
        assert!(result.is_ok());
    }
}
