use axum::http::StatusCode;
use serde::Serialize;

// Complete top-level API response format.
//
// D is the response data type. It's not an Option, to differentate
// endpoints where data is always or sometimes returned.
#[derive(Serialize)]
pub struct Response<D>
where
    D: Serialize,
{
    // Data queried
    data: D,
    // Zero or more errors
    errors: Vec<ResponseError>,
}

// Type for situations where a response is returned, but no data it returned with it.
// For example implimentations for From<SomeError> to a Response.
pub type EmptyResponse = Response<()>;

// Useful methods to create new Responses with any kind of data type.
impl<D: Serialize> Response<D> {
    // Data without any errors
    pub fn data(data: D) -> Self {
        Response {
            data,
            errors: vec![],
        }
    }

    // Data with one error
    pub fn data_with_error(data: D, error: ResponseError) -> Self {
        Response {
            data,
            errors: vec![error],
        }
    }

    // Data with multiple errors
    pub fn data_with_errors(data: D, errors: Vec<ResponseError>) -> Self {
        Response { data, errors }
    }

    // Add an error to an existing response
    pub fn add_error(&mut self, error: ResponseError) {
        self.errors.push(error);
    }

    // Create a new error with a message and add it to the response
    pub fn add_error_msg(&mut self, msg: String) {
        self.errors.append(&mut ResponseError::msg(msg));
    }
}

// Single error as a response to an API request.
//
// Could be a single field or whole unauthorized request
#[derive(Serialize)]
pub struct ResponseError {
    // Human readable error message
    msg: String,
    // If error is about a field, the name is here
    field_name: Option<String>,
}

impl ResponseError {
    // Creates a new ResponseError with no fields and the specified error message.
    pub fn msg(msg: String) -> Vec<Self> {
        vec![ResponseError {
            msg,
            field_name: None,
        }]
    }

    // Automatically fills message field with a generic "invalid input" message
    // for the given field.
    pub fn field(field_name: String) -> Vec<Self> {
        vec![ResponseError {
            msg: format!("Invalid input in field '{}'.", field_name),
            field_name: Some(field_name),
        }]
    }
}

// Here are conversion implementations for common error types.
// Errors should be converted to give as less information as possible about the software,
// but still be helpful to a human reading them.
//
// Because a Reponse will impliment axum traits to be returned directly from a handler,
// common errors can be returned easily with the ? -syntax.
impl From<argon2::Error> for EmptyResponse {
    fn from(value: argon2::Error) -> Self {
        Response {
            data: (),
            errors: ResponseError::msg(
                "Password hash operation failed. More information in the server logs.".into(),
            ),
        }
    }
}
