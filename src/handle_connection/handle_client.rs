//! Handles the connection with a client, in particular the connection opening and closing.
use crate::constants::END_FLAG;
use crate::handle_connection::executor::write_redis_msg;
use crate::handle_connection::process_request::{process_request, run_exit_cmd};
use crate::server::app_info::AppInfo;
use crate::server::connection_resolver::ConnectionResolver;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn handle_client(
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
    write_redis_msg(address, stream.try_clone().unwrap());

    let lines = reader.lines();
    println!("Reading stream conections, job {} ...", id);

    let mut request = "".to_string();

    for line in lines {
        if request != *"monitor" {
            let mut app_info = app_info.clone();
            request = line.unwrap_or_else(|_| String::from(END_FLAG));

            if request == END_FLAG || connection_client.over() {
                run_exit_cmd(connection_client, &mut app_info, id, id_client);
                return;
            }

            println!("Server job {}, receive: {:?}", id, request);

            let response = process_request(request.clone(), &app_info, id, id_client);
            connection_client.send(response);
            connection_client.renew_connection();
        }
    }
}
