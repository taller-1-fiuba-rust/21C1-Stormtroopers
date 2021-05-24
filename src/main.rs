use std::env::args;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

use crate::command_builder::CommandBuilder;
use crate::config_server::ConfigServer;
#[allow(unused_imports)]
use crate::logger::Logger;
//use crate::threadpool::ThreadPool;
use std::io::prelude::*;

mod command_builder;
mod config_server;
mod logger;
mod threadpool;

static THREAD_POOL_COUNT: usize = 4;

static END_FLAG: &str = "EOF";

//pub static INFO_LOAD_CONFIG_FILE: &str = "Load file config ...";

const LOG_NAME: &str = "log";
const LOG_PATH: &str = "./";

//use std::cell::RefCell;
//use std::thread;

struct Server {
    config_server: ConfigServer,
    logger: Logger<String>,
    command_builder: CommandBuilder,
}

impl Clone for Server {
    fn clone(&self) -> Server {
        let config_server = self.config_server.clone();
        let logger = self.logger.clone();
        let command_builder = self.command_builder.clone();

        Self {
            config_server,
            logger,
            command_builder,
        }
    }
}

impl Server {
    pub fn new() -> Self {
        let config_server = ConfigServer::new();
        let logger =
            Logger::new(LOG_NAME.to_string(), LOG_PATH.to_string()).expect("ERROR CREATING LOGGER");
        let command_builder = CommandBuilder::new();
        Self {
            config_server,
            logger,
            command_builder,
        }
    }

    pub fn get_logger(&self) -> Logger<String> {
        self.logger.clone()
    }
}

fn main() -> Result<(), std::io::Error> {
    let argv = args().collect::<Vec<String>>();
    let mut server = Server::new();
    match argv.len() {
        2 => {
            println!("Load file config ...");
            server
                .config_server
                .load_config_server_with_path(argv[1].as_str(), server.get_logger())?;
        }
        1 => {
            println!("Load file config server default ...");
            server
                .config_server
                .load_config_server(server.get_logger())?;
        }
        _ => {
            println!("Error count args");
        }
    }
    let app_name = &argv[0];
    println!("Serger args: {:?}", &argv);
    println!("Server {} is up!", app_name);

    let port = server.config_server.get_prop("port", server.get_logger());

    let mut path = String::from(server.config_server.get_prop("server", server.get_logger()));
    path.push_str(":");
    path.push_str(&port);

    println!();
    println!("Server address: {}", &path);
    println!();

    println!("Execute listener ...");

    let _listener = exec_server(&path, &server);

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
