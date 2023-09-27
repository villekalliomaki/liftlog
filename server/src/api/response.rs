use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;
use std::fmt::Debug;
use tracing::{debug, error, instrument, warn};

// Reponse to a successful API request
// Status 200 by default
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct RouteSuccess<D>
where
    D: Serialize + Debug,
{
    // Human readable message
    msg: String,
    // Data being returned
    data: D,
    // HTTP status
    #[serde(skip_serializing)]
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
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct RouteError {
    // Human readable error message
    msg: String,
    // Field name, if error was caused by bad input
    field: Option<String>,
    // HTTP status
    #[serde(skip_serializing)]
    status: StatusCode,
}

impl RouteError {
    // Creates a new RouteError
    #[instrument]
    pub fn new(
        msg: impl ToString + Debug,
        field: Option<impl ToString + Debug>,
        status: StatusCode,
    ) -> Self {
        debug!("Creating new RouteError");
        RouteError {
            msg: msg.to_string(),
            field: match field {
                Some(from) => Some(from.to_string()),
                None => None,
            },
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

impl Into<RouteError> for sqlx::error::Error {
    #[instrument]
    fn into(self) -> RouteError {
        debug!("Converting sqlx::error::Error to a RouteError");

        match self {
            // TODO: implement user-facing enum variants
            _ => {
                warn!("Encountered an unimplemented database error: {}. API will respond, but with a generic error.", self);

                RouteError::new(
                    "A database error occurred.",
                    None::<&str>,
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }
        }
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
        assert_eq!(response.field, field);
        assert_eq!(response.msg, msg);
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
