use rocket::response::status;
use rocket::http::Status;
use rocket::request::{Request, FromRequest};
use rocket::Outcome::*;
use std::ops::Deref;
use regex::Regex;
use htmlescape::encode_minimal;

// Checks if the request has been tampered with to prevent XSS.
#[derive(Debug, Clone)]
struct XssGuard;

// Implement `FromRequest` for XssGuard to check for XSS in each request.
impl<'r> FromRequest<'r> for XssGuard {
    type Error = status::BadRequest<&'r str>;

    fn from_request(request: &'r Request<'_>) -> outcome::Outcome<Self, Self::Error> {
        let mut is_safe = true;
        let content_type = request.content_type().map(|s| s.to_string());
        let uri = request.uri().clone();

        // Check query parameters for XSS patterns.
        let query_pairs = request.uri().query().unwrap_or_default();
        for pair in query_pairs.split("&") {
            let (key, value) = pair.split_once("=").unwrap_or((key, ""));
            if is_xss_pattern(&key) || is_xss_pattern(value) {
                is_safe = false;
                break;
            }
        }

        // Check the URI path for XSS patterns.
        if is_xss_pattern(uri.path()) {
            is_safe = false;
        }

        // If the request is not safe, return an error.
        if !is_safe {
            Err(status::BadRequest(Some("XSS detected"), true))
        } else {
            Ok(XssGuard)
        }
    }
}

// Utility function to check if a string contains potential XSS patterns.
fn is_xss_pattern(input: &str) -> bool {
    let xss_pattern = Regex::new(r"(<|>|["\'"])).*").unwrap();
    xss_pattern.is_match(input)
}

// Main function to launch the Rocket application with XSS protection.
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home])
        .attach(XssGuard)
}

// A simple endpoint that returns a welcome message.
#[get("/")]
fn home() -> &'static str {
    "Welcome to the XSS-protected site!"
}
