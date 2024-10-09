use std::{
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
    let file = fs::read_to_string(format!("{ROOT_DIR}/index.html")).unwrap();
    let length = file.len();


    let response = format!("{status}{CRLF}Content-Length: {length}{CRLF}{CRLF}{file}");

    match stream.write_all(response.as_bytes()) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Error occured during writing data: {:?}", err);
        }
    }
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
