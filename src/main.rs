use std::{
    fs, 
    io::{
        prelude::*, 
        BufReader
    }, net::{
        TcpListener, 
        TcpStream
    }, 
    str::FromStr
};

const CRLF: &str = "\r\n";
const PORT: &str = "6969";
const IP_ADDRESS: &str = "127.0.0.1";

#[derive(Debug, Clone)]
enum HttpMethods {
    POST,
    GET,
    PUT
}
#[derive(Debug, PartialEq, Eq)]
struct ParseHttpMethodError;

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

#[derive(Debug, Clone)]
enum HttpHeaderKind {
    Request,
    Response,
    Representation,
    Payload
}
#[derive(Debug, Clone)]
struct HttpHeader {
    kind: HttpHeaderKind,
    name: String,
    value: String
}


#[derive(Debug)]
struct Request {
    protocol: String,
    method: HttpMethods,
    path: String,
    headers: Vec<HttpHeader>
}

impl Request {
    pub fn from(req: String, headers: Vec<HttpHeader>) -> Self {
        
        let mut method: HttpMethods = HttpMethods::GET;
        let mut protocol: String = String::from("HTTP/1.1");
        let mut path: String = String::from("/"); 
        
        let mut parts = req.split_whitespace();
        if let Some(val) = parts.next() {
            match val.to_string().parse::<HttpMethods>() {
                Ok(i) =>  {
                    method = i;
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
        // let method = 
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
    pub fn fetch_headers(req: Vec<String>) -> Vec<HttpHeader> {
        vec![
            HttpHeader {
                kind: HttpHeaderKind::Request,
                name: String::from("Accept"), 
                value: String::from("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8"),

            }
        ]
    }
}

fn handle_connetion(stream: &mut TcpStream) {
    let buf_reader = BufReader::new(&mut *stream);
    let mut request: Vec<_> = buf_reader
        .lines()
        .map(|result| { result.unwrap()})
        .take_while(|line| { !line.is_empty()})
        .collect();
    let headers = request.drain(1..).collect();
    let headers = Request::fetch_headers(headers);
    let req = Request::from(request[0].clone(), headers);
    println!("{:#?}", req);
    let status = "HTTP/1.1 200 OK";
    let file = fs::read_to_string("index.html").unwrap();
    let length = file.len();


    let response = format!("{status}{CRLF}Content-Length: {length}{CRLF}{CRLF}{file}");

    match stream.write_all(response.as_bytes()) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Error occured during writing data: {:?}", err);
        }
    }
    // println!("Connection established! Stream: {:#?}", request);
}

fn main() {
    let addr = format!("{IP_ADDRESS}:{PORT}");
    let listener = TcpListener::bind(addr);

    println!("{}", CRLF.to_string());
    match listener {
        Ok(listener) => {
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        handle_connetion(&mut stream);
                    },
                    Err(err) => {
                        eprintln!("Error occured during connection: {:?}", err);
                    }
                }
            }
        },
        Err(err) => {
            eprintln!("Error occured during connection: {:?}", err);
        }
    }
}
