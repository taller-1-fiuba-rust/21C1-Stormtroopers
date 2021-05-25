use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::logger::{Loggable, Logger};
use std::time::SystemTime;

/*
 * Min redis.config props
 *  verbose: "false",
 *  port: "8080",
 *  timeout: "60",
 *  dbfilename: "dump.rdb",
 *  logfile: "redislog.log",
 */

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl Loggable for ConfigServer {
    fn get_id_client(&self) -> i32 {
        1414
    }
    fn get_id_thread(&self) -> i32 {
        14
    }

    fn get_timestamp(&self) -> SystemTime {
        SystemTime::now()
    }
}

pub struct ConfigServer {
    pub props: HashMap<String, String>,
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
        ConfigServer { props: map }
    }

    pub fn load_config_server_with_path(
        &mut self,
        path_file: &str,
        logger: Logger<String>,
    ) -> Result<(), std::io::Error> {
        logger.info(self, "Init load file config ...")?;
        if let Ok(lines) = read_lines(path_file) {
            for line in lines {
                if let Ok(prop) = line {
                    let prop_slited: Vec<&str> = prop.split("=").collect();
                    if prop_slited.len() == 2 {
                        self.props
                            .insert(String::from(prop_slited[0]), String::from(prop_slited[1]));
                    }
                }
            }
            return logger.info(self, "Load file config OK");
        }
        Ok(())
    }

    pub fn load_config_server(&mut self, logger: Logger<String>) -> Result<(), std::io::Error> {
        self.load_config_server_with_path("./redis.config", logger)
    }

    pub fn get_server_port(&self, logger: Logger<String>) -> String {
        let logger2 = logger.clone();
        let port = self.get_prop("port", logger);

        let mut path_server_port = String::from(self.get_prop("server", logger2));

        path_server_port.push_str(":");
        path_server_port.push_str(&port);
        path_server_port
    }

    pub fn get_prop(&self, prop_name: &str, logger: Logger<String>) -> String {
        if let Some(prop) = self.props.get(prop_name) {
            logger
                .info(self, format!("Getting property: {}", prop).as_str())
                .expect("ERROR GETTING PROPERTY");
            return String::from(prop.as_str());
        };
        logger
            .info(
                self,
                format!("Getting property default: {}", prop_name).as_str(),
            )
            .expect("ERROR GETTING PROPERTY DEFAULT");
        String::from(prop_name)
    }
}
