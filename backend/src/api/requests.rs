use crate::model::models::{Course, Question, QuestionRequest, Answer, AnswerRequest, Review, ReviewRequest};
use crate::repository::ddb::DDBRepository;
use actix_web::{
    error::ResponseError,
    Error,
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
    let result = ddb_repo.get_course(course_code.into_inner()).await;

    match result {
        Ok(Some(course)) => Ok(Json(course)), 
        Ok(None) => Err(CourseError::CourseNotFound.into()),
        Err(e) => {
            Err(CourseError::CourseNotFound)
        },
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
        average_difficulty: new_course.average_difficulty.clone(),
        average_rating: new_course.average_rating.clone(),
        prerequisites: new_course.prerequisites.clone(),
        assesments: new_course.assesments.clone(),
    };

    match ddb_repo.put_course(course).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Course added successfully.")),
        Err(e) => Err(CourseError::CourseCreationFailure(format!("Failed to add course: {}", e))),
    }
}

pub async fn get_reviews(
    ddb_repo: Data<DDBRepository>,
    course_code: Path<String>,
) -> Result<Json<Vec<Review>>, CourseError> {
    let result = ddb_repo.get_reviews(course_code.into_inner()).await;

    match result {
        Ok(reviews) => Ok(Json(reviews)),
        Err(_) => Err(CourseError::CourseNotFound),
    }
}

pub async fn create_review(
    ddb_repo: Data<DDBRepository>,
    new_review: Json<ReviewRequest>,

) -> Result<HttpResponse, CourseError> {
    let review = ReviewRequest {
        course_id: new_review.course_id.clone(),
        rating: new_review.rating,
        text: new_review.text.clone(),
    };

    match ddb_repo.put_review(review).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Review added successfully.")),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Failed to add review.")),
    }
}

pub async fn get_questions(
    ddb_repo: Data<DDBRepository>,
    course_code: Path<String>,
) -> Result<Json<Vec<Question>>, CourseError> {
    let result = ddb_repo.get_questions(course_code.into_inner()).await;

    match result {
        Ok(questions) => Ok(Json(questions)),
        Err(_) => Err(CourseError::CourseNotFound),
    }
}

pub async fn create_question(
    ddb_repo: Data<DDBRepository>,
    new_question: Json<QuestionRequest>,

) -> Result<HttpResponse, CourseError> {
    let question = QuestionRequest {
        course_id: new_question.course_id.clone(),
        text: new_question.text.clone(),
        date: new_question.date.clone(),
    };

    match ddb_repo.put_question(question).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Question added successfully.")),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Failed to add question.")),
    }
}

pub async fn get_answers(
    ddb_repo: Data<DDBRepository>,
    path: Path<(String, String)>,
) -> Result<Json<Vec<Answer>>, CourseError> {
    let (course_id, question_id) = path.into_inner();
    let result = ddb_repo.get_answers(course_id, question_id).await;

    match result {
        Ok(answers) => Ok(Json(answers)),
        Err(_) => Err(CourseError::CourseNotFound),
    }
}

pub async fn create_answer(
    ddb_repo: Data<DDBRepository>,
    new_answer: Json<AnswerRequest>,
) -> Result<HttpResponse, CourseError> {
    let answer = AnswerRequest {
        course_id: new_answer.course_id.clone(),
        question_id: new_answer.question_id.clone(),
        text: new_answer.text.clone(),
        date: new_answer.date.clone(),
    };

    match ddb_repo.put_answer(answer).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Answer added successfully.")),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Failed to add answer.")),
    }
}