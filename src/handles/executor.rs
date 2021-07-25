//! Responsible of setting and executing the main threadpool.
use crate::constants::THREAD_POOL_COUNT;
use crate::data_base::filedump::{load_filedump, start_filedump};
use crate::handles::handle_threadpool::{threadpool_read, threadpool_write};
use crate::server::app_info::AppInfo;
use crate::server::threadpool::ThreadPool;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

///Initial server run.
/// * Get the server time out for connections.
/// * Run the ttl_scheduler
/// * Start the filedump process
/// * Open the connection to listen the messages
/// * Run the threadpool reader & threadpool writer
pub fn exec_server(address: &str, app_info: &mut AppInfo) -> Result<(), std::io::Error> {
    let threadpool = ThreadPool::new(THREAD_POOL_COUNT);

    let timeout = app_info.get_timeout();
    println!("Timeout for connections: {:?} secs", timeout);

    app_info.get_ttl_scheduler().run(app_info.clone());

    println!("{}", load_filedump(&app_info));

    start_filedump(&app_info);

    let listener = TcpListener::bind(&address)?;

    for (id_client, stream) in listener.incoming().enumerate() {
        let mut app_info = app_info.clone();
        let app_info2 = app_info.clone();
        let stream = stream?;

        let receiver = app_info.connect_client(id_client);

        println!("Handler stream request ...");
        app_info.activate_threads(2);

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
            app_info2.get_connection_resolver(),
            id_client,
        );
    }

    Ok(())
}

pub fn write_redis_msg(address: String, mut stream: TcpStream) {
    let mut msg = address;
    msg.push_str("> ");
    stream.write_all(msg.as_bytes()).unwrap();
}
