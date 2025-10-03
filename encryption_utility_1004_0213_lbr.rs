use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use std::sync::Mutex;
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use block_modes::CbcEncryptor;
use rand::Rng;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use zeroize::Zeroizing;
use zeroize_derive::Zeroize;

// 使用aes-soft加密库，需要添加依赖
// [dependencies]
// aes = "0.8.1"
// block-modes = "0.9.0"
// rand = "0.8.5"
// sha2 = "0.10.2"
// zeroize = { version = "1.5.4", features = ["zeroize_derive"] }

#[macro_use]
extern crate rocket;

// 定义加密配置
#[derive(Debug, Deserialize, Serialize, Clone)]
struct EncryptionConfig {
    secret_key: String,
    iv: String,
}

// 定义加密响应
#[derive(Debug, Serialize)]
struct EncryptionResponse {
    encrypted_data: String,
    iv: String,
}

// 定义解密响应
#[derive(Debug, Serialize)]
struct DecryptionResponse {
    decrypted_data: String,
}

// 工具类，用于加密解密
struct EncryptionUtility {
    encryption_config: EncryptionConfig,
}

impl EncryptionUtility {
    fn new(secret_key: String, iv: String) -> Self {
        EncryptionUtility {
            encryption_config: EncryptionConfig { secret_key, iv },
        }
    }

    fn encrypt(&self, data: &str) -> EncryptionResponse {
        let secret_key = hex::decode(&self.encryption_config.secret_key).unwrap();
        let iv = hex::decode(&self.encryption_config.iv).unwrap();
        let cipher = CbcEncryptor::new_from_slices::<Pkcs7>(
            Aes256::new_from_slice(&secret_key).unwrap(),
            &iv
        ).unwrap();
        let plaintext = data.as_bytes();
        let mut ciphertext = vec![0; cipher.cipher().block_size() + plaintext.len()];
        cipher.encrypt_vec(plaintext, &mut ciphertext);
        let encrypted_data = hex::encode(&ciphertext);
        EncryptionResponse {
            encrypted_data,
            iv: self.encryption_config.iv.clone(),
        }
    }

    fn decrypt(&self, encrypted_data: &str) -> DecryptionResponse {
        let secret_key = hex::decode(&self.encryption_config.secret_key).unwrap();
        let iv = hex::decode(&self.encryption_config.iv).unwrap();
        let cipher = CbcEncryptor::new_from_slices::<Pkcs7>(
            Aes256::new_from_slice(&secret_key).unwrap(),
            &iv
        ).unwrap();
        let ciphertext = hex::decode(encrypted_data).unwrap();
        let mut plaintext = vec![0; cipher.cipher().block_size() + ciphertext.len() - cipher.cipher().block_size()];
        cipher.decrypt_vec(&ciphertext, &mut plaintext).unwrap();
        let decrypted_data = String::from_utf8(plaintext).unwrap();
        DecryptionResponse {
            decrypted_data,
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![encrypt_endpoint, decrypt_endpoint])
        .manage(EncryptionUtility::new("your_secret_key_here".to_string(), "your_iv_here".to_string()))
}

// 加密接口
#[post("/encrypt", format = "json", data = "<request>")]
async fn encrypt_endpoint(utility: &State<EncryptionUtility>, request: Json<EncryptionRequest>) -> Json<EncryptionResponse> {
    let encrypted_data = utility.encrypt(&request.data);
    Json(encrypted_data)
}

// 解密接口
#[post("/decrypt", format = "json", data = "<request>")]
async fn decrypt_endpoint(utility: &State<EncryptionUtility>, request: Json<DecryptionRequest>) -> Json<DecryptionResponse> {
    let decrypted_data = utility.decrypt(&request.encrypted_data);
    Json(decrypted_data)
}

// 加密请求数据
#[derive(Debug, Deserialize, Serialize)]
struct EncryptionRequest {
    #[serde(rename = "data")]
    data: String,
}

// 解密请求数据
#[derive(Debug, Deserialize, Serialize)]
struct DecryptionRequest {
    #[serde(rename = "encrypted_data")]
    encrypted_data: String,
}
