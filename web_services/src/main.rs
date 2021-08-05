use crate::http_server::server::Server;
use std::env::args;

mod http;
mod http_server;

fn main() {
    //delegarle esto a otra instancia
    let argv = args().collect::<Vec<String>>();
    if argv.len() != 3 {
        println!("Error: Debe agregar el host y port del servicio web y del servidor Redis destino: <host>:<port> <host_redis>:<port_redis>");
        return;
    }
    let host_port = &argv[1];
    let host_port_redis = &argv[2];
    //hasta acá -> quizás puede ser el mismo server o un validator extra

    let server = Server::new(host_port, host_port_redis);
    server.run();
}
