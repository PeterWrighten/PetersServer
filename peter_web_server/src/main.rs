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

        let contents = fs::read_to_string("peter.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    
}
