use super::HTTPMethod;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: HTTPMethod,
}

impl Request {}