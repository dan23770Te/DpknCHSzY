use rocket::get;
use rocket::Route;
use rocket::State;
use rocket_diesel::{DieselServer, Connection};
use diesel::prelude::*;
use diesel::pg::PgConnection;

// 定义数据库模式
#[database("my_database")]
pub struct DbConn(diesel::PgConnection);

// 数据库连接池管理模块
#[macro_use]
mod db_pool_manager {
    use super::*;
    use diesel::r2d2::{self, ConnectionManager};
    use diesel::pg::PgConnection;

    // 定义数据库连接池
    pub struct Pool {
        inner: r2d2::Pool<ConnectionManager<PgConnection>>,
    }

    impl Pool {
        // 创建数据库连接池
        pub fn new(database_url: &str) -> Result<Self, r2d2::Error> {
            let manager = ConnectionManager::<PgConnection>::new(database_url);
            let pool = r2d2::Pool::builder()
                .build(manager)
                .expect("Failed to create pool.");
            Ok(Pool { inner: pool })
        }

        // 获取数据库连接
        pub fn get_connection(&self) -> Result<r2d2::PooledConnection<ConnectionManager<PgConnection>>, r2d2::Error> {
            self.inner.get()
        }
    }
}

// 主程序
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount(