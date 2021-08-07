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

    if argv.len() == 1 {
        help();
        return;
    }
    if argv.len() != 3 {
        println!("Error: Debe agregar el host y port del servicio web y del servidor Redis destino: <host>:<port> <host_redis>:<port_redis>");
        return;
    }
    let host_port = &argv[1];
    let host_port_redis = &argv[2];
    //hasta acá -> quizás puede ser el mismo server o un validator extra

    let _server = Server::new(host_port.to_string(), host_port_redis.to_string());
    run(host_port.to_string(), host_port_redis.to_string());
}
