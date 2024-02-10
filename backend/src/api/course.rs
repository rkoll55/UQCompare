use crate::model::models::Course;
use crate::repository::ddb::DDBRepository;
use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};

use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display)]
pub enum CourseError {
    CourseNotFound,
    CourseUpdateFailure,
    CourseCreationFailure,
    BadCourseRequest,
}

#[derive(Deserialize)]
pub struct CourseCode {
    course_id: String,
}

impl ResponseError for CourseError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            CourseError::CourseNotFound => StatusCode::NOT_FOUND,
            CourseError::CourseUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            CourseError::CourseCreationFailure => StatusCode::FAILED_DEPENDENCY,
            CourseError::BadCourseRequest => StatusCode::BAD_REQUEST,
        }
    }
}

// #[get("/courses/top_courses")]
// pub async fn get_top_courses(
//     ddb_repo: Data<DDBRepository>,
//     course_name: Path<CourseName>,
// ) -> Result<Json<Vec<Course>>, CourseError> {
    
//     let courses = ddb_repo.get_top_courses(
//         course_name.into_inner().name).await;

//     match courses {
//         Ok(courses) => Ok(Json(courses)),
//         Err(_) => Err(CourseError::CourseNotFound), // Or any other appropriate error
//     }
// }

#[get("/courses/get/{course_code}")]
pub async fn get_course(
    ddb_repo: Data<DDBRepository>,
    course_code: Path<String>,
) -> Result<Json<Course>, CourseError> {
     
    let course = ddb_repo.get_course(
        course_code.into_inner()).await;

    println!("Match Response: {:?}", course);
    match course {
        Some(course) => Ok(Json(course)),
        None => Err(CourseError::CourseNotFound),
    }
}