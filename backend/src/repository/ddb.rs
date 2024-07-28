pub use crate::repository::ddb_attributes::*;
use crate::model::models::{Answer, AnswerRequest, Assesments, Course, Question, QuestionRequest, Review, ReviewRequest};
use aws_config::Config;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::Client;
use log::error;
use std::collections::HashMap;
use aws_sdk_dynamodb::output::{ScanOutput, QueryOutput};
use aws_sdk_dynamodb::error::{ScanError, QueryError};

// Takes the database response and a function and returns the Result of that function applied to the 
// unwrapped db data
fn map_db_scan_response<F, T>(response: Result<ScanOutput, SdkError<ScanError>>, f: F) -> Result<Vec<T>, DDBError> 
where
    F: Fn(&HashMap<String, AttributeValue>) -> Result<T, DDBError> {
    response
        .map(|response| {
            response.items.map_or(Ok(Vec::new()), |items| {
                items.iter()
                    .map(f)
                    .collect::<Result<Vec<_>, _>>()
            })
        }).unwrap_or_else(|err| {
            error!("{:?}", err);
            Err(DDBError::General("Could not access DB".to_string()))
        })
}

// need to do same thing for query cause rust doesn't have inheritence 
fn map_db_query_response<F, T>(response: Result<QueryOutput, SdkError<QueryError>>, f: F) -> Result<Vec<T>, DDBError> 
where
    F: Fn(&HashMap<String, AttributeValue>) -> Result<T, DDBError> {
    response
        .map(|response| {
            response.items.map_or(Ok(Vec::new()), |items| {
                items.iter()
                    .map(f)
                    .collect::<Result<Vec<_>, _>>()
            })
        }).unwrap_or_else(|err| {
            error!("{:?}", err);
            Err(DDBError::General("Could not access DB".to_string()))
        })
}

pub struct DDBRepository {
    client: Client,
    table_name: String,
}

impl DDBRepository {

    pub fn init(table_name: String, config: Config) -> DDBRepository {
        let client = Client::new(&config);
        DDBRepository { table_name, client }
    }

    pub async fn put_course(&self, course: Course) -> Result<(), DDBError> {
        let prerequisites_av = course
            .prerequisites
            .into_iter()
            .map(|prerequisite| AttributeValue::S(prerequisite))
            .collect::<Vec<_>>();

        let request = self
            .client
            .put_item()
            .table_name(&self.table_name)
            .item(
                "course_id",
                AttributeValue::S(String::from(course.course_id)),
            )
            .item("category", AttributeValue::S(String::from(course.category)))
            .item(
                "course_name",
                AttributeValue::S(String::from(course.course_name)),
            )
            .item(
                "description",
                AttributeValue::S(String::from(course.description)),
            )
            .item("lecturer", AttributeValue::S(String::from(course.lecturer)))
            .item("prerequisites", AttributeValue::L(prerequisites_av));

        request.send().await.map_or_else(|_| { Err(DDBError::General("Put error".to_string())) }, |_| {Ok(())})
    }

    pub async fn get_course(&self, course_id: String) -> Result<Option<Course>, DDBError> {
        let course_id_av = AttributeValue::S(course_id);
        let category_info = AttributeValue::S(String::from("INFO"));
    
        let response = self
            .client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression("course_id = :course_id AND category = :category")
            .expression_attribute_values(":course_id", course_id_av)
            .expression_attribute_values(":category", category_info)
            .send()
            .await;
    
        map_db_query_response(response, item_to_course).map(|courses| courses.into_iter().next())
    }
    
    pub async fn get_all_courses(&self) -> Result<Vec<Course>, DDBError> {
        let category_info = AttributeValue::S(String::from("INFO"));
        let response = self
            .client
            .scan()
            .table_name(&self.table_name)
            .filter_expression("category = :category")
            .expression_attribute_values(":category", category_info)
            .send()
            .await;

        map_db_scan_response(response, item_to_course)
    }

