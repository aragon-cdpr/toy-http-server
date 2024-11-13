use std::{
    io,
    fs, 
    io::{
        prelude::*, 
        BufReader
    }, net::{
        TcpListener, 
        TcpStream
    }    
};
mod utils;
mod request;
use utils::consts::*;
use request::*;
use response::*;

fn handle_connetion(stream: &mut TcpStream) -> std::io::Result<()> {
    let buf_reader = BufReader::new(&mut *stream);
    let mut request: Vec<_> = buf_reader
        .lines()
        .map(|result| { result.unwrap()})
        .take_while(|line| { !line.is_empty()})
        .collect();
    let headers = request.drain(1..).collect();
    let headers = Request::fetch_headers(&headers);
    let req = Request::from(&request[0], headers);
    let mut status_code: String = "".to_string();

    let mut file = fs::read_to_string(format!("{ROOT_DIR}/404.html")).unwrap();

    if let Ok(val) = fs::read_to_string(format!("{ROOT_DIR}/{req_path}/index.html", req_path = req.get_path())) {
        file = val;
        status_code = "200".to_string();
    } else if let Ok(val) = fs::read_to_string(format!("{ROOT_DIR}/{req_path}.html", req_path = req.get_path())){
        file = val;
        status_code = "200".to_string();
    } else {
        println!("not found: 404");
        status_code = "404".to_string();
    }

    let length = file.len();

    let response = Response::new(
        status_code, 
vec![
            HttpHeader::new(HttpHeaderKind::Response, "Content-Length".to_string(), length.to_string()),
            HttpHeader::new(HttpHeaderKind::Response, "X-Custom-Header".to_string(), "test".to_string())
        ], 
        Some(file.clone())
    );

    let response = response.to_string();

    stream.write_all(response.as_bytes())
}

fn main() {
    let addr = format!("{IP_ADDRESS}:{PORT}");
    let listener = TcpListener::bind(addr);

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
