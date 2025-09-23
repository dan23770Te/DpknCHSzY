#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::http::Status;
use std::result;

// Define a custom error type for mathematical operations
#[derive(Debug, PartialEq)]
enum MathError {
    InvalidInput(String),
    OperationFailed(String),
}

// Define a result type that wraps the MathError
type Result<T> = result::Result<T, MathError>;

// Define a structure for the request payload
#[derive(serde::Deserialize)]
struct MathRequest {
    a: f64,
    b: f64,
    operation: String,
# TODO: 优化性能
}

// Define the MathOperations structure to encapsulate the operations
#[derive(Debug)]
struct MathOperations;

impl MathOperations {
    // Add two numbers
    fn add(a: f64, b: f64) -> Result<f64> {
        Ok(a + b)
    }

    // Subtract two numbers
    fn subtract(a: f64, b: f64) -> Result<f64> {
        Ok(a - b)
    }

    // Multiply two numbers
    fn multiply(a: f64, b: f64) -> Result<f64> {
# FIXME: 处理边界情况
        Ok(a * b)
    }

    // Divide two numbers, with error handling for division by zero
    fn divide(a: f64, b: f64) -> Result<f64> {
        if b == 0.0 {
            Err(MathError::OperationFailed("Cannot divide by zero".to_string()))
        } else {
            Ok(a / b)
        }
# 添加错误处理
    }

    // Perform the requested operation and return the result
    async fn perform_operation(req: &MathRequest) -> Result<Json<f64>> {
        match req.operation.as_str() {
            "add" => Ok(Json(MathOperations::add(req.a, req.b)?)),
            "subtract" => Ok(Json(MathOperations::subtract(req.a, req.b)?)),
            "multiply" => Ok(Json(MathOperations::multiply(req.a, req.b)?)),
            "divide" => Ok(Json(MathOperations::divide(req.a, req.b)?)),
            _ => Err(MathError::InvalidInput("Invalid operation".to_string())),
        }
    }
}

// Define a route for performing mathematical operations
#[post("/calculate", format = "json", data = "<math_req>")]
async fn calculate(math_req: Json<MathRequest>) -> Result<Json<f64>> {
    MathOperations::perform_operation(&math_req.into_inner()).await
}

// Rocket config
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![calculate])
# 增强安全性
}
