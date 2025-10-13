use rocket::get;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::http::Status;
use rocket::response::status;
use rocket::outcome::IntoOutcome;
use rocket::serde::json::Json;
use std::result::Result;

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TransactionResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    InvalidTransaction,
    // Add more error types as needed for the application
}

impl From<AppError> for rocket::Response<'static> {
    fn from(error: AppError) -> rocket::Response<'static> {
        match error {
            AppError::InvalidTransaction => status::Custom(Status::BadRequest, "Invalid transaction"),
            // Handle other errors similarly
        }
    }
}

#[get("/<sender>/<receiver>/<amount>")]
#[catch(default)]
fn process_transaction(sender: String, receiver: String, amount: u64) -> Result<Json<TransactionResponse>, rocket::Response<'static>> {
    // Here you would typically interact with a decentralized ledger or blockchain
    // For this example, we are just simulating a transaction
    
    // Check if the transaction is valid (e.g., amount is not zero, sender has enough balance, etc.)
    if amount == 0 {
        return Err(AppError::InvalidTransaction.into());
    }
    
    let response = TransactionResponse {
        status: "success".to_string(),
        message: format!("Transaction from {} to {} of {} units is successful", sender, receiver, amount),
    };
    
    Ok(Json(response))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![process_transaction])
        // Add more routes as needed for the application
}