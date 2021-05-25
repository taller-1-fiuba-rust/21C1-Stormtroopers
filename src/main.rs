use std::env::args;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

use crate::command_builder::CommandBuilder;
use std::io::prelude::*;
#[allow(unused_imports)]
use crate::logger::Logger;
use crate::server::Server;

mod command_builder;
mod config_server;
mod logger;
mod threadpool;
mod server;

static THREAD_POOL_COUNT: usize = 4;

static END_FLAG: &str = "EOF";

const LOG_NAME: &str = "log";
const LOG_PATH: &str = "./";
const ERROR_LOG_CREATE: &str = "Error creating Logger";

fn main() -> Result<(), std::io::Error> {
    let argv = args().collect::<Vec<String>>();
    let mut server = Server::new(argv.clone());

    server.load_config(argv)?;

    //println!("Serger args: {:?}", &argv);
    println!("Server {} is up!", server.server_name());

    let config_server = server.get_config_server();

    let server_port = config_server.get_server_port(server.get_logger());

    println!();
    println!("Server address: {}", server_port);
    println!();

    println!("Execute listener ...");

    let _listener = exec_server(&server_port, &server);

    Ok(())
}

fn exec_server(address: &String, server: &Server) -> Result<(), std::io::Error> {
    let threadpool = threadpool::ThreadPool::new(THREAD_POOL_COUNT.clone());
    let listener = TcpListener::bind(&address)?;
    for stream in listener.incoming() {
        let stream = stream;
        println!("Handler stream request ...");
        let server = server.clone();
        let stream = stream?;
        threadpool.execute(move || {
            //let stream = stream.unwrap();
            handle_connection(stream, &server);
        });
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream, server: &Server) {
    handle_client(&mut stream, server);
}

fn handle_client(stream: &mut TcpStream, server: &Server) {
    let stream_reader = stream.try_clone().expect("Cannot clone stream reader");
    let reader = BufReader::new(stream_reader);

    let mut lines = reader.lines();
    println!("Reading stream conections ...");

    while let Some(line) = lines.next() {
        let request = line.unwrap_or(String::from(END_FLAG));

        if request == END_FLAG {
            return;
        }
        println!("Server receive: {:?}", request);

        let response = process_request(request, server);
        (*stream).write(response.as_bytes()).unwrap_or(0);
    }
    println!("End handle client");
}

fn process_request(request: String, server: &Server) -> String {
    let mut command_builder = CommandBuilder::new();
    let comm = command_builder.get_command(&mut String::from(request.trim()));
    String::from(comm.str_response(server.get_logger()))
}