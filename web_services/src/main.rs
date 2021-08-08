use crate::help::help;
use crate::http_server::server::{Server, run};
use std::env::args;
use threadpool::ThreadPool;
mod handlers;
pub mod help;
mod http;
mod http_server;
mod threadpool;

fn main() {
    let argv = args().collect::<Vec<String>>();

    if argv.len() < 3 {
        help();
        return;
    }

    let host_port = &argv[1];
    let host_port_redis = &argv[2];

    let server = Server::new(host_port.to_string(), host_port_redis.to_string());
    run(server);
}
