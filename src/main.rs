use std::env::args;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

use std::io::prelude::*;
use crate::threadpool::ThreadPool;
use crate::config_server::ConfigServer;
use crate::command_builder::CommandBuilder;
use crate::logger::Logger;

mod threadpool;
mod command_builder;
mod config_server;
mod logger;

static THREAD_POOL_COUNT: usize = 4;

static END_FLAG: &str = "EOF";

//pub static INFO_LOAD_CONFIG_FILE: &str = "Load file config ...";

const LOG_NAME: &str = "log";
const  LOG_PATH: &str = "./";

use std::cell::RefCell;
use std::thread;




fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    let mut config_server = ConfigServer::new();
    //static LOGGER: Logger<String> = Logger::new(LOG_NAME, LOG_RELATIVE_PATH).unwrap();
    match argv.len() {
        2 => {
            println!("Load file config ...");
            config_server.load_config_server_with_path(argv[1].as_str());
        }
        1 => {
            println!("Load file config server default ...");
            config_server.load_config_server();
        }
        _ => { println!("Error count args");}
    }
    let app_name = &argv[0];
    println!("Serger args: {:?}",  &argv);
    println!("Server {} is up!",app_name);

    let server = config_server.get_prop("server");
    let port = config_server.get_prop("port");

    let mut path = String::from(server);
    path.push_str(":");
    path.push_str(&port);

    println!();
    println!("Server address: {}", &path);
    println!();

    println!("Execute listener ...");

    let _listener = exec_server(&path);

    Ok(())
}

fn exec_server(address: &String) -> Result<(),()>{
    let threadpool = threadpool::ThreadPool::new(THREAD_POOL_COUNT.clone());
    let listener = TcpListener::bind(&address).unwrap();
    for stream in listener.incoming() {
        let stream = stream;
        println!("Handler stream request ...");
        threadpool.execute(|| {
            handle_connection(stream.unwrap());
        });
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    handle_client(&mut stream);
}

fn handle_client(stream: &mut TcpStream) {
    let stream_reader = stream.try_clone().expect("Cannot clone stream reader");
    let reader = BufReader::new(stream_reader);

    let mut lines = reader.lines();
    println!("Reading stream conections ...");

    while let Some(line) = lines.next() {
        let request = line.unwrap_or(String::from(END_FLAG));

        if request == END_FLAG {
            return
        }
        println!("Server receive: {:?}", request);

        let response = process_request(request);
        (*stream).write(response.as_bytes()).unwrap_or(0);
    }
    println!("End handle client");
}

fn process_request(request: String) -> String {
    let mut command_builder = CommandBuilder::new();
    let comm = command_builder.get_command(&mut String::from(request.trim()));
    String::from(comm.str_response())
}