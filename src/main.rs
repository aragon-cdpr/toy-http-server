use std::{fs, io::{prelude::*, BufReader}, net::TcpListener};



fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969");

    
    match listener {
        Ok(_) => {
            for stream in listener.expect("REASON").incoming() {
                match stream {
                    Ok(_) => {
                        let mut stream = stream.unwrap();
                        let buf_reader = BufReader::new(&mut stream);
                        let request: Vec<_> = buf_reader
                            .lines()
                            .map(|result| { result.unwrap()})
                            .take_while(|line| { !line.is_empty()})
                            .collect();
                        let status = "HTTP/1.1 200 OK";
                        let file = fs::read_to_string("index.html").unwrap();
                        let length = file.len();


                        let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{file}");
                        stream.write_all(response.as_bytes()).unwrap();
                        println!("Connection established! Stream: {:#?}", request);
                    },
                    Err(err) => {
                        eprintln!("Error occured during connection: {:?}", err);
                        break;
                    }
                }
            }
        },
        Err(err) => {
            eprintln!("Error occured during connection: {:?}", err);
        }
    }
}
