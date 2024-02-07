use serde::{Serialize, Deserialize};
use strum_macros::{Display, EnumString};
use uuid::Uuid;


#[derive(Serialize, EnumString, Display, Eq, PartialEq)]
pub enum TaskState {
    NotStarted,
    InProgress,
    Completed,
    Paused,
    Failed,
}

#[derive(Serialize)]
pub struct Course {
    course_name: String,
    course_code: String,
    description: String,
    lecturer: String,
    assesment: String,
}
