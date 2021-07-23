//! The logic that the threadpool uses to read and write streams.
use crate::constants::MSG_OVER;
use crate::handle_connection::executor::write_redis_msg;
use crate::handle_connection::handle_client::handle_client;
use crate::server::threadpool::ThreadPool;
use crate::AppInfo;
use crate::ConnectionResolver;
use std::io::Write;
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

pub fn threadpool_read(
    threadpool: &ThreadPool,
    connection_resolver: ConnectionResolver,
    mut stream: TcpStream,
    mut app_info: AppInfo,
    id_global: u32,
    id_client: usize,
    address: String,
) {
    threadpool.execute(move || {
        handle_client(
            connection_resolver,
            &mut stream,
            &mut app_info,
            id_global,
            id_client,
            address,
        );
    });
}

pub fn threadpool_write(
    threadpool: &ThreadPool,
    rx: Arc<Mutex<Receiver<String>>>,
    stream: TcpStream,
    address: String,
    connection_resolver: ConnectionResolver,
    id_client: usize,
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
            //no usar connection_client acá porque el resolver es el que actualiza el monitor
            } else if connection_resolver.monitor(id_client) {
                continue;
            } else {
                write_redis_msg(address.clone(), stream);
            }
        }
    });
}
