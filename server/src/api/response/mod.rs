mod content;
mod response_error;

use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;
use std::fmt::Debug;
use tracing::{debug, error, instrument, warn};

use self::{content::Content, response_error::ResponseError};

// Complete top-level API response format.
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Response<D>
where
    D: Serialize + Debug,
{
    // Data or an error
    // Serialized in their own fields
    #[serde(flatten)]
    content: Content<D>,
    // Status code for IntoResponse impl
    // Defaults to 200, if still None when converted into an axum Response
    #[serde(skip_serializing)]
    status: Option<StatusCode>,
}

// Useful methods to create new Responses with any kind of data type.
// Should ALWAYS be created with
impl<D: Serialize + Debug> Response<D> {
    // New response with data
    pub fn success(data: D) -> Self {
        Response {
            content: Content::data(data),
            status: Some(StatusCode::OK),
        }
    }

    // New Response with just an error
    pub fn error(error: impl Into<ResponseError>, status: StatusCode) -> Self {
        Response {
            content: Content::error(error),
            status: Some(status),
        }
    }
}

// When implementing IntoResponse, the Reponse instance can be returned directly from any routes.
impl<D: Serialize + Debug> IntoResponse for Response<D> {
    #[instrument]
    fn into_response(self) -> axum::response::Response {
        debug!("Serializing response to JSON body");

        // Try to serialize body JSON
        let body = match serde_json::to_string(&self) {
            Ok(serialized_body) => serialized_body,
            Err(error) => {
                error!("Failed to serialize a response JSON body: {}", error);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to serialize a response JSON body (check logs)",
                )
                    .into_response();
            }
        };

        let final_status = match self.status {
            Some(status) => {
                debug!("Overriding default HTTP status with {}", status);
                status
            }
            None => {
                debug!("Using default HTTP response status");
                StatusCode::OK
            }
        };

        (
            final_status,
            [(header::CONTENT_TYPE, "application/json")],
            body.into_bytes(),
        )
            .into_response()
    }
}

// Conversion implementations for common error types.
// Errors should be converted to give as less information as possible about the software,
// but still be helpful to a human reading them.
//
// Because a Reponse will impliment axum traits to be returned directly from a handler,
// common errors can be returned easily with the ? -syntax.
//
// The data type is wrapped in an Option, because there is no case where an error is converted
// to an Reposnse where data is returned.
impl<D: Serialize + Debug> From<argon2::Error> for Response<D> {
    fn from(_: argon2::Error) -> Self {
        Response::error(
            ResponseError::msg(
                "Password hash operation failed. More information in the server logs.",
            ),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }
}

// Should maybe not use this:
// - exposes internal errors
// - status code has no use
// - doesn't really tell what the user did wrong
impl<D: Serialize + Debug> From<anyhow::Error> for Response<D> {
    fn from(value: anyhow::Error) -> Self {
        Response::error(
            ResponseError::msg(format!("An error occurred: {}", value)),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }
}

impl<D: Serialize + Debug> From<sqlx::error::Error> for Response<D> {
    fn from(value: sqlx::error::Error) -> Self {
        match value {
            _ => {
                warn!(
                    "Encountered a sqlx error without a conversion for Response: {}",
                    value
                );
                Response::error(
                    "Unspecified database error occurred.",
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_test() -> anyhow::Result<()> {
        Ok(())
    }
}