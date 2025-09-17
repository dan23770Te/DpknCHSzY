use rocket::serde::json::Json;
use rocket::get;
use rocket::http::Status;
use rocket::response::status;
use rocket::Request;

// 定义一个简单的数据模型
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
}

// 定义一个服务结构体，包含数据模型的实例
pub struct UserService;

// 实现UserService的方法
impl UserService {
    // 获取用户列表
    #[get("/users")]
    pub fn get_users() -> Result<Json<Vec<User>>, status::Custom<&'static str>> {
        let users = vec![
            User { id: 1, username: "user1".to_string(), email: "user1@example.com".to_string() },
            User { id: 2, username: "user2".to_string(), email: "user2@example.com".to_string() },
        ];

        // 模拟错误处理
        if users.is_empty() {
            Err(status::Custom(Status::InternalServerError, "No users found"))
        } else {
            Ok(Json(users))
        }
    }
}

// 火箭框架的启动函数
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![UserService::get_users])
}

// 用于单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn test_get_users() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let mut response = client.get("/users").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let users: Vec<User> = response.json().expect("body should be valid JSON");
        assert_eq!(users.len(), 2);
    }
}
