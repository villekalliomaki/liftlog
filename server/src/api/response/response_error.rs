use serde::Serialize;
use std::fmt::Debug;
use tracing::{debug, instrument};

// Single error as a response to an API request.
// Could be a single field or whole unauthorized request
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ResponseError {
    // Human readable error message
    msg: String,
    // If error is about a field, the name is here
    field_name: Option<String>,
}

impl ResponseError {
    // Creates a new ResponseError with no fields and the specified error message.
    #[instrument]
    pub fn msg(msg: impl ToString + Debug) -> Self {
        debug!("Creating new ResponseError");
        ResponseError {
            msg: msg.to_string(),
            field_name: None,
        }
    }

    // Automatically fills message field with a generic "invalid input" message
    // for the given field.
    #[instrument]
    pub fn field(field_name: impl ToString + Debug) -> Self {
        debug!("Creating new generic field error");
        ResponseError {
            msg: format!("Invalid input in field '{}'.", field_name.to_string()),
            field_name: Some(field_name.to_string()),
        }
    }
}

impl Into<ResponseError> for String {
    fn into(self) -> ResponseError {
        ResponseError {
            msg: self,
            field_name: None,
        }
    }
}

impl Into<ResponseError> for &str {
    fn into(self) -> ResponseError {
        ResponseError {
            msg: self.to_string(),
            field_name: None,
        }
    }
}
