use crate::model::models::Course;
use crate::repository::ddb::DDBRepository;
use actix_web::{
    error::ResponseError,
    http::header::ContentType,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,

};
use derive_more::Display;
use serde_json::{json, Error as SerdeJsonError};

#[derive(Debug, Display)]
pub enum CourseError {
    CourseNotFound,
    CourseUpdateFailure,
    CourseCreationFailure(String),
    BadCourseRequest,
}

impl ResponseError for CourseError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CourseError::CourseNotFound | CourseError::CourseUpdateFailure | CourseError::BadCourseRequest => HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .body(self.to_string()),

            CourseError::CourseCreationFailure(msg) => HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .json(&json!({
                    "error": self.to_string(),
                    "message": msg,
                })),
        }
    }
}

impl From<SerdeJsonError> for CourseError {
    fn from(_err: SerdeJsonError) -> Self {
        CourseError::CourseNotFound
    }
}

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

pub async fn get_all_courses(
    ddb_repo: Data<DDBRepository>,
) -> Result<Json<Vec<Course>>, CourseError> {
    let courses = ddb_repo.get_all_courses().await;
    match courses {
        Ok(course_list) => Ok(Json(course_list)),
        _ => {
            Err(CourseError::CourseNotFound)
        },
    }
}

pub async fn get_top_courses(
    ddb_repo: Data<DDBRepository>,
    num_courses: Path<i32>,
) -> Result<Json<Vec<Course>>, CourseError> {

    let courses = ddb_repo.get_top_courses(num_courses.into_inner()).await;
    match courses {
        Ok(course_list) => Ok(Json(course_list)),
        _ => {
            Err(CourseError::CourseNotFound)
        },
    }
}

pub async fn create_course(
    ddb_repo: Data<DDBRepository>,
    new_course: Json<Course>,
) -> Result<HttpResponse, CourseError> {

    let course = Course {
        course_id: new_course.course_id.clone(),
        category: new_course.category.clone(),
        course_name: new_course.course_name.clone(),
        description: new_course.description.clone(),
        lecturer: new_course.lecturer.clone(),
        prerequisites: new_course.prerequisites.clone(),
    };

    match ddb_repo.put_course(course).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Course added successfully.")),
        Err(e) => Err(CourseError::CourseCreationFailure(format!("Failed to add course: {}", e))),
    }
}
