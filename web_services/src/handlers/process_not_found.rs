use crate::http::http_response::HttpResponse;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn process_not_found(stream: &mut TcpStream) {
    let path = "src/resources/404.html";
    let mut file = match File::open(path) {
        Ok(f) => {f},
        Err(e) => {
            println!("Error load resources {}", path);
            println!("Cause: {}", e);
            return;
        },
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response: HttpResponse = HttpResponse::new(
        "HTTP/1.1".to_string(),
        "404".to_string(),
        "NOT FOUND".to_string(),
        contents,
    );
    response.send_response(stream);
}
