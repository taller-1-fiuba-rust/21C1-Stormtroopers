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
        .set_read_timeout(Some(Duration::from_millis(20)))
        .unwrap();
    loop {
        match stream_redis.read_to_string(&mut buffer) {
            Ok(_n) => {
                println!("Reading: {} -> {}", _len, buffer);
            }
            _ => {
                println!("Nothing to read in Redis stream");
                break;
            }
        }
    }

    _res = buffer;
    let mut replace_host_redis = host_port_redis.to_string();
    replace_host_redis.push_str(">");
    println!("Redis replace: {} ", replace_host_redis);
    println!("Redis buffer: {} ", _res);

    let res_splited: Vec<&str> = _res.split(host_port_redis).collect();

    for res in res_splited {
        println!("##########");
        println!("Response Redis clean: {}", res);
        println!("##########");
        process_generate_response(stream, res.to_string());
    }
    /*
    println!("Response Redis: {}", _res);
    let mut replace_host_redis = host_port_redis.to_string();
    replace_host_redis.push('>');
    let res_clean = _res.replace(replace_host_redis.as_str(), "");
    println!("Response Redis clean: {}", res_clean);

    process_generate_response(stream, res_clean);
     */


}
