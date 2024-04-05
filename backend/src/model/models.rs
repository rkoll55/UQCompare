use serde::{Serialize, Deserialize};
use core::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Course {
    pub course_id: String,
    pub category: String,
    pub course_name: String,
    pub description: String,
    pub lecturer: String,
    pub average_rating: u8,
    pub average_difficulty: u8,
    pub prerequisites: Vec<String>,
    pub assesments: Vec<Assesments>,
}

impl fmt::Display for Course {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Course ID: {}\n\
             Category: {}\n\
             Course Name: {}\n\
             Description: {}\n\
             Lecturer: {}\n\
             Prerequisites: {:?}\n\
             Assesments: {:?}",
            self.course_id, self.category, self.course_name, self.description, self.lecturer, self.prerequisites, self.assesments
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Assesments {
    pub name: String,
    pub weight: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Review {
    pub course_id: String,
    pub category: String,
    pub rating: u8,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReviewRequest {
    pub course_id: String,
    pub rating: u8,
    pub text: String,
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
