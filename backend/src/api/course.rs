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
use log::error;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Error as SerdeJsonError;

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

impl From<SerdeJsonError> for CourseError {
    fn from(err: SerdeJsonError) -> Self {
        CourseError::CourseNotFound
    }
}

#[get("/courses/get/{course_code}")]
pub async fn get_course(
    ddb_repo: Data<DDBRepository>,
    course_code: Path<String>,
) -> Result<Json<Course>, CourseError> {
    let course = ddb_repo.get_course(course_code.into_inner()).await;
    match course {
        Some(course) => Ok(Json(course)),
        None => Err(CourseError::CourseNotFound),
    }
}

#[get("/courses/getall")]
pub async fn get_all_courses(
    ddb_repo: Data<DDBRepository>,
) -> Result<Json<Vec<Course>>, CourseError> {
    println!("Error Error Error");
    let courses = ddb_repo.get_all_courses().await;
    match courses {
        Ok(course_list) => Ok(Json(course_list)),
        _ => {
            println!("Error Error Error");
            Err(CourseError::CourseNotFound)
        },
    }
}
