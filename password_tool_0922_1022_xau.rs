use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use rocket::Rocket;
use serde::{Deserialize, Serialize};
use ring::agreement::{EphemeralPrivateKey, X25519};
use ring::{rand, rand::SystemRandom, RngCore};
use ring::signature::{Ed25519KeyPair, KeyPair};

// 定义一个密码加密解密的结构体
#[derive(Debug, Deserialize, Serialize)]
struct PasswordTool {
    #[serde(rename = "operation")]
    operation: String, // 加密（encrypt）或解密（decrypt）
    #[serde(rename = "password")]
    password: String, // 待加密或解密的密码
}

// 定义一个响应结构体
#[derive(Debug, Serialize)]
struct Response {
    status: String,
    message: String,
}

// 加密函数
fn encrypt(password: &str) -> Result<String, String> {
    let rng = SystemRandom::new();
    let private_key = Ed25519KeyPair::generate_pkcs8(&rng).map_err(|e| e.to_string())?;
    let public_key = private_key.public_key().as_ref().to_vec();

    let encrypted_password = base64::encode(private_key.sign(&password.as_bytes()).to_bytes());
    Ok(format!("{}", base64::encode(encrypted_password)))
}

// 解密函数
fn decrypt(ciphertext: &str) -> Result<String, String> {
    let decoded_ciphertext = base64::decode(ciphertext).map_err(|e| e.to_string())?;
    let signature = decoded_ciphertext;

    let public_key = Ed25519KeyPair::from_pkcs8(&signature).map_err(|e| e.to_string())?;
    let decrypted_password = public_key.verify(signature).map_err(|e| e.to_string())?;

    Ok(String::from_utf8(decrypted_password).map_err(|e| e.to_string())?)
}

#[post("/password_tool", format = "json", data = "<password_tool>")]
async fn password_tool(
    password_tool: Json<PasswordTool>,
    rocket: &State<Rocket>,
) -> status::Custom<Json<Response>> {
    match password_tool.operation.as_str() {
        "encrypt" => {
            match encrypt(&password_tool.password) {
                Ok(encrypted_password) => status::Custom(
                    Status::Ok,
                    Json(Response {
                        status: "success".to_string(),
                        message: encrypted_password,
                    }),
                ),
                Err(e) => status::Custom(
                    Status::InternalServerError,
                    Json(Response {
                        status: "error".to_string(),
                        message: e,
                    }),
                ),
            }
        }
        "decrypt" => {
            match decrypt(&password_tool.password) {
                Ok(decrypted_password) => status::Custom(
                    Status::Ok,
                    Json(Response {
                        status: "success".to_string(),
                        message: decrypted_password,
                    }),
                ),
                Err(e) => status::Custom(
                    Status::InternalServerError,
                    Json(Response {
                        status: "error".to_string(),
                        message: e,
                    }),
                ),
            }
        }
        _ => status::Custom(
            Status::BadRequest,
            Json(Response {
                status: "error".to_string(),
                message: "Invalid operation".to_string(),
            }),
        ),
    }
}

#[launch]
fn rocket() -> Rocket {
    rocket::build().manage(Rocket::new())
}
