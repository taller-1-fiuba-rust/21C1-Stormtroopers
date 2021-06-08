use crate::app_info::AppInfo;
use crate::command::command_builder::CommandBuilder;
use crate::connection::Connection;
use crate::threadpool::ThreadPool;
use std::env::args;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

mod app_info;
mod command;
mod config_server;
mod connection;
mod errors;
mod logger;
mod pubsub;
mod structure_general;
mod structure_list;
mod structure_set;
mod structure_string;
mod threadpool;
mod utils;

static THREAD_POOL_COUNT: usize = 8;
static END_FLAG: &str = "EOF";
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

    let config_server = server.get_config_server();

    let server_port = config_server.get_server_port(server.get_logger());

    println!("\nServer address: {}\n", server_port);
    println!("Execute listener ...");
    let _listener = exec_server(&server_port, &mut server);

    Ok(())
}

fn exec_server(address: &str, app_info: &mut AppInfo) -> Result<(), std::io::Error> {
    let threadpool = ThreadPool::new(THREAD_POOL_COUNT);

    let pubsub = app_info.get_pubsub();

    let listener = TcpListener::bind(&address)?;
    //let mut ids_clients = 0;

    for (ids_clients, stream) in listener.incoming().enumerate() {
        let id_client = ids_clients;
        //ids_clients += 1;

        let mut pubsub = pubsub.clone();
        let connection_client = Connection::<String>::new();

        let (tx_client, rx_client) = (
            connection_client.get_sender(),
            connection_client.get_receiver(),
        );

        let _rx_client = pubsub.add_client_with_recv(id_client, tx_client, rx_client);

        println!("Handler stream request ...");
        let app_info = app_info.clone();
        let stream = stream?;
        let _id_global = -1;
        let connection_clone = connection_client.clone();
        let receiver = connection_client.get_receiver();
        let stream_lectura = stream.try_clone().unwrap();

        threadpool.execute(move |_id_global| {
            handle_connection(
                connection_clone,
                stream.try_clone().unwrap(),
                &app_info,
                _id_global,
                id_client,
            );
        });

        let rx = receiver.clone();
        //let _connection = connection_client.clone();
        threadpool.execute(move |_| {
            let r = rx.lock().unwrap();
            for msg in r.iter() {
                stream_lectura
                    .try_clone()
                    .unwrap()
                    .write_all(msg.as_bytes())
                    .unwrap();
            }
        });
    }

    Ok(())
}

fn handle_connection(
    connection_client: Connection<String>,
    mut stream: TcpStream,
    app_info: &AppInfo,
    id: u32,
    id_client: usize,
) {
    handle_client(connection_client, &mut stream, app_info, id, id_client);
}

fn handle_client(
    connection_client: Connection<String>,
    stream: &mut TcpStream,
    app_info: &AppInfo,
    id: u32,
    id_client: usize,
) {
    let stream_reader = stream.try_clone().expect("Cannot clone stream reader");
    let reader = BufReader::new(stream_reader);

    let lines = reader.lines();
    println!("Reading stream conections, job {} ...", id);

    for line in lines {
        let app_info = app_info.clone();
        let request = line.unwrap_or_else(|_| String::from(END_FLAG));

        if request == END_FLAG {
            return; //tendríamos que cerrar a los clientes (drop y demás)
        }

        println!("Server job {}, receive: {:?}", id, request);

        let response = process_request(request, &app_info, id, id_client);
        connection_client.send(response);
    }
    println!("End handle client, job {}", id);
}

//TODO: ver porque si vienen mal los args explota
fn process_request(request: String, app_info: &AppInfo, id_job: u32, id_client: usize) -> String {
    let command_builder = CommandBuilder::new(id_job, app_info.get_logger());

    let comm = command_builder.get_command(&String::from(request.trim()));
    let mut command_splited: Vec<&str> = request.split(' ').collect();
    command_splited.remove(0);

    match comm {
        Ok(comm) => match comm.run(command_splited, app_info, id_client) {
            Ok(res) => res,
            Err(res) => res.to_string(),
        },
        Err(comm) => comm.to_string(),
    }
}
