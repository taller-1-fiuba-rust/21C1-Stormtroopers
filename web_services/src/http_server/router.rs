use crate::handlers::process_get_index::process_get_index;
use crate::handlers::process_not_found::process_not_found;
use crate::handlers::process_redis::process_redis;
use crate::http::http_request::{HttpRequest, Method, Resource};
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
        println!("Route request method: {}", cmd);
        let _idx_resource = Resource::Path("/index".to_string());
        match req.method {
            Method::Get => {
                println!("GET");
                match req.resource {
                    Resource::Path(s) => {
                        if s == *"/index".to_string() {
                            process_get_index(stream);
                        } else {
                            process_not_found(stream);
                        };
                    },

                }
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
