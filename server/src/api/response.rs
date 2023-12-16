use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tracing::{debug, error, instrument, warn};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::{access_token::AccessToken, exercise::Exercise, session::Session, user::User, exercise_instance::ExerciseInstance};

// Reponse to a successful API request
// Status 200 by default
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
// All different generics appering in API documentation have
// to be defined manually because some utoipa limitation...
#[aliases(RouteSuccessUuid = RouteSuccess<Uuid>, RouteSuccessString = RouteSuccess<String>, RouteSuccessAccessToken = RouteSuccess<AccessToken>, RouteSuccessUser = RouteSuccess<User>, RouteSuccessExercise = RouteSuccess<Exercise>, RouteSuccessExerciseVec = RouteSuccess<Vec<Exercise>>, RouteSuccessSession = RouteSuccess<Session>, RouteSuccessSessionVec = RouteSuccess<Vec<Session>>, RouteSuccessExerciseInstance = RouteSuccess<ExerciseInstance>, RouteSuccessExerciseInstanceVec = RouteSuccess<Vec<ExerciseInstance>>)]
pub struct RouteSuccess<D>
where
    D: Serialize + Debug,
{
    // Human readable message
    #[schema(example = "Request was successful.")]
    pub msg: String,
    // Data being returned
    pub data: D,
    // HTTP status
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    status: StatusCode,
}

impl<D> RouteSuccess<D>
where
    D: Serialize + Debug,
{
    // Creates a new RouteSuccess
    #[instrument]
    pub fn new(msg: impl ToString + Debug, data: D, status: StatusCode) -> Self {
        debug!("Creating new RouteSuccess");
        RouteSuccess {
            msg: msg.to_string(),
            data,
            status,
        }
    }
}

impl<D: Serialize + Debug> IntoResponse for RouteSuccess<D> {
    fn into_response(self) -> axum::response::Response {
        debug!("Converting RouteSuccess to an axum Response");

        // Try to serialize body JSON
        let body = match serde_json::to_string(&self) {
            Ok(serialized_body) => serialized_body,
            Err(error) => {
                error!("Failed to serialize a response JSON body: {}", error);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to serialize a response JSON body",
                )
                    .into_response();
            }
        };

        (
            self.status,
            [(header::CONTENT_TYPE, "application/json")],
            body.into_bytes(),
        )
            .into_response()
    }
}

// Response to an API request which resulted in an error
// Status 400 by default
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub struct RouteError {
    // Should always be atleast one
    errors: Vec<SingleRouteError>,
    // HTTP status
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    status: StatusCode,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub struct SingleRouteError {
    // Human readable error message
    #[schema(example = "Invalid input in field_name.")]
    pub msg: String,
    // Field name, if error was caused by bad input
    #[schema(example = "field_name")]
    pub field: Option<String>,
}

impl RouteError {
    // Creates a new RouteError with one error
    #[instrument]
    pub fn new(
        msg: impl ToString + Debug,
        field: Option<impl ToString + Debug>,
        status: StatusCode,
    ) -> Self {
        debug!("Creating new RouteError with one error");
        RouteError {
            errors: vec![SingleRouteError {
                msg: msg.to_string(),
                field: match field {
                    Some(field) => Some(field.to_string()),
                    None => None,
                },
            }],
            status,
        }
    }

    // Add a new error the list of errors
    #[instrument]
    pub fn add(&mut self, msg: impl ToString + Debug, field: Option<impl ToString + Debug>) {
        debug!("Adding a new error to RouteError");

        self.errors.push(SingleRouteError {
            msg: msg.to_string(),
            field: match field {
                Some(field) => Some(field.to_string()),
                None => None,
            },
        });
    }

    // RouteError without any errors
    // Should only be used when adding n amount of them
    pub fn empty(status: StatusCode) -> Self {
        RouteError {
            errors: Vec::new(),
            status,
        }
    }
}

impl IntoResponse for RouteError {
    fn into_response(self) -> axum::response::Response {
        debug!("Converting RouteError to an axum Response");

        // Try to serialize body JSON
        let body = match serde_json::to_string(&self) {
            Ok(serialized_body) => serialized_body,
            Err(error) => {
                error!("Failed to serialize a response JSON body: {}", error);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to serialize a response JSON body",
                )
                    .into_response();
            }
        };

        (
            self.status,
            [(header::CONTENT_TYPE, "application/json")],
            body.into_bytes(),
        )
            .into_response()
    }
}

