use std::{
    fs,
    io::{prelude::*, BufReader}, 
    net::{TcpListener, TcpStream},
};
   
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream); 
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream); 
    let http_request: Vec<_> = buf_reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();

    let path = if let Some(first_line) = http_request.first() {
        let mut parts = first_line.split_whitespace();

        if let Some(path) = parts.nth(1) {
            path
        } else {
            "/"
        }
    } else {
        "/"
    };

    let (status_line, contents) = if path == "/" {
        ("HTTP/1.1 200 OK", fs::read_to_string("hello.html").unwrap())
    } else {
        ("HTTP/1.1 404 NOT FOUND", fs::read_to_string("bad.html").unwrap())
    };
    
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap();
}