use axum::http::StatusCode;
use serde::Serialize;
use tracing::{debug, error, instrument, warn};
use std::fmt::Debug;

// Reponse to a successful API request
// Status 200 by default
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct RouteSuccess<D> where D: Serialize + Debug {
    // Human readable message
    msg: String,
    // Data being returned
    data: D,
    // HTTP status
    #[serde(skip_serializing)]
    status: StatusCode,
}

impl<D> RouteSuccess<D> where D: Serialize + Debug {
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
    pub fn new(msg: impl ToString + Debug, field: Option<impl ToString + Debug>, status: StatusCode) -> Self {
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

        let response = RouteError::new(msg, field, status);

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

        let axum_response: axum::response::Response = response.into();

        assert_eq!(status, axum_response.status());
    }

    // RouteError should be able to be converted into an axum HTTP response
    #[test]
    fn error_to_axum_response() {
        let msg = "test";
        let field = None;
        let status = StatusCode::INTERNAL_SERVER_ERROR;

        let response = RouteError::new(msg, field, status);

        let axum_response: axum::response::Response = response.into();

        assert_eq!(status, axum_response.status());
    }

    // Status code should not be serialized
    #[tokio::test]
    async fn success_body_status_serialization() {
        let response = RouteSuccess::<i32>::new("test", 1, StatusCode::OK);

        let axum_response: axum::response::Response = response.into();

        // Check that status code is not in serialized JSON
        let serialized_body = axum_response.into_body().data().await.unwrap().unwrap();
        let body_as_string = String::from(serialized_body);
        let as_json: serde_json::Value = serde_json::from_str(&body_as_string).unwrap().into();

        assert!(as_json.get("status").is_none())
    }
}
