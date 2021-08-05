use crate::handlers::process_get_index::process_get_index;
use crate::handlers::process_not_found::process_not_found;
use crate::handlers::process_redis::process_redis;
use crate::http::http_request::{HttpRequest, Method};
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
