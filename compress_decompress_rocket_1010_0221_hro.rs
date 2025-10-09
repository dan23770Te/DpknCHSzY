use rocket::form::Form;
use rocket::form::StringField;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::serde_json::Value;
use rocket::http::Status;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::fs::File;
use rocket::post;
use rocket::get;
use rocket::Rocket;
use rocket::Config;

// Define a structure to represent the request data
#[derive(FromForm)]
struct CompressRequest {
    #[field(value = "input")]
    content: String,
}

#[post("/compress", data = "<compress_request>")]
fn compress(compress_request: Form<CompressRequest>) -> Result<status::Accepted<Json<String>>, Status> {
    let content = compress_request.into_inner().content;
    let compressed = compress_content(content.as_bytes());
    Ok(status::Accepted(Some(json!{"compressed": String::from_utf8(compressed).expect("Failed to convert compressed data to UTF-8")})))
}

#[post("/decompress", data = "<compress_request>")]
fn decompress(compress_request: Form<CompressRequest>) -> Result<status::Accepted<Json<String>>, Status> {
    let content = compress_request.into_inner().content;
    let decompressed = decompress_content(content.as_bytes());
    Ok(status::Accepted(Some(json!{"decompressed": String::from_utf8(decompressed).expect("Failed to convert decompressed data to UTF-8")})))
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the compression and decompression tool!"
}

// Compress the input content using gzip compression
fn compress_content(input: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(input).expect("Failed to write to encoder");
    encoder.finish().expect("Failed to finish encoding");
    encoder.get_ref().to_vec()
}

// Decompress the input content using gzip decompression
fn decompress_content(input: &[u8]) -> Vec<u8> {
    let mut decoder = GzDecoder::new(&input[..]);
    let mut output = Vec::new();
    decoder.read_to_end(&mut output).expect("Failed to read from decoder");
    output
}

#[launch]
fn rocket() -> Rocket {
    rocket::build()
        .mount("/", routes![index, compress, decompress])
}
