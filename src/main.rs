use std::env::args;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

use std::io::prelude::*;
use crate::threadpool::ThreadPool;
use crate::command::CommandBuilder;


mod threadpool;
mod command;

static SERVER_ARGS: usize = 3;
static THREAD_POOL_COUNT: usize = 2;

static END_FLAG: &str = "EOF";

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != SERVER_ARGS {
        println!("Count of arguments invalid");
        return Err(());
    }
    let app_name = &argv[0];
    println!("Serger args: {:?}",  &argv);

    println!("Server {} is up!",app_name);

    let mut address = argv[1].clone();
    let port = argv[2].clone();
    address.push_str(":");
    address.push_str(&port);

    println!("Server address: {}", &address);
    println!();
    println!("Execute listener ...");
    let _listener = exec(&address);

    Ok(())
}

fn exec(address: &String) -> Result<(),()>{
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
    let mut commandBuilder = CommandBuilder::new();
    let mut comm = commandBuilder.get_command(&mut String::from(request.trim()));
    String::from(comm.str_response())
}