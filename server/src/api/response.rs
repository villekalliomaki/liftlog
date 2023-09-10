use serde::Serialize;

// Complete top-level API response format.
//
// D is the response data type. It's not an Option, to differentate
// endpoints where data is always or sometimes returned.
#[derive(Serialize)]
pub struct Reponse<D>
where
    D: Serialize,
{
    // Data queried
    data: D,
    // Zero or more errors
    errors: Vec<ResponseError>,
}

// Single error as a response to an API request.
//
// Could be a single field or whole unauthorized request
#[derive(Serialize)]
pub struct ResponseError {
    // Human readable error message
    msg: String,
    // If error is about a field, the name is here
    fields: Option<String>,
}
