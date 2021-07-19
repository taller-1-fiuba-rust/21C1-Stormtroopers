use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use web_services::ThreadPool;
use std::str;
use std::thread;
use rand::prelude::*;
use std::time::{Duration, SystemTime};
use web_services::Request;
use std::io::{BufRead, BufReader};

// CONSTANTS
const HTTP_PORT: &str = "127.0.0.1:8081";
const REDIS_PORT: &str = "127.0.0.1:8082";
const HTTP_GET: &[u8; 16] = b"GET / HTTP/1.1\r\n";
const HTTP_GET_INDEX: &[u8; 21] = b"GET /index HTTP/1.1\r\n";
const HTTP_POST_REDIS: &[u8; 22] = b"POST /redis HTTP/1.1\r\n";
const HTTP_POST_CONFIG: &[u8; 23] = b"POST /config HTTP/1.1\r\n";

fn main() {

    let listener = TcpListener::bind(HTTP_PORT).unwrap();
    let pool = ThreadPool::new(4);
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

    let mut stream_reader = stream.try_clone().expect("Cannot clone stream reader");
    //let msg = process_message(&mut stream);

    //Parse request


    println!("Request out");
    let mut buffer = [0; 2048];
    let len = stream.read(&mut buffer).unwrap();
    //buffer = bytes.Trim(buffer, "\x00");
    //std::replace( vecBuffer.begin(), vecBuffer.end(), '\x00', "");
    let req = String::from_utf8_lossy(&buffer);
    println!("Request Buffer: {}",req);
    let req_split = req.split('\n').collect::<Vec<&str>>();
//    println!("line0: {}",req_split[0]);
    let mut cmd = "".to_string();
    for line in req_split {
        if line.contains("command="){
            cmd = line.split_at(8).1.to_string().replace("\x00","");
        }
    }
    println!("CMD: {}", cmd);

    //println!("request: {}", req);
    if buffer.starts_with(HTTP_GET_INDEX) {
        process_get_index(stream);
    } else if buffer.starts_with(HTTP_POST_REDIS) {
        println!("in redis");
        /*
       for line in reader.lines() {

           println!("Request line: {}", line.unwrap());

           if line.to_string().contains("message=") {
               //msg = line.clone().split('=').collect::<Vec<&str>>();
               //println!("Message found {} -> {}",msg[0],msg[1]);
               return line

           }

            break;
        } */
        process_redis(stream, stream_redis, cmd);
    } else if buffer.starts_with(HTTP_POST_CONFIG) {
        process_config(stream);
    } else if buffer.starts_with(HTTP_GET) {
        let mut rng = thread_rng();
        let rand = rng.gen_range(0..5);
        println!("Random number: {}",rand);

        thread::sleep(Duration::from_secs(rand));

        let mut file = File::open("src/resources/index.html").unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
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

fn process_message(stream: &mut TcpStream) -> String {
    let stream_reader = stream.try_clone().expect("Cannot clone stream reader");
    let reader = BufReader::new(stream);

    //let splited_request = req.split('\n');
    //let mut msg = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        println!("Request line: {}", line);
        /*
        if line.to_string().contains("message=") {
            //msg = line.clone().split('=').collect::<Vec<&str>>();
            //println!("Message found {} -> {}",msg[0],msg[1]);
            return line

        }
         */
    }
    println!("OUT");
    "ping".to_string()
}
fn process_redis(stream: &mut TcpStream, stream_redis: &mut TcpStream, msg_redis: String) {
    println!();
    println!("ME CONECTO A REDIS!!!");
    println!("Sending ... {:?}", msg_redis);

/*
    let host_port = "127.0.0.1:8082";
    let mut stream_out = TcpStream::connect(host_port).unwrap();
    println!("Connection Redis server[{}]...",host_port);
 */
    let mut msg = msg_redis.to_string();
    msg.push('\n');
    stream_redis.write_all(msg.as_bytes());
    //let mut buffer = [0;64];
    //stream2.read(&mut buffer);
    let stream_reader = stream_redis.try_clone().expect("Cannot clone stream reader");
    let reader = BufReader::new(stream_reader);
    let lines = reader.lines();
    let mut res = "".to_string();

    /*
    let mut buffer = [0; 2048];
    let len = stream_redis.read(&mut buffer).unwrap();
    res = String::from_utf8_lossy(&buffer);
    println!("Response Redis Buffer: {}",res);
     */


    for line in lines {
        if let Ok(line_ok) = line {
            res += &line_ok;
            break;
        }
    }
    println!("Response Redis: {}", res);


    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",res.len(), res);//String::from_utf8_lossy(&buffer)
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    /*
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        handle_connection_redis(&mut stream);
    }
     */
    /*let mut contents = "OK".to_string();

    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",contents.len(), contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    */
}

fn handle_connection_redis(stream_redis: &mut TcpStream){
    stream_redis.write("PING".as_bytes()).unwrap();
    stream_redis.flush().unwrap();
}

fn process_get_index(stream: &mut TcpStream) {
    let mut file = File::open("src/resources/index.html").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",contents.len(), contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn process_config(stream: &mut TcpStream){
    let mut buffer = [0;8];
    let mut tot = String::from("");
    println!("Reading contents ");
    let mut request = Request::new();
    //loop {

        while let Ok(bytes_read) = stream.read(&mut buffer) {
            if bytes_read < buffer.len() { process(&buffer, bytes_read, &mut tot, &mut request); break; }
            process(&buffer, bytes_read, &mut tot, &mut request);
        }
      //  break;
    //}
    print!("Request -> {}",request.body.1);
}

fn process(buffer: &[u8], len: usize, buffer_tot: &mut String, _req: &mut Request){
    let partial = String::from_utf8_lossy(&buffer[..len]);
    let mut pstr = String::from("");
    pstr.push_str(&partial);
    let _pos = 0;

    if pstr.contains('\n'){
        let splited: Vec<&str> = pstr.split("\r\n").collect();
        let _lens = splited.len();
        print!("{}",buffer_tot);
        buffer_tot.clear();
        for line_spl in splited {
            print!("{}",line_spl);
            }
    } else {
        buffer_tot.push_str(&partial);
    }
}