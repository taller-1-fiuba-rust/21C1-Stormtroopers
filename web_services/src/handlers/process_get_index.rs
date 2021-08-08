use crate::http::http_response::HttpResponse;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn process_get_index(stream: &mut TcpStream) {
	let path = "src/resources/index.html";
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

    /*let response = HttpResponse::response_with_content(HttpResponse::default(), contents);
    response.send_response(stream);*/
    process_generate_response(stream, contents);
}

pub fn process_generate_response(stream: &mut TcpStream, contents: String) {
    let response = HttpResponse::response_with_content(HttpResponse::default(), contents);
    response.send_response(stream);
}
