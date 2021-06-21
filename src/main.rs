use crate::command::command_builder::CommandBuilder;
use crate::server::app_info::AppInfo;
use crate::server::connection::Connection;
use crate::server::threadpool::ThreadPool;
use crate::server::utils::format_timestamp_now;
use std::env::args;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::{Shutdown, TcpListener, TcpStream};

mod command;
mod data_base;
mod errors;
mod server;

static THREAD_POOL_COUNT: usize = 8;
static END_FLAG: &str = "quit";
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

fn exec_server(address: &str, app_info: &mut AppInfo) -> Result<(), std::io::Error> {
    let threadpool = ThreadPool::new(THREAD_POOL_COUNT);

    let pubsub = app_info.get_pubsub();
    let mut priv_pubsub = app_info.get_private_pubsub();
    let timeout = app_info.get_timeout();
    println!("Timeout for connections: {:?} secs", timeout);

    let listener = TcpListener::bind(&address)?;

    for (ids_clients, stream) in listener.incoming().enumerate() {
        let id_client = ids_clients;

        let mut pubsub = pubsub.clone();
        let connection_client = Connection::<String>::new(timeout);

        let (tx_client, rx_client) = (
            connection_client.get_sender(),
            connection_client.get_receiver(),
        );

        let _ = priv_pubsub.add_client_with_recv(id_client, tx_client.clone(), rx_client.clone());
        let _rx_client = pubsub.add_client_with_recv(id_client, tx_client, rx_client);

        println!("Handler stream request ...");
        let app_info = app_info.clone();
        let stream = stream?;
        let _id_global = 0;
        let connection_clone = connection_client.clone();
        let receiver = connection_client.get_receiver();
        let stream_lectura = stream.try_clone().unwrap();
        let ad = address.to_string();
        let ad_clone = ad.clone();

        let _thread_write = threadpool.execute(move || {
            handle_connection(
                connection_clone,
                stream.try_clone().unwrap(),
                &app_info,
                _id_global,
                id_client,
                ad_clone,
            );
        });

        let rx = receiver.clone();
        let _thread_read = threadpool.execute(move || {
            let r = rx.lock().unwrap();
            for msg in r.iter() {
                stream_lectura
                    .try_clone()
                    .unwrap()
                    .write_all(msg.as_bytes())
                    .unwrap_or(());

                if msg == *MSG_OVER {
                    stream_lectura
                        .shutdown(Shutdown::Both)
                        .expect("shutdown call failed");
                } else {
                    write_redis_msg(ad.clone(), stream_lectura.try_clone().unwrap());
                }
            }
        });

        //drop(thread_write);
        //drop(thread_read);
    }

    Ok(())
}

fn write_redis_msg(address: String, mut stream: TcpStream) {
    let msg = format!("{:?}> ", address);
    stream.write_all(msg.as_bytes()).unwrap();
}

fn handle_connection(
    connection_client: Connection<String>,
    mut stream: TcpStream,
    app_info: &AppInfo,
    id: u32,
    id_client: usize,
    address: String,
) {
    handle_client(
        connection_client,
        &mut stream,
        app_info,
        id,
        id_client,
        address,
    );
}

fn handle_client(
    mut connection_client: Connection<String>,
    stream: &mut TcpStream,
    app_info: &AppInfo,
    id: u32,
    id_client: usize,
    address: String,
) {
    let stream_reader = stream.try_clone().expect("Cannot clone stream reader");
    let reader = BufReader::new(stream_reader);
    write_redis_msg(address, stream.try_clone().unwrap());

    let lines = reader.lines();
    println!("Reading stream conections, job {} ...", id);

    let mut request = "".to_string();

    for line in lines {
        //hacer un poco más prolijo esto
        if request != *"monitor" {
            let app_info = app_info.clone();
            request = line.unwrap_or_else(|_| String::from(END_FLAG));

            if request == END_FLAG || connection_client.over() {
                //printear en servidor que el cliente se desconectó
                connection_client.send(MSG_OVER.to_string());
                println!("Disconnecting client {:?}", id);
                //drop(connection_client);
                return; //tendríamos que cerrar al cliente (drop y demás)
            }

            println!("Server job {}, receive: {:?}", id, request);

            let response = process_request(request.clone(), &app_info, id, id_client);
            connection_client.send(response);
            connection_client.renew_connection();
        }
    }
    println!("End handle client, job {}", id);
}

//TODO: ver porque si vienen mal los args explota
fn process_request(request: String, app_info: &AppInfo, id_job: u32, id_client: usize) -> String {
    let command_builder = CommandBuilder::new(id_job, app_info.get_logger());

    let comm = command_builder.get_command(&String::from(request.trim()));
    let mut command_splited: Vec<&str> = request.split(' ').collect();
    publish_monitor(app_info.clone(), command_splited.clone(), id_client);

    command_splited.remove(0);

    match comm {
        Ok(comm) => match comm.run(command_splited, app_info, id_client) {
            Ok(res) => res,
            Err(res) => res.to_string(),
        },
        Err(comm) => comm.to_string(),
    }
}

fn publish_monitor(app_info: AppInfo, args: Vec<&str>, id_client: usize) {
    let priv_pubsub = app_info.get_private_pubsub();
    let port = app_info.get_server_port();

    let mut msg = format!(
        "{:?} [id: {:?} -- port: {:?}] ",
        format_timestamp_now(),
        id_client,
        port
    );

    for arg in args {
        let msg_aux = format!("{:?} ", arg);
        msg.push_str(&msg_aux);
    }

    priv_pubsub.publish("MONITOR".to_string(), msg);
}
