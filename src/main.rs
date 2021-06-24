use crate::command::command_builder::CommandBuilder;
use crate::server::app_info::AppInfo;
use crate::server::connection::Connection;
use crate::server::connection_resolver::ConnectionResolver;
use crate::server::threadpool::ThreadPool;
use crate::server::utils::format_timestamp_now;
use std::env::args;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

mod command;
mod data_base;
mod errors;
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

fn exec_server(address: &str, app_info: &mut AppInfo) -> Result<(), std::io::Error> {
    let threadpool = ThreadPool::new(THREAD_POOL_COUNT);

    let timeout = app_info.get_timeout();
    println!("Timeout for connections: {:?} secs", timeout);

    app_info.get_ttl_scheduler().run(&app_info);
    let listener = TcpListener::bind(&address)?;

    for (id_client, stream) in listener.incoming().enumerate() {
        let app_info = app_info.clone();
        let stream = stream?;

        let receiver = app_info.connect_client(id_client);

        println!("Handler stream request ...");

        threadpool_read(
            &threadpool,
            app_info.get_connection_resolver(),
            stream.try_clone().expect("Clone failed"),
            app_info,
            0,
            id_client,
            address.to_string().clone(),
        );

        let rx = receiver.clone();
        threadpool_write(
            &threadpool,
            rx,
            stream.try_clone().expect("Clone failed"),
            address.to_string(),
        );
    }

    Ok(())
}

fn threadpool_read(
    threadpool: &ThreadPool,
    connection_resolver: ConnectionResolver,
    stream: TcpStream,
    mut app_info: AppInfo,
    id_global: u32,
    id_client: usize,
    address: String,
) {
    threadpool.execute(move || {
        handle_connection(
            connection_resolver,
            stream,
            &mut app_info,
            id_global,
            id_client,
            address,
        );
    });
}

fn threadpool_write(
    threadpool: &ThreadPool,
    rx: Arc<Mutex<Receiver<String>>>,
    stream: TcpStream,
    address: String,
) {
    threadpool.execute(move || {
        let r = rx.lock().unwrap();
        for msg in r.iter() {
            let mut stream = stream.try_clone().expect("Clone failed");
            stream.write_all(msg.as_bytes()).unwrap_or(());

            if msg == *MSG_OVER {
                stream
                    .shutdown(Shutdown::Both)
                    .expect("Shutdown call failed");
            } else {
                write_redis_msg(address.clone(), stream);
            }
        }
    });
}

fn write_redis_msg(address: String, mut stream: TcpStream) {
    let mut msg = address;
    msg.push_str("> ");
    stream.write_all(msg.as_bytes()).unwrap();
}

fn handle_connection(
    connection_resolver: ConnectionResolver,
    mut stream: TcpStream,
    app_info: &mut AppInfo,
    id: u32,
    id_client: usize,
    address: String,
) {
    handle_client(
        connection_resolver,
        &mut stream,
        app_info,
        id,
        id_client,
        address,
    );
}

fn handle_client(
    connection_resolver: ConnectionResolver,
    stream: &mut TcpStream,
    app_info: &mut AppInfo,
    id: u32,
    id_client: usize,
    address: String,
) {
    let mut connection_client = connection_resolver.get_connection_client(id_client);
    let stream_reader = stream.try_clone().expect("Cannot clone stream reader");
    let reader = BufReader::new(stream_reader);
    write_redis_msg(address.clone(), stream.try_clone().unwrap());

    let lines = reader.lines();
    println!("Reading stream conections, job {} ...", id);

    let mut request = "".to_string();

    for line in lines {
        if request != *"monitor" {
            let app_info = app_info.clone();
            request = line.unwrap_or_else(|_| String::from(END_FLAG));

            if request == END_FLAG || connection_client.over() {
                run_exit_cmd(connection_client, &app_info, id, id_client);
                return;
            }

            println!("Server job {}, receive: {:?}", id, request);

            let response = process_request(request.clone(), &app_info, id, id_client);
            connection_client.send(response);
            connection_client.renew_connection();
        }
    }
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
        "+{:?} [id: {:?} -- port: {:?}] ",
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

fn run_exit_cmd(
    connect_client: Connection<String>,
    app_info: &AppInfo,
    id_job: u32,
    id_client: usize,
) {
    let response = process_request(END_FLAG.to_string(), app_info, id_job, id_client);
    connect_client.send(response);
}
