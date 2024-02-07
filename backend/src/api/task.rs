use crate::model::task::Course;
use crate::model::task::TaskState;
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
pub enum TaskError {
    TaskNotFound,
    TaskUpdateFailure,
    TaskCreationFailure,
    BadTaskRequest,
}

#[derive(Deserialize)]
pub struct CourseName {
    name: String,
}

impl ResponseError for TaskError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::TaskNotFound => StatusCode::NOT_FOUND,
            TaskError::TaskUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            TaskError::TaskCreationFailure => StatusCode::FAILED_DEPENDENCY,
            TaskError::BadTaskRequest => StatusCode::BAD_REQUEST,
        }
    }
}

#[get("/courses/top_courses")]
pub async fn get_top_coruses(
    ddb_repo: Data<DDBRepository>,
    course_name: Path<CourseName>,
) -> Result<Json<Vec<Course>>, TaskError> {
    
    let courses = ddb_repo.get_top_courses(
        course_name.into_inner().name).await;

    match courses {
        Ok(courses) => Ok(Json(courses)),
        Err(_) => Err(TaskError::TaskNotFound), // Or any other appropriate error
    }
}

#[get("/courses/get/{course_name}")]
pub async fn get_course(
    ddb_repo: Data<DDBRepository>,
    course_name: Path<CourseName>,
) -> Result<Json<Course>, TaskError> {
    
    let courses = ddb_repo.get_course(
        course_name.into_inner().name).await;

    match courses {
        Ok(courses) => Ok(Json(courses)),
        Err(_) => Err(TaskError::TaskNotFound), // Or any other appropriate error
    }
}