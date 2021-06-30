use crate::handle_connection::executor::exec_server;
use crate::server::app_info::AppInfo;
use crate::server::connection::Connection;
use crate::server::connection_resolver::ConnectionResolver;
use std::env::args;

mod command;
mod data_base;
mod errors;
mod handle_connection;
mod server;

static THREAD_POOL_COUNT: usize = 8;
static END_FLAG: &str = "exit";
static MSG_OVER: &str = "MESSAGE: Connection over\n";
static LINE_BREAK: char = '\n';
#[allow(dead_code)]
static RESP_SIMPLE_STRING: &str = "OK\r\n";
#[allow(dead_code)]
const LOG_NAME: &str = "log";
#[allow(dead_code)]
const LOG_PATH: &str = "./";
#[allow(dead_code)]
const ERROR_LOG_CREATE: &str = "Error creating Logger";

fn main() -> Result<(), std::io::Error> {
    let argv = args().collect::<Vec<String>>();
    let mut server = AppInfo::new(argv.clone());

    server.load_config(argv)?;

    println!("Server {} is up!", server.server_name());

    let server_port = server.get_server_port();

    println!("\nServer address: {}\n", server_port);
    println!("Execute listener ...");
    let _listener = exec_server(&server_port, &mut server);

    Ok(())
}
