use std::{
    fs, 
    io::{
        prelude::*, 
        BufReader
    }, net::{
        TcpListener, 
        TcpStream
    }, 
    str::FromStr,
    collections::{
        HashSet
    },
    sync::{
        LazyLock
    }
};

const CRLF: &str = "\r\n";
const PORT: &str = "6969";
const IP_ADDRESS: &str = "127.0.0.1";

static REQUEST_HEADERS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "accept", "accept-charset", "accept-encoding", "accept-language",
        "authorization", "expect", "from", "host", "if-match", "if-modified-since",
        "if-none-match", "if-range", "if-unmodified-since", "max-forwards",
        "proxy-authorization", "range", "referer", "te", "user-agent",
    ])
});

static GENERAL_HEADERS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "cache-control", "connection", "date", "pragma", "trailer", "transfer-encoding",
        "upgrade", "via", "warning",
    ])
});

static ENTITY_HEADERS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "allow", "content-encoding", "content-language", "content-length", "content-location",
        "content-md5", "content-range", "content-type", "expires", "last-modified",
    ])
});

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
    General,
    Response,
    Entity,
    Custom
}
#[derive(Debug, Clone)]
struct HttpHeader {
    kind: HttpHeaderKind,
    name: String,
    value: String
}
impl HttpHeader {
    pub fn new(kind: HttpHeaderKind, name: String, value: String) -> Self {
        HttpHeader {kind, name, value}
    }
}


#[derive(Debug)]
struct Request {
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

fn handle_connetion(stream: &mut TcpStream) {
    let buf_reader = BufReader::new(&mut *stream);
    let mut request: Vec<_> = buf_reader
        .lines()
        .map(|result| { result.unwrap()})
        .take_while(|line| { !line.is_empty()})
        .collect();
    let headers = request.drain(1..).collect();
    let headers = Request::fetch_headers(&headers);
    let req = Request::from(&request[0], headers);
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
