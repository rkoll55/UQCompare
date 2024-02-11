use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Course {
    pub course_id: String,
    pub category: String,
    pub course_name: String,
    pub description: String,
    pub lecturer: String,
    pub prerequisites: Vec<String>,
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
             Prerequisites: {:?}",
            self.course_id, self.category, self.course_name, self.description, self.lecturer, self.prerequisites
        )
    }
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
