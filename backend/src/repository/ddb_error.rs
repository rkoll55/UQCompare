use std::error::Error;
use std::fmt;

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