// Some conversions for common error types to RouteError
// for easy error handling

impl From<sqlx::error::Error> for RouteError {
    #[instrument]
    fn from(error: sqlx::error::Error) -> Self {
        debug!("Converting sqlx::error::Error to a RouteError");

        match error {
            // TODO: implement user-facing enum variants
            _ => {
                warn!("Encountered an unimplemented database error: {}. API will respond, but with a generic error.", error);

                RouteError::new(
                    "A database error occurred.",
                    None::<&str>,
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }
        }
    }
}

impl From<validator::ValidationErrors> for RouteError {
    #[instrument]
    fn from(error: validator::ValidationErrors) -> Self {
        debug!("Converting validator::ValidationErrors to a RouteError");

        // Get all field level errors
        let fields = error.field_errors();

        let mut response = RouteError::empty(StatusCode::BAD_REQUEST);

        // Generate single errors for all field validation errors
        for (key, errors) in fields {
            let mut formatted_messages = format!("Invalid input in {} field: ", key);

            if errors.len() > 1 {
                // Combile all validation errors for the single field if there are multiple
                for (i, error) in errors.iter().enumerate() {
                    match error.message.to_owned() {
                        Some(message) => {
                            if i == errors.len() - 1 {
                                // Last error
                                formatted_messages = formatted_messages + message.as_ref();
                            } else {
                                // Still other errors left
                                formatted_messages = formatted_messages + &format!("{}, ", message);
                            }
                        }
                        None => {
                            warn!("Encountered a validation error without an error message");

                            formatted_messages = formatted_messages + "invalid input";
                        }
                    }
                }
            } else if errors.len() == 1 {
                // Just one validation error
                match errors.get(0) {
                    Some(error) => match error.message.to_owned() {
                        Some(message) => {
                            formatted_messages = formatted_messages + message.as_ref();
                        }
                        None => {
                            warn!("Encountered a validation error without an error message");

                            formatted_messages = formatted_messages + "invalid input";
                        }
                    },
                    None => {
                        error!("Empty errors vec after checking, should be impossible");

                        return RouteError::new(
                            "Internal error, expected atleast one field error.",
                            None::<&str>,
                            StatusCode::INTERNAL_SERVER_ERROR,
                        );
                    }
                }
            } else {
                // No validation errors for some reason, should not happen
                error!("Validation failed, but didn't return any errors");

                return RouteError::new(
                    "Input validation failed, but didn't return any errors.",
                    None::<&str>,
                    StatusCode::INTERNAL_SERVER_ERROR,
                );
            }

            response.add(formatted_messages + ".", Some(key));
        }

        response
    }
}

// Shorthand for the complete return type for route handlers
//
// Most common errors should implement Into<RouteError>, which means
// they can easily be converted with the "?" syntax
pub type RouteResponse<D> = Result<RouteSuccess<D>, RouteError>;

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use super::*;

    // Create a new successful response
    #[test]
    fn success_new() {
        let msg = "test";
        let data = 1;
        let status = StatusCode::OK;

        let response = RouteSuccess::<i32>::new(msg, data, status);

        assert_eq!(response.status, status);
        assert_eq!(response.data, data);
        assert_eq!(response.msg, msg);
    }

    // Create a new error response
    #[test]
    fn error_new() {
        let msg = "test";
        let field = Some("name".to_string());
        let status = StatusCode::BAD_REQUEST;

        let response = RouteError::new(msg, field.clone(), status);

        assert_eq!(response.status, status);
        assert_eq!(response.errors.get(0).unwrap().field, field);
        assert_eq!(response.errors.get(0).unwrap().msg, msg);
    }

    // RouteSuccess should be able to be converted into an axum HTTP response
    #[test]
    fn success_to_axum_response() {
        let msg = "test";
        let data = 1;
        let status = StatusCode::OK;

        let response = RouteSuccess::<i32>::new(msg, data, status);

        let axum_response: axum::response::Response = response.into_response();

        assert_eq!(status, axum_response.status());
    }

    // RouteError should be able to be converted into an axum HTTP response
    #[test]
    fn error_to_axum_response() {
        let msg = "test";
        let field: Option<String> = None;
        let status = StatusCode::INTERNAL_SERVER_ERROR;

        let response = RouteError::new(msg, field, status);

        let axum_response: axum::response::Response = response.into_response();

        assert_eq!(status, axum_response.status());
    }
}
