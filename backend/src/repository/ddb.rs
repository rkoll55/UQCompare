use crate::model::models::{Answer, Course, Question, Review};
use aws_config::Config;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use log::error;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

pub struct DDBRepository {
    client: Client,
    table_name: String,
}


#[derive(Debug)]
pub enum DDBError {
    MissingAttribute(String),
    UnexpectedType(String),
    General(String),
}

impl fmt::Display for DDBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DDBError::MissingAttribute(ref attr) => write!(f, "Missing attribute: {}", attr),
            DDBError::UnexpectedType(ref message) => write!(f, "Unexpected type: {}", message),
            DDBError::General(ref message) => write!(f, "General error: {}", message),
        }
    }
}

impl Error for DDBError {
    fn description(&self) -> &str {
        match *self {
            DDBError::MissingAttribute(_) => {
                "An expected attribute is missing from the DynamoDB item"
            }
            DDBError::UnexpectedType(_) => {
                "An attribute in the DynamoDB item has an unexpected type"
            }
            DDBError::General(_) => "A general error occurred in processing the DynamoDB item",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

fn required_item_value(
    key: &str,
    item: &HashMap<String, AttributeValue>,
) -> Result<String, DDBError> {
    match item_value(key, item) {
        Ok(Some(value)) => Ok(value),
        Ok(None) => Err(DDBError::General("required item value".to_string())),
        Err(e) => Err(e),
    }
}

fn item_value(
    key: &str,
    item: &HashMap<String, AttributeValue>,
) -> Result<Option<String>, DDBError> {
    match item.get(key) {
        Some(value) => match value.as_s() {
            Ok(val) => Ok(Some(val.clone())),
            Err(_) => Err(DDBError::General("Item".to_string())),
        },
        None => Ok(None),
    }
}

fn item_to_course(item: &HashMap<String, AttributeValue>) -> Result<Course, DDBError> {

    let course_id = required_item_value("course_id", item)?;
    let category = required_item_value("category", item)?;
    let course_name = required_item_value("name", item)?;
    let description = required_item_value("description", item)?;
    let lecturer = required_item_value("lecturer", item)?;

    // Extract and parse numerical attributes using a helper function
    let average_rating = parse_numeric_attribute(item, "average_rating")?;
    let average_difficulty = parse_numeric_attribute(item, "average_difficulty")?;

    // Extract and process list attributes using a helper function
    let prerequisites_list = parse_list_attribute(item, "prerequisites")?;
    

    Ok(Course {
        course_id,
        category,
        course_name,
        description,
        lecturer,
        average_rating,
        average_difficulty,
        prerequisites: prerequisites_list,
    })
}

// Helper function to parse numeric attributes
fn parse_numeric_attribute(item: &HashMap<String, AttributeValue>, key: &str) -> Result<u8, DDBError> {
    item.get(key)
        .ok_or_else(|| DDBError::MissingAttribute(key.to_string()))
        .and_then(|av| match av {
            AttributeValue::N(number_str) => number_str.parse::<u8>().map_err(|_| DDBError::UnexpectedType(format!("Failed to parse {}", key))),
            _ => Err(DDBError::UnexpectedType(format!("Expected number for {}", key))),
        })
}

// Helper function to parse list attributes
fn parse_list_attribute(item: &HashMap<String, AttributeValue>, key: &str) -> Result<Vec<String>, DDBError> {
    item.get(key)
        .ok_or_else(|| DDBError::MissingAttribute(key.to_string()))
        .and_then(|av| match av {
            AttributeValue::L(list) => list.iter().map(|av| av.as_s().ok_or_else(|| DDBError::UnexpectedType(format!("Expected string in {} list", key)))).collect(),
            _ => Err(DDBError::UnexpectedType(format!("Expected list for {}", key))),
        })
}

fn item_to_review(item: &HashMap<String, AttributeValue>) -> Result<Review, DDBError> {
    let course_id = required_item_value("course_id", item)?;
    let category = required_item_value("category", item)?;
    let rating_str = required_item_value("rating", item)?;
    let rating = rating_str
        .parse::<u8>()
        .map_err(|_| DDBError::UnexpectedType("Item to review".to_string()))?;
    let text = required_item_value("text", item)?;
    let date = required_item_value("date", item)?;

    Ok(Review {
        course_id,
        category,
        rating,
        text,
        date,
    })
}

fn item_to_question(item: &HashMap<String, AttributeValue>) -> Result<Question, DDBError> {
    let course_id = required_item_value("course_id", item)?;
    let category = required_item_value("category", item)?;
    let text = required_item_value("text", item)?;
    let date = required_item_value("date", item)?;

    Ok(Question {
        course_id,
        category,
        text,
        date,
    })
}

fn item_to_answer(item: &HashMap<String, AttributeValue>) -> Result<Answer, DDBError> {
    let course_id = required_item_value("course_id", item)?;
    let category = required_item_value("category", item)?;
    let text = required_item_value("text", item)?;
    let date = required_item_value("date", item)?;

    Ok(Answer {
        course_id,
        category,
        text,
        date,
    })
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

        match request.send().await {
            Ok(_) => Ok(()),
            Err(_) => Err(DDBError::General("Put error".to_string())),
        }
    }

    pub async fn get_course(&self, course_id: String) -> Option<Course> {
        let course_id_av = AttributeValue::S(course_id);
        let category_info = AttributeValue::S(String::from("INFO"));

        let res = self
            .client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression("course_id = :course_id AND category = :category")
            .expression_attribute_values(":course_id", course_id_av)
            .expression_attribute_values(":category", category_info)
            .send()
            .await;

            error!("{:?}",res);
        return match res {
            Ok(output) => match output.items {
                Some(items) => {
                    let item = &items.first()?;
                    match item_to_course(item) {
                        Ok(task) => Some(task),
                        Err(_) => None,
                    }
                }
                None => None,
            },
            Err(error) => {
              //  error!("{:?}", error);
                None
            }
        };
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

        let mut courses = Vec::new();

        match response {
            Ok(response) => {
                match response.items {
                    Some(items) => {
                        for item in items {
                            match item_to_course(&item) {
                                Ok(task) => courses.push(task),
                                Err(_) => break,
                            }
                        }
                    }
                    None => return Ok(courses),
                }
                Ok(courses)
            }
            Err(err) => {
                error!("{:?}", err);
                Err(DDBError::General("Could not access DB".to_string()))
            }
        }
    }

    pub async fn get_top_courses(&self, num_courses: i32) -> Result<Vec<Course>, DDBError> {
        let category_info = AttributeValue::S(String::from("INFO"));

        // I couldn't figure out how to get a specified number from the DB lmao
        let response = self
            .client
            .scan()
            .table_name(&self.table_name)
            .filter_expression("category = :category")
            .expression_attribute_values(":category", category_info)
            .send()
            .await;

        let mut courses = Vec::new();

        match response {
            Ok(response) => {
                match response.items {
                    Some(items) => {
                        let mut count = 0;
                        for item in items {
                            match item_to_course(&item) {
                                Ok(task) => {
                                    if (count >= num_courses) {
                                        break;
                                    }
                                    courses.push(task);
                                    count += 1;
                                }
                                Err(_) => break,
                            }
                        }
                    }
                    None => return Ok(courses),
                }
                Ok(courses)
            }
            Err(err) => {
                error!("{:?}", err);
                Err(DDBError::General("Could not access DB".to_string()))
            }
        }
    }
}
