use axum::{
    async_trait,
    extract::FromRequest,
    http::{Request, StatusCode},
    middleware::future::FromFnResponseFuture,
    Json, RequestExt,
};
use serde::Deserialize;
use validator::Validate;

use crate::api::response::RouteError;

pub struct ValidatedJson<J>(pub J);

impl<'de, S, J, B> FromRequest<S, J, B> for ValidatedJson<J>
where
    J: Validate + Deserialize<'de>,
    S: Send + Sync
{
    type Rejection = RouteError;

    fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();

        todo!()
    }
}
