use crate::http::http_request::{HttpRequest, Method};
use crate::http_server::process_get_index::process_get_index;
use crate::http_server::process_not_found::process_not_found;
use crate::http_server::process_redis::process_redis;
use std::net::TcpStream;

pub struct Router;

impl Router {
    pub fn route(
        req: HttpRequest,
        stream: &mut TcpStream,
        stream_redis: &mut TcpStream,
        cmd: String,
        host_port_redis: &str,
    ) {
        match req.method {
            Method::Get => {
                process_get_index(stream);
            }
            Method::Post => {
                process_redis(stream, stream_redis, cmd, host_port_redis);
            }
            _ => {
                process_not_found(stream);
            }
        };
    }
}
