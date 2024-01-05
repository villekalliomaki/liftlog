use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
};
use tracing::error;
use validator::Validate;

use crate::api::response::RouteError;

// Wrapper to get custom error responses from invalid JSON input
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
    T: Validate,
{
    type Rejection = RouteError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();

        let req = Request::from_parts(parts, body);

        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => {
                value.0.validate()?;

                Ok(Self(value.0))
            }
            Err(rejection) => {
                return match rejection {
                    JsonRejection::JsonDataError(error) => Err(RouteError::new(
                        format!(
                            "Input JSON was valid but could not be serialized to input format: {}.",
                            error.body_text()
                        ),
                        None::<&str>,
                        error.status(),
                    )),
                    JsonRejection::JsonSyntaxError(error) => Err(RouteError::new(
                        format!("Syntax error in JSON input: {}.", error.body_text()),
                        None::<&str>,
                        error.status(),
                    )),
                    JsonRejection::MissingJsonContentType(_) => Err(RouteError::new(
                        "Content-Type header missing for JSON input.",
                        None::<&str>,
                        StatusCode::BAD_REQUEST,
                    )),
                    JsonRejection::BytesRejection(error) => Err(RouteError::new(
                        format!("Failed to extract bytes for JSON: {}.", error.body_text()),
                        None::<&str>,
                        error.status(),
                    )),
                    rejection => {
                        error!("Unimplemented JSON error encounrered: {}", rejection);

                        Err(RouteError::new(
                            "Unimplemented JSON error occurred. Check logs for details.",
                            None::<&str>,
                            StatusCode::BAD_REQUEST,
                        ))
                    }
                };
            }
        }
    }
}
