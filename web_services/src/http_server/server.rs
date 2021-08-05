use crate::http::http_request::HttpRequest;
use crate::http_server::router::Router;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
    redis_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str, redis_addr: &'a str) -> Self {
        Server {
            socket_addr,
            redis_addr,
        }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(self.socket_addr).unwrap();

        println!("Init web server. Server hosting in: {}", self.socket_addr);

        let mut stream_redis = TcpStream::connect(self.redis_addr).unwrap();
        println!("Connection Redis server[{}]...", self.redis_addr);
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Execute listener!");

            let mut buffer = [0; 2048];
            let _len = stream.read(&mut buffer).unwrap();

            let req_string: String = String::from_utf8_lossy(&buffer).to_string();
            let req: HttpRequest = String::from_utf8(buffer.to_vec()).unwrap().into();

            handle_connection(
                &mut stream,
                &mut stream_redis,
                self.redis_addr,
                req,
                req_string,
            );
        }
    }
}

fn handle_connection(
    stream: &mut TcpStream,
    stream_redis: &mut TcpStream,
    host_port_redis: &str,
    req: HttpRequest,
    req_string: String,
) {
    let req_split = req_string.split('\n').collect::<Vec<&str>>();

    let mut cmd = "".to_string();
    for line in req_split {
        if line.contains("command=") {
            cmd = line.split_at(8).1.to_string().replace("\x00", "");
        }
    }
    println!("Command received: {}", cmd);

    Router::route(req, stream, stream_redis, cmd, host_port_redis);
}
