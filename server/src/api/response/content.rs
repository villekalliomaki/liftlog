use serde::Serialize;
use std::fmt::Debug;

use super::response_error::ResponseError;

// Content is either data or an error
#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Content<D>
where
    D: Serialize + Debug,
{
    Data(D),
    Error(ResponseError),
}

// Ergonomic methods to create instances
impl<D: Serialize + Debug> Content<D> {
    pub fn data(data: D) -> Self {
        Content::Data(data)
    }

    pub fn error(error: impl Into<ResponseError>) -> Self {
        Content::Error(error.into())
    }
}
