// 使用RUST和ROCKET框架创建防止SQL注入的程序

// 引入rocket框架和其它必需的库
#[macro_use]
extern crate rocket;

// 定义一个简单的用户结构体
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

// 引入数据库操作库，确保使用参数化查询防止SQL注入
#[macro_use]
extern crate diesel;

// 使用diesel的postgres backend
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::mysql::MysqlConnection;

// 定义数据库模式
table! {
    users (id) {
        id -> Integer,
        name -> Text,
        age -> Integer,
    }
}

// 使用Rocket的路由功能创建一个API端点
#[macro_use]
extern crate serde_derive;

// 引入serde库用于序列化和反序列化
use serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

// 定义一个请求体，用于接收客户端发送的数据
#[derive(Serialize, Deserialize)]
struct UserCreate {
    name: String,
    age: u8,
}

// 定义一个错误类型，用于处理API错误
#[derive(Debug, serde::Serialize)]
enum UserServiceError {
    InvalidInput,
    DbError,
}

// 实现Rocket的Response类型的错误处理
impl<'r> rocket::response::Responder<'r, 'static> for UserServiceError {
    fn respond_to(self, _: &'r rocket::Request) -> rocket::response::Result<'static> {
        match self {
            UserServiceError::InvalidInput => Err(rocket::response::status::BadRequest(Some(serde_json::to_string(&self).unwrap())).into()),
            UserServiceError::DbError => Err(rocket::response::status::InternalServerError(Some(serde_json::to_string(&self).unwrap())).into()),
        }
    }
}

// 定义用户服务模块
mod user_service;

// 入口函数，启动Rocket服务器
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![
            user_service::create_user,
            user_service::get_users,
        ])
}

// 用户服务模块
mod user_service {
    use super::*;

    // 创建用户的API端点
    #[post("/users", format = "json", data = "<user>")]
    fn create_user(user: Json<UserCreate>, conn: diesel::pg::PgConnection) -> Result<Json<User>, UserServiceError> {
        // 使用参数化查询防止SQL注入
        let user = User {
            name: user.name,
            age: user.age,
        };
        diesel::insert_into(users::table)
            .values(&user)
            .execute(&conn)
            .map_err(|_| UserServiceError::DbError)?;

        // 这里为了演示，我们直接返回用户数据，实际应用中应该查询数据库返回新插入的用户数据
        Ok(Json(user))
    }

    // 获取所有用户的API端点
    #[get("/users")]
    fn get_users(conn: diesel::pg::PgConnection) -> Result<Json<Vec<User>>, UserServiceError> {
        // 使用参数化查询防止SQL注入
        let users = users::table
            .load::<User>(&conn)
            .map_err(|_| UserServiceError::DbError)?;

        Ok(Json(users))
    }
}
