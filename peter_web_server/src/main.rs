use std::{net::{TcpListener, TcpStream}, 
           io::{Read, Write, prelude::*},
           fs};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:5659").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_stream(stream);
    }

    fn handle_stream(mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        let get = b"GET / HTTP/1.1\r\n";
        let (status_line, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK\r\n\r\n", "peter.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
        };
        let contents = fs::read_to_string(filename).unwrap();
        let response = format!("{}{}", status_line, contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    
}
