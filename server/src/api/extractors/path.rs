use axum::{
    async_trait,
    extract::{path::ErrorKind, rejection::PathRejection, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use serde::de::DeserializeOwned;
use tracing::error;

use crate::api::response::RouteError;

// Custom Path extractor to  return errors in the correct format
pub struct Path<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for Path<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = RouteError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(error) => {
                Err(match error {
                    PathRejection::FailedToDeserializePathParams(inner) => {
                        let kind = inner.into_kind();

                        let status = StatusCode::BAD_REQUEST;

                        match &kind {
                            ErrorKind::WrongNumberOfParameters { .. } => {
                                RouteError::new(kind.to_string(), None::<&str>, status)
                            }
                            ErrorKind::ParseErrorAtKey { key, .. } => {
                                RouteError::new(kind.to_string(), Some(key), status)
                            }
                            ErrorKind::ParseErrorAtIndex { .. } => {
                                RouteError::new(kind.to_string(), None::<&str>, status)
                            }
                            ErrorKind::ParseError { .. } => {
                                RouteError::new(kind.to_string(), None::<&str>, status)
                            }
                            ErrorKind::InvalidUtf8InPathParam { key } => {
                                RouteError::new(kind.to_string(), Some(key), status)
                            }
                            ErrorKind::UnsupportedType { .. } => RouteError::new(
                                kind.to_string(),
                                None::<&str>,
                                StatusCode::INTERNAL_SERVER_ERROR,
                            ),
                            ErrorKind::Message(msg) => {
                                RouteError::new(msg.clone(), None::<&str>, status)
                            }
                            _ => {
                                error!("Unhandled path extractor deserialization error: {}", kind);

                                RouteError::new("Unhandled path extractor deserialization error. Check logs for details.", None::<&str>, StatusCode::INTERNAL_SERVER_ERROR)
                            }
                        }
                    }
                    PathRejection::MissingPathParams(error) => {
                        error!(
                            "Missing path parameters in internal representation: {}",
                            error
                        );

                        RouteError::new(
                            "Missing path parameter error. Check logs for details.",
                            None::<&str>,
                            StatusCode::INTERNAL_SERVER_ERROR,
                        )
                    }

                    _ => {
                        error!("Unhandled path extractor deserialization error: {}", error);

                        RouteError::new("Unhandled path extractor deserialization error. Check logs for details.", None::<&str>, StatusCode::INTERNAL_SERVER_ERROR)
                    }
                })
            }
        }
    }
}
