use super::http::*;
use crate::utils::*;

#[derive(Debug)]
pub struct Request {
    protocol: String,
    method: HttpMethods,
    path: String,
    headers: Vec<HttpHeader>
}

impl Request {
    pub fn from(req: &String, headers: Vec<HttpHeader>) -> Self {
        
        let mut method: HttpMethods = HttpMethods::GET;
        let mut protocol: String = String::from("HTTP/1.1");
        let mut path: String = String::from("/"); 
        
        let mut parts = req.split_whitespace();
        if let Some(val) = parts.next() {
            match val.to_string().parse::<HttpMethods>() {
                Ok(met) =>  {
                    method = met;
                },
                Err(err) => {
                    eprintln!("{:?}", err);
                }
            }
        }
        
        if let Some(val) = parts.next() {
            path = val.to_string();
        }
        
        if let Some(val) = parts.next() {
            protocol = val.to_string();
        }
        return Request {
            protocol,
            method,
            path,
            headers
        }
    }
    pub fn get_path(&self) -> String {
        return self.path.clone();

    }
    pub fn get_method(&self) -> HttpMethods {
        return self.method.clone();

    }
    pub fn get_protocol(&self) -> String {
        return self.protocol.clone();
    } 
    pub fn fetch_headers(headers: &Vec<String>) -> Vec<HttpHeader> {

        headers.iter().filter_map(|header| {
            let mut parts = header.splitn(2, ':');
            let name = parts.next()?.trim().to_lowercase();
            let value = parts.next()?.trim().to_string();
    
            let kind = if REQUEST_HEADERS.contains(name.as_str()) {
                HttpHeaderKind::Request
            } else if GENERAL_HEADERS.contains(name.as_str()) {
                HttpHeaderKind::General
            } else if ENTITY_HEADERS.contains(name.as_str()) {
                HttpHeaderKind::Entity
            } else {
                HttpHeaderKind::Custom
            };
    
            Some(HttpHeader::new(kind, name, value))
        }).collect()
        
    }
}