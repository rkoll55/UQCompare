pub use crate::repository::ddb_error::*;
use crate::model::models::{Answer, Course, Question, Review, Assesments};
use aws_sdk_dynamodb::model::AttributeValue;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn required_item_value( key: &str, item: &HashMap<String, AttributeValue>) -> Result<String, DDBError> {
    item_value(key, item)
    .and_then(|maybe_value| maybe_value.ok_or_else(|| {
        DDBError::General("required item value".to_string())
    }))
}

pub fn item_value(key: &str, item: &HashMap<String, AttributeValue>) -> Result<Option<String>, DDBError> {
    item.get(key)
    .map(|value| value.as_s().map(|val| val.clone()))
    .transpose()
    .map_err(|_| DDBError::General("Item".to_string()))
}

pub fn item_to_course(item: &HashMap<String, AttributeValue>) -> Result<Course, DDBError> {
    let course_id = required_item_value("course_id", item)?;
    let category = required_item_value("category", item)?;
    let course_name = required_item_value("name", item)?;
    let description = required_item_value("description", item)?;
    let lecturer = required_item_value("lecturer", item)?;

    let assesments_av = item.get("assesments")
    .ok_or_else(|| DDBError::MissingAttribute("assesments".to_string()))?;

    let assesments = match assesments_av {
        AttributeValue::M(map) => {
            
            let mut assesments_vec = Vec::new();
            for(key, value) in map.iter() {
                let number = match value {
                    AttributeValue::N(n) => n.parse::<u64>().map_err(|_| {
                        DDBError::UnexpectedType(
                            "Expected number for assessment value".to_string(),
                        )
                    })?,
                    _ => {
                        return Err(DDBError::UnexpectedType(
                            "Expected number for assessment value".to_string()))
                    }
                };
                assesments_vec.push(Assesments{name: key.clone(), weight: number});
            }

            assesments_vec
        }
        _ => { return Err(DDBError::UnexpectedType("Expected map for assessments".to_string())) }
    };

    // Handle prerequisites as a list (assuming it's stored as a List in DynamoDB)
    let average_rating = parse_numeric_attribute("average_rating", item)?;
    let average_difficulty = parse_numeric_attribute("average_difficulty", item)?;
    let prerequisites_list = parse_list_attribute("prerequisites", item)?;

    Ok(Course { course_id, category, course_name, description, lecturer, average_rating, 
        average_difficulty, prerequisites: prerequisites_list, assesments: assesments, })
}

pub fn parse_numeric_attribute(key: &str, item: &HashMap<String, AttributeValue>,) -> Result<u8, DDBError> {
    item.get(key)
        .ok_or_else(|| DDBError::MissingAttribute(key.to_string()))
        .and_then(|av| match av {
            AttributeValue::N(number_str) => number_str.parse::<u8>().map_err(|_| DDBError::UnexpectedType(format!("Failed to parse {}", key))),
            _ => Err(DDBError::UnexpectedType(format!("Expected number for {}", key))),
        })
}

pub fn parse_list_attribute(key: &str, item: &HashMap<String, AttributeValue>) -> Result<Vec<String>, DDBError> {
    match item.get(key) {
        Some(AttributeValue::L(list)) => list.iter().map(|av| match av {
            AttributeValue::S(s) => Ok(s.clone()),
            _ => Err(DDBError::UnexpectedType(format!("Expected string in {} list", key))),
        }).collect(),
        Some(_) => Err(DDBError::UnexpectedType(format!("Expected list for {}", key))),
        None => Err(DDBError::MissingAttribute(key.to_string())),
    }
}

// Helps create a unique name for each answer and question
pub fn generate_unique_suffix() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp_nanos = since_the_epoch.as_nanos().to_string();
    timestamp_nanos
}

pub fn item_to_review(item: &HashMap<String, AttributeValue>) -> Result<Review, DDBError> {
    let course_id = required_item_value("course_id", item)?;
    let category = required_item_value("category", item)?;
    let rating = parse_numeric_attribute("rating", item)?;
    let text = required_item_value("text", item)?;

    Ok(Review { course_id, category, rating, text, })
}

pub fn item_to_question(item: &HashMap<String, AttributeValue>) -> Result<Question, DDBError> {
    let course_id = required_item_value("course_id", item)?;
    let category = required_item_value("category", item)?;
    let text = required_item_value("text", item)?;
    let date = required_item_value("date", item)?;

    Ok(Question { course_id, category, text, date,})
}


pub fn item_to_answer(item: &HashMap<String, AttributeValue>) -> Result<Answer, DDBError> {
    let course_id = required_item_value("course_id", item)?;
    let category = required_item_value("category", item)?;
    let question_id = format!("QUESTION#{}", category.split('#').nth(2).unwrap_or(""));
    let text = required_item_value("text", item)?;
    let date = required_item_value("date", item)?;

    Ok(Answer { course_id, category, question_id, text, date,})
}