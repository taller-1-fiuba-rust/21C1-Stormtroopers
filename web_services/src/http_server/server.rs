use crate::http::http_request::HttpRequest;
use crate::http_server::router::Router;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use crate::ThreadPool;

pub static THREAD_POOL_COUNT: usize = 200;

pub struct Server {
    socket_addr: String,
    redis_addr: String,
}

impl Server {
    pub fn new(socket_addr: String, redis_addr: String) -> Self {
        Server {
            socket_addr,
            redis_addr,
        }
    }
}
    pub fn run(socket_addr: String, redis_addr: String) {
        let thread_pool = ThreadPool::new(THREAD_POOL_COUNT);

        let listener = TcpListener::bind(socket_addr.clone()).unwrap();

        println!("Init web server. Server hosting in: {}", socket_addr.clone());

        let mut stream_redis = TcpStream::connect(redis_addr.clone()).unwrap();
        println!("Connection Redis server[{}]...", redis_addr.clone());
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Execute listener!");

            let mut buffer = [0; 2048];
            let _len = stream.read(&mut buffer).unwrap();

            let req_string: String = String::from_utf8_lossy(&buffer).to_string();
            let req: HttpRequest = String::from_utf8(buffer.to_vec()).unwrap().into();


            let mut stream2 = stream.try_clone().expect("Clone failed");
            let mut stream_redis2 = stream_redis.try_clone().expect("Clone failed");
            let redis_addr2 = redis_addr.clone();
            thread_pool.execute( move || {
                handle_connection(
                    &mut stream2,
                    &mut stream_redis2,
                    redis_addr2,
                    req,
                    req_string,
                )
            }

            )
        }
    }

fn handle_connection (
    stream: &mut TcpStream,
    stream_redis: &mut TcpStream,
    host_port_redis: String,
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

    Router::route(req, stream, stream_redis, cmd, host_port_redis.as_str());
}
