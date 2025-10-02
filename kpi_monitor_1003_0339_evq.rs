use rocket::get;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::Route;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io;
use std::time::SystemTime;
use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::serde_json::Value;
use rocket::response::NamedFile;
use rocket::fs::NamedFileOptions;
use rocket::http::RawStr;
use rocket::response::stream::RangeStream;
use rocket::fs::DefaultFileOptions;
use rocket::tokio;
use tokio::time::{self, Duration};
use rocket::fs::FileServer;
use rocket::fs::FileOptions;
use rocket::fs::file_server;

// Define a structure for KPI data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KpiData {
    pub metric_name: String,
    pub current_value: f64,
    pub target_value: f64,
    pub status: String,
    pub timestamp: u64,
}

// Define a structure for KPI threshold
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KpiThreshold {
    pub metric_name: String,
    pub threshold_value: f64,
}

// Define a custom error type
#[derive(Debug)]
pub enum KpiError {
    InvalidMetricName,
    MetricNotFound,
    InvalidThresholdValue,
}

impl fmt::Display for KpiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            KpiError::InvalidMetricName => write!(f, "Invalid metric name"),
            KpiError::MetricNotFound => write!(f, "Metric not found"),
            KpiError::InvalidThresholdValue => write!(f, "Invalid threshold value"),
        }
    }
}

impl Error for KpiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// Define a service for KPI monitoring
#[rocket::get("/metrics")]
async fn get_metrics() -> Result<Json<Vec<KpiData>>, status::Custom<KpiError>> {
    // Mock KPI data for demonstration purposes
    let kpi_data = vec![
        KpiData {
            metric_name: "CPU Usage".to_string(),
            current_value: 75.0,
            target_value: 80.0,
            status: "OK".to_string(),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        },
        KpiData {
            metric_name: "Memory Usage".to_string(),
            current_value: 50.0,
            target_value: 60.0,
            status: "OK".to_string(),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        },
    ];

    // Return the KPI data as JSON
    Ok(Json(kpi_data))
}

// Define a service for checking KPI thresholds
#[rocket::get("/check_thresholds")]
async fn check_thresholds(thresholds: Json<Vec<KpiThreshold>>) -> Result<(), status::Custom<KpiError>> {
    // Mock KPI data for demonstration purposes
    let kpi_data = vec![
        KpiData {
            metric_name: "CPU Usage".to_string(),
            current_value: 75.0,
            target_value: 80.0,
            status: "OK".to_string(),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        },
        KpiData {
            metric_name: "Memory Usage".to_string(),
            current_value: 50.0,
            target_value: 60.0,
            status: "OK".to_string(),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        },
    ];

    // Check each threshold against the KPI data
    for threshold in &thresholds.0 {
        match kpi_data.iter().find(|kpi| kpi.metric_name == threshold.metric_name) {
            Some(kpi) => {
                if kpi.current_value > threshold.threshold_value {
                    return Err(status::Custom(
                        KpiError::MetricNotFound,
                        Status::InternalServerError,
                    ));
                }
            },
            None => {
                return Err(status::Custom(
                    KpiError::MetricNotFound,
                    Status::InternalServerError,
                ));
            },
        }
    }

    // Return success if all thresholds are within limits
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![get_metrics, check_thresholds])
        .attach(DbConn::fairing())
}
