use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use web_services::ThreadPool;
use std::str;
use std::time::{Duration, SystemTime};

// CONSTANTS
const HTTP_PORT: &str = "127.0.0.1:8081";
const REDIS_PORT: &str = "127.0.0.1:8082";
//const HTTP_GET: &[u8; 16] = b"GET / HTTP/1.1\r\n";
const HTTP_GET_INDEX: &[u8; 21] = b"GET /index HTTP/1.1\r\n";
const HTTP_POST_REDIS: &[u8; 22] = b"POST /redis HTTP/1.1\r\n";
//const HTTP_POST_CONFIG: &[u8; 23] = b"POST /config HTTP/1.1\r\n";

fn main() {

    let listener = TcpListener::bind(HTTP_PORT).unwrap();
    let _pool = ThreadPool::new(4);
    println!("Init web server. Server host:port: {}", HTTP_PORT);

    let mut stream_redis = TcpStream::connect(REDIS_PORT).unwrap();
    println!("Connection Redis server[{}]...",REDIS_PORT);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Execute listener!");

        handle_connection(&mut stream, &mut stream_redis);

        /*
        pool.execute(|| {
            handle_connection(stream);
        });
         */
    }
}

fn handle_connection(stream: &mut TcpStream, stream_redis: &mut TcpStream) {
    let time = SystemTime::now();

    let _stream_reader = stream.try_clone().expect("Cannot clone stream reader");

    /* Parse request */
    let mut buffer = [0; 2048];
    let _len = stream.read(&mut buffer).unwrap();

    let req = String::from_utf8_lossy(&buffer);

    let req_split = req.split('\n').collect::<Vec<&str>>();

    let mut cmd = "".to_string();
    for line in req_split {
        if line.contains("command="){
            cmd = line.split_at(8).1.to_string().replace("\x00","");
        }
    }
    println!("Command received: {}", cmd);

    if buffer.starts_with(HTTP_GET_INDEX) {
        process_get_index(stream);
    } else if buffer.starts_with(HTTP_POST_REDIS) {
        process_redis(stream, stream_redis, cmd);
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let mut file = File::open("src/resources/404.html").unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    let new_time = SystemTime::now();

    let difference = new_time.duration_since(time)
        .expect("Clock may have gone backwards");
    println!();
    println!("Time duration {:?}",difference);
}

fn process_redis(stream: &mut TcpStream, stream_redis: &mut TcpStream, msg_redis: String) {
    println!("Execute Redis command..");

    let mut msg = msg_redis;
    msg.push('\n');
    stream_redis.write_all(msg.as_bytes()).unwrap();
    stream_redis.flush().unwrap();
    let max_read = 4;
    let _len = 0;
    let _eof = false;

    let mut buffer = "".to_string();
    let _buffer2 = vec![0;max_read];
    let mut _res = "".to_string();
    stream_redis.set_read_timeout(Some(Duration::from_millis(500))).unwrap();
    loop {
        match stream_redis.read_to_string(&mut buffer) {
            Ok(_n) => { println!("Reding: {} -> {}", _len, buffer); },
            _ => { println!("Nothing to read in Redis stream");
                   break;
                 }
        }
    }

    _res = buffer;

    println!("Response Redis: {}", _res);
    let res_clean = _res.replace("127.0.0.1:8082>","");
    println!("Response Redis clean: {}", res_clean);

    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",res_clean.len(), res_clean);//String::from_utf8_lossy(&buffer)
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

fn process_get_index(stream: &mut TcpStream) {
    let mut file = File::open("src/resources/index.html").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",contents.len(), contents);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}