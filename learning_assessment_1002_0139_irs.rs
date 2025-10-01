use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::http::Status;
use std::error::Error;
use std::fmt;
use rocket::response::status;

// 定义一个结构体来表示学生信息
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Student {
    name: String,
    age: u32,
    scores: Vec<u32>,
}

// 定义一个结构体来表示评估结果
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Assessment {
    student_name: String,
    average_score: f32,
    performance_category: String,
}

// 自定义错误类型
#[derive(Debug, Clone)]
struct AssessmentError {
    message: String,
}

impl fmt::Display for AssessmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AssessmentError {}

// 函数计算平均分
fn calculate_average(scores: &[u32]) -> Result<f32, AssessmentError> {
    if scores.is_empty() {
        return Err(AssessmentError {
            message: "Scores list is empty.".to_string(),
        });
    }
    let sum: u32 = scores.iter().sum();
    Ok(sum as f32 / scores.len() as f32)
}

// 函数对学习效果进行评估
fn evaluate_performance(average_score: f32) -> String {
    match average_score {
        x if x >= 90.0 => "Excellent".to_string(),
        x if x >= 80.0 => "Good".to_string(),
        x if x >= 70.0 => "Average".to_string(),
        x if x >= 60.0 => "Below Average".to_string(),
        _ => "Poor".to_string(),
    }
}

// 路由处理函数
#[get("/assessment/<name>")]
fn assessment(name: String) -> Result<Json<Assessment>, status::Custom<AssessmentError>> {
    let student = Student {
        name: name.clone(),
        age: 18, // 假设所有学生都是18岁
        scores: vec![85, 90, 88, 76], // 假设学生成绩
    };

    // 计算平均分
    let average_score = match calculate_average(&student.scores) {
        Ok(avg) => avg,
        Err(e) => return Err(status::Custom(e, Status::BadRequest)),
    };

    // 评估学习效果
    let performance_category = evaluate_performance(average_score);

    // 返回评估结果
    Ok(Json(Assessment {
        student_name: student.name,
        average_score: average_score,
        performance_category: performance_category,
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![assessment])
}