    pub async fn get_top_courses(&self, num_courses: i32) -> Result<Vec<Course>, DDBError> {
        let category_info = AttributeValue::S(String::from("INFO"));
        let response = self
            .client
            .scan()
            .table_name(&self.table_name)
            .filter_expression("category = :category")
            .expression_attribute_values(":category", category_info)
            .send()
            .await;

        map_db_scan_response(response, item_to_course).map(|courses|courses.into_iter().take(num_courses as usize).collect())
    
    }

    pub async fn get_reviews(&self, course_id: String) -> Result<Vec<Review>, DDBError> {
        let course_id_av = AttributeValue::S(course_id);
        let review_prefix = AttributeValue::S(String::from("REVIEW#"));
        let response = self
            .client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression("course_id = :course_id AND begins_with(category, :review_prefix)")
            .expression_attribute_values(":course_id", course_id_av)
            .expression_attribute_values(":review_prefix", review_prefix)
            .send()
            .await;

        map_db_query_response(response, item_to_review)
    }

    pub async fn put_review(&self, review: ReviewRequest) -> Result<(), DDBError> {
        let request = self
            .client
            .put_item()
            .table_name(&self.table_name)
            .item("course_id", AttributeValue::S(String::from(review.course_id)))
            .item("category", AttributeValue::S(format!("REVIEW#{}", generate_unique_suffix())))
            .item("rating", AttributeValue::N(review.rating.to_string()))
            .item("text", AttributeValue::S(String::from(review.text)));

        match request.send().await {
            Ok(_) => Ok(()),
            Err(_) => Err(DDBError::General("Put error".to_string())),
        }
    }

    pub async fn get_questions(&self, course_id: String) -> Result<Vec<Question>, DDBError> {
        let course_id_av = AttributeValue::S(course_id);
        let question_prefix = AttributeValue::S(String::from("QUESTION#"));

        let response = self
        .client
        .query()
        .table_name(&self.table_name)
        .key_condition_expression("course_id = :course_id AND begins_with(category, :question_prefix)")
        .expression_attribute_values(":course_id", course_id_av)
        .expression_attribute_values(":question_prefix", question_prefix)
        .send()
        .await;

        map_db_query_response(response, item_to_question)
    }
    
    pub async fn put_question(&self, question: QuestionRequest) -> Result<(), DDBError> {
        let request = self
            .client
            .put_item()
            .table_name(&self.table_name)
            .item("course_id", AttributeValue::S(String::from(question.course_id)))
            .item("category", AttributeValue::S(format!("QUESTION#{}", generate_unique_suffix())))
            .item("text", AttributeValue::S(String::from(question.text)))
            .item("date", AttributeValue::S(question.date));

        match request.send().await {
            Ok(_) => Ok(()),
            Err(_) => Err(DDBError::General("Put error".to_string())),
        }
    }

    pub async fn get_answers(&self, course_id: String, question_id: String) -> Result<Vec<Answer>, DDBError> {
        let course_id_av = AttributeValue::S(course_id);
        let answer_prefix = AttributeValue::S(format!("ANSWER#QA#{}", question_id));

        let response = self
        .client
        .query()
        .table_name(&self.table_name)
        .key_condition_expression("course_id = :course_id AND begins_with(category, :answer_prefix)")
        .expression_attribute_values(":course_id", course_id_av)
        .expression_attribute_values(":answer_prefix", answer_prefix)
        .send()
        .await;

        map_db_query_response(response, item_to_answer)
    }
    
    pub async fn put_answer(&self, answer: AnswerRequest) -> Result<(), DDBError> {
        
        let request = self
            .client
            .put_item()
            .table_name(&self.table_name)
            .item("course_id", AttributeValue::S(String::from(answer.course_id)))
            .item("category", AttributeValue::S(format!("ANSWER#QA#{}#{}", answer.question_id, generate_unique_suffix())))
            .item("text", AttributeValue::S(String::from(answer.text)))
            .item("date", AttributeValue::S(answer.date));

        match request.send().await {
            Ok(_) => Ok(()),
            Err(_) => Err(DDBError::General("Put error".to_string())),
        }
    }
}
