use crate::errors::run_error::RunError;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;

use crate::constants::SHARING_COUNT_DEFAULT;
use crate::server::logger::{Loggable, Logger};

const INFO_LOAD_FILE_CONFIG: &str = "Init load file config ...\n";
const INFO_LOAD_FILE_CONFIG_OK: &str = "Load file config OK\n";
const PATH_FILE_CONFIG_DEFAULT: &str = "./redis.config";

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl Loggable for ConfigServer {
    fn get_id_client(&self) -> &str {
        "ConfigServer"
    }
    fn get_id_thread(&self) -> u32 {
        14_u32
    }
}

pub struct ConfigServer {
    pub props: Arc<Mutex<HashMap<String, String>>>,
}

impl Default for ConfigServer {
    fn default() -> Self {
        ConfigServer::new()
    }
}

impl Clone for ConfigServer {
    fn clone(&self) -> Self {
        let props = self.props.clone();
        Self { props }
    }
}

impl ConfigServer {
    pub fn new() -> ConfigServer {
        let map = HashMap::new();
        ConfigServer {
            props: Arc::new(Mutex::new(map)),
        }
    }

    pub fn load_config_server_with_path(
        &mut self,
        path_file: &str,
        logger: Logger<String>,
    ) -> Result<(), std::io::Error> {
        logger.info(self, INFO_LOAD_FILE_CONFIG, false)?;
        let mut props = self.props.lock().unwrap();
        println!("Load file config {}", path_file);
        if let Ok(lines) = read_lines(path_file) {
            for line in lines.into_iter().flatten() {
                //let mut props = props.clone();
                let prop_slited: Vec<&str> = line.split('=').collect();
                println!("Load prop config '{}' -> {}", prop_slited[0], prop_slited[1]);
                props.insert(String::from(prop_slited[0]), String::from(prop_slited[1]));
            }
            return logger.info(self, INFO_LOAD_FILE_CONFIG_OK, false);
        }
        Ok(())
    }

    pub fn load_config_server(&mut self, logger: Logger<String>) -> Result<(), std::io::Error> {
        self.load_config_server_with_path(PATH_FILE_CONFIG_DEFAULT, logger)
    }

    pub fn get_server_port(&self, _logger: Logger<String>) -> String {
        let port = self.get_prop("port");

        let mut path_server_port = self.get_prop("server");

        path_server_port.push(':');
        path_server_port.push_str(&port);
        path_server_port
    }

    pub fn get_prop(&self, prop: &str) -> String {
        let map = self.props.lock().unwrap();
        map.get(prop).unwrap().to_string()
    }

    pub fn get(&self) -> String {
        let mut msg = "".to_string();
        let props = self.props.lock().unwrap();
        for (i, (key, value)) in props.iter().enumerate() {
            let str_aux = format!("{:?}) {:?}: {:?}\n", i, key, value);
            msg.push_str(&str_aux);
        }
        msg
    }

    pub fn get_verbose(&self) -> bool {
        let props = self.props.lock().unwrap();
        let verbose = props.get("verbose").unwrap(); //hacerlo bien
        parse_value(verbose.to_string(), false)
    }

    pub fn get_timeout(&self) -> u64 {
        let props = self.props.lock().unwrap();
        let timeout = props.get("timeout").unwrap(); //hacerlo bien
        parse_value(timeout.to_string(), 0)
    }

    pub fn get_count_sharing_db(&self) -> Result<u32, RunError> {
        let props = self.props.lock().unwrap();
        let count = match props.get("sharing_count") {
            Some(sc) => parse_value(sc.to_string(), 1),
            None => SHARING_COUNT_DEFAULT,
        };
        if count < 1 {
            return Err(RunError {
                message: "La propiedad 'sharing_count' debe ser mayor a 0".to_string(),
                cause: "Propiedad invalida\n".to_string(),
            });
        }
        Ok(count)
    }

    pub fn set(&self, key: String, value: String) -> Result<String, RunError> {
        let mut map = self.props.lock().unwrap();
        if !map.contains_key(&key) | key_not_allowed(key.clone()) {
            return Err(RunError {
                message: "Property is not allowed to change".to_string(),
                cause: "The property must be: verbose, logfile or dbfilename\n".to_string(),
            });
        }
        map.insert(key, value);
        Ok("OK".to_string())
    }
}

fn parse_value<T>(value: String, type_to_parse: T) -> T
where
    T: FromStr,
{
    if let Ok(val) = value.parse::<T>() {
        return val;
    }

    type_to_parse
}

fn key_not_allowed(key: String) -> bool {
    !((key == *"verbose") | (key == *"logfile") | (key == *"dbfilename"))
}
