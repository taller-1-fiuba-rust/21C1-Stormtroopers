use crate::handlers::command_filter::filter_cmd;
use crate::handlers::process_get_index::process_generate_response;
use core::time::Duration;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn process_redis(
    stream: &mut TcpStream,
    stream_redis: &mut TcpStream,
    msg_redis: String,
    host_port_redis: &str,
) {
    println!("Execute Redis command..");
    let mut msg = filter_cmd(msg_redis);

    msg.push('\n');
    stream_redis.write_all(msg.as_bytes()).unwrap();
    stream_redis.flush().unwrap();
    let max_read = 4;
    let _len = 0;

    let mut buffer = "".to_string();
    let _buffer2 = vec![0; max_read];
    let mut _res = "".to_string();
    stream_redis
        .set_read_timeout(Some(Duration::from_millis(500)))
        .unwrap();
    loop {
        match stream_redis.read_to_string(&mut buffer) {
            Ok(_n) => {
                println!("Reding: {} -> {}", _len, buffer);
            }
            _ => {
                println!("Nothing to read in Redis stream");
                break;
            }
        }
    }

    _res = buffer;

    println!("Response Redis: {}", _res);
    let mut replace_host_redis = host_port_redis.to_string();
    replace_host_redis.push('>');
    let res_clean = _res.replace(replace_host_redis.as_str(), "");
    println!("Response Redis clean: {}", res_clean);

    /*let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        res_clean.len(),
        res_clean
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();*/

    process_generate_response(stream, res_clean);
}
