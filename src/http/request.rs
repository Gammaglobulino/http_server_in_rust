use super::method::*;

pub struct Request {
    path: String,
    query: Option<String>,
    method: HttpMethods,
}
