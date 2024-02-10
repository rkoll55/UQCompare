use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Course {
    pub course_id: String,
    pub category: String,
    pub course_name: String,
    pub description: String,
    pub lecturer: String,
    pub prerequisites: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Review {
    pub course_id: String,
    pub category: String,
    pub rating: u8,
    pub text: String,
    pub date: String,
}

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub course_id: String,
    pub category: String,
    pub text: String,
    pub date: String,
}

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub course_id: String,
    pub category: String,
    pub text: String,
    pub date: String,
}
