use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum HttpMethods {
    POST,
    GET,
    PUT
}
#[derive(Debug, PartialEq, Eq)]
pub struct ParseHttpMethodError;

impl FromStr for HttpMethods {
    type Err = ParseHttpMethodError;
    fn from_str(s: &str) -> Result<HttpMethods, Self::Err> {
        match s {
            "GET" => Ok(HttpMethods::GET),
            "POST" => Ok(HttpMethods::POST),
            "PUT" => Ok(HttpMethods::PUT),
            _ => Err(ParseHttpMethodError)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpHeaderKind {
    Request,
    General,
    Response,
    Entity,
    Custom
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpHeader {
    kind: HttpHeaderKind,
    name: String,
    value: String
}
impl HttpHeader {
    pub fn new(kind: HttpHeaderKind, name: String, value: String) -> Self {
        HttpHeader {kind, name, value}
    }
}
