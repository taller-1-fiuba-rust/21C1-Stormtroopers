use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn process_get_index(stream: &mut TcpStream) {
    let mut file = File::open("src/resources/index.html").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
