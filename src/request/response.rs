use super::http::*;
use std::str::FromStr;
use std::string::ToString;


#[derive(Debug, PartialEq, Eq)]
pub struct ParseStatusCodeError;

#[derive(Debug, PartialEq, Eq)]
enum ResponseStatus {
    StatusOk(u16),
    ClientError(u16),
    ServerError(u16)
}

impl FromStr for ResponseStatus {
    type Err = ParseStatusCodeError;
    fn from_str(s: &str) -> Result<ResponseStatus, Self::Err> {
        match s {
            "200" => Ok(ResponseStatus::StatusOk(200)),
            "201" => Ok(ResponseStatus::StatusOk(201)),
            "400" => Ok(ResponseStatus::ClientError(400)),
            "401" => Ok(ResponseStatus::ClientError(401)),
            "403" => Ok(ResponseStatus::ClientError(403)),
            "404" => Ok(ResponseStatus::ClientError(404)),
            "500" => Ok(ResponseStatus::ServerError(500)),
            _ => Err(ParseStatusCodeError)
        }
    }
}

impl ToString for ResponseStatus {
    fn to_string(&self) -> String {
        match &self {
            ResponseStatus::StatusOk(code) => {
                match code {
                    200 => "Ok".to_string(),
                    201 => "Created".to_string(),
                    _ => "Error".to_string()
                }
            },
            ResponseStatus::ClientError(code) => {
                match code {
                    400 => "Bad request".to_string(),
                    401 => "Unauthorized".to_string(),
                    403 => "Forbidden".to_string(),
                    404 => "Not Found".to_string(),
                    _ => "Error".to_string()
                }
            },
            ResponseStatus::ServerError(code) => {
                match code {
                    500 => "Internal Server Error".to_string(),
                    _ => "Error".to_string()
                }
            },
            _ => "Error".to_string()
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Response {
    status: String,
    headers: Vec<HttpHeader>,
    message: Option<String>
}

impl Response {
    pub fn new(code: String, headers: Vec<HttpHeader>, message: Option<String>) -> Self {
        let status = ResponseStatus::from_str(&code.as_str()).unwrap_or(ResponseStatus::ServerError(500));
        let status_message = status.to_string();
        Response {
            status: format!("{protocol} {status_code} {status_message}", protocol="HTTP/1.1", status_code=code, status_message=status_message),
            headers,
            message
        }
    }
}