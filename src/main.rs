use crate::handle_connection::executor::exec_server;
use crate::server::app_info::AppInfo;
use crate::server::connection::Connection;
use crate::server::connection_resolver::ConnectionResolver;
use std::env::args;

mod command;
mod constants;
mod data_base;
mod errors;
mod handle_connection;
mod server;

fn main() {
    let argv = args().collect::<Vec<String>>();

    println!("Init Server ...");

    let mut server = match AppInfo::new(argv) {
        Ok(s) => s,
        Err(e) => panic!("{}", e.message),
    };

    //deprecated
    //server.load_config(argv)?;

    println!("Server {} is up!", server.server_name());

    let server_port = server.get_server_port();

    println!("\nServer address: {}\n", server_port);
    println!("Execute listener ...");
    let _listener = exec_server(&server_port, &mut server);
}
