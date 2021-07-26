//! Este proyecto es una implementación acotada de Redis hecha en Rust, como trabajo integrador de la materia
//! Taller de Programación I, cátedra Deimonnaz.
//!
//! El objetivo principal de este trabajo es integrar los nuevos conceptos presentados en la materia,
//! en particular Web Sockets y Concurrencia básica.
//!
//! Corrector:
//! Kelman, Uriel
//!
//! Alumnos:
//! Sabatino, Gonzalo
//! Verón, Lucas
//! Queirolo Dominguez, Cristian Daniel
//!
use crate::handles::executor::exec_server;
use crate::server::app_info::AppInfo;
use crate::server::connection::Connection;
use crate::server::connection_resolver::ConnectionResolver;
use std::env::args;

mod command;
mod constants;
mod data_base;
mod errors;
mod handles;
mod server;

/// Main function of the app, this works as it's main entrance point.
/// Is in charge of reading the command line arguments and creates the server 'facade' structure and executes it.
fn main() {
    let argv = args().collect::<Vec<String>>();

    println!("Init Server ...");

    let mut server = match AppInfo::new(argv) {
        Ok(s) => s,
        Err(e) => panic!("{}", e.message),
    };

    println!("Server {} is up!", server.server_name());

    let server_port = server.get_server_port();

    println!("\nServer address: {}\n", server_port);
    println!("Execute listener ...");
    let _listener = exec_server(&server_port, &mut server);
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    extern crate redis;
    const REDIS_HOST_TEST: &str = "redis://127.0.0.1:8081";
    #[test]
    fn redis_it() -> redis::RedisResult<()> {
            let client = redis::Client::open(REDIS_HOST_TEST)?;
            let mut con = client.get_connection()?;

            /* do something here */

            let res : () = redis::cmd("SET").arg("my_key").arg(42).query(&mut con)?;
            let res2 = redis::cmd("GET").arg("my_key").query(&mut con);
            assert!(true);
            assert_eq!(res2, Ok("42".to_string()));
        Ok(())
    }
}
 */