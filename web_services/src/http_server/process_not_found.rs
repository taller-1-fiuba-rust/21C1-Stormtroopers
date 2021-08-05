use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn process_not_found(stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let mut file = File::open("src/resources/404.html").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
