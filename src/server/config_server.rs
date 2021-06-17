use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::server::logger::Loggable;
use std::time::Duration;

/*
const INFO_LOAD_FILE_CONFIG: &str = "Init load file config ...\n";
const INFO_LOAD_FILE_CONFIG_OK: &str = "Load file config OK\n";
const ERROR_GETTING_PROP: &str = "Error getting property\n";
const ERROR_GETTING_PROP_DEFAULT: &str = "Error getting property default\n";
*/
const PATH_FILE_CONFIG_DEFAULT: &str = "./redis.config";

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
    fn get_id_client(&self) -> &str {
        "ConfigServer"
    }
    fn get_id_thread(&self) -> u32 {
        14_u32
    }
}

impl Clone for ConfigServer {
    fn clone(&self) -> Self {
        let verbose = self.verbose;
        let server = self.server.clone();
        let port = self.port.clone();
        let timeout = self.timeout;
        let dbfilename = self.dbfilename.clone();
        let logfile = self.logfile.clone();
        let path_config_file = self.path_config_file.clone();

        Self {
            verbose,
            server,
            port,
            timeout,
            dbfilename,
            logfile,
            path_config_file,
        }
    }
}

pub struct ConfigServer {
    verbose: bool,
    server: String,
    port: String,
    timeout: Duration,
    dbfilename: String,
    logfile: String,
    path_config_file: String,
}

pub struct ConfigServerBuilder {
    verbose: bool,
    server: String,
    port: String,
    timeout: Duration,
    dbfilename: String,
    logfile: String,
    path_config_file: String,
}

impl ConfigServerBuilder {
    pub fn new() -> Self {
        let verbose = false;
        let server = "0.0.0.0".to_string();
        let port = "8081".to_string();
        let timeout = Duration::new(60, 0);
        let dbfilename = "dump.rdb".to_string();
        let logfile = "redislog.log".to_string();
        let path_config_file = PATH_FILE_CONFIG_DEFAULT.to_string();

        Self {
            verbose,
            server,
            port,
            timeout,
            dbfilename,
            logfile,
            path_config_file,
        }
    }

    fn build(&self) -> ConfigServer {
        ConfigServer::new(
            self.verbose,
            self.server.clone(),
            self.port.clone(),
            self.timeout,
            self.dbfilename.clone(),
            self.logfile.clone(),
            self.path_config_file.clone(),
        )
    }

    fn find_verbose(&mut self, vec_values: Vec<String>, verbose: String) {
        let i = find_in_vector(vec_values.clone(), verbose);
        if i >= 0 {
            if let Ok(val) = vec_values[(i as usize) + 1].parse::<bool>() {
                self.verbose = val;
            } //hacerlo bien, tendría que retornar en todo error
        }
    }

    fn find_server(&mut self, vec_values: Vec<String>, server: String) {
        let i = find_in_vector(vec_values.clone(), server);

        if i >= 0 {
            self.server = vec_values[(i as usize) + 1].clone();
        }
    }

    fn find_port(&mut self, vec_values: Vec<String>, port: String) {
        let i = find_in_vector(vec_values.clone(), port);

        if i >= 0 {
            self.port = vec_values[(i as usize) + 1].clone();
        }
    }

    fn find_timeout(&mut self, vec_values: Vec<String>, timeout: String) {
        let i = find_in_vector(vec_values.clone(), timeout);
        if i >= 0 {
            if let Ok(val) = vec_values[(i as usize) + 1].clone().parse::<u64>() {
                self.timeout = Duration::new(val, 0);
            } //hacerlo bien, tendría que retornar en todo error
        }
    }

    fn find_dbfilename(&mut self, vec_values: Vec<String>, dbfilename: String) {
        let i = find_in_vector(vec_values.clone(), dbfilename);
        if i >= 0 {
            self.dbfilename = vec_values[(i as usize) + 1].clone();
        }
    }

    fn find_logfile(&mut self, vec_values: Vec<String>, logfile: String) {
        let i = find_in_vector(vec_values.clone(), logfile);
        if i >= 0 {
            self.logfile = vec_values[(i as usize) + 1].clone();
        }
    }

    fn find_values(&mut self, vec: Vec<String>) {
        self.find_verbose(vec.clone(), "verbose".to_string());
        self.find_server(vec.clone(), "server".to_string());
        self.find_port(vec.clone(), "port".to_string());
        self.find_timeout(vec.clone(), "timeout".to_string());
        self.find_dbfilename(vec.clone(), "dbfilename".to_string());
        self.find_logfile(vec, "logfile".to_string());
    }

    pub fn build_with_path(path: String) -> ConfigServer {
        let mut builder = ConfigServerBuilder::new();
        let vec_values = generate_vec_values(path);
        builder.find_values(vec_values);

        builder.build()
    }

    pub fn build_without_path() -> ConfigServer {
        let mut builder = ConfigServerBuilder::new();
        let vec_values = generate_vec_values(PATH_FILE_CONFIG_DEFAULT.to_string());
        builder.find_values(vec_values);

        builder.build()
    }
}

fn find_in_vector(vec: Vec<String>, x: String) -> i32 {
    let index = vec.iter().position(|elem| *elem == x).unwrap_or(usize::MAX);
    if index < usize::MAX {
        index as i32
    } else {
        -1
    }
}

fn generate_vec_values(path: String) -> Vec<String> {
    let mut vec_values = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.into_iter().flatten() {
            let prop_slited: Vec<&str> = line.split('=').collect();
            vec_values.push(prop_slited[0].to_string());
            vec_values.push(prop_slited[1].to_string());
        }
    }
    vec_values
}

impl ConfigServer {
    pub fn new(
        verbose: bool,
        server: String,
        port: String,
        timeout: Duration,
        dbfilename: String,
        logfile: String,
        path_config_file: String,
    ) -> Self {
        Self {
            verbose,
            server,
            port,
            timeout,
            dbfilename,
            logfile,
            path_config_file,
        }
    }

    pub fn get_server_port(&self) -> String {
        let mut port = self.server.clone();
        port.push(':');
        port.push_str(&self.port);
        port
    }

    pub fn get(&self) -> String {
        format!(
            "Verbose: {:?}\nPort: {:?}\nTimeout: {:?}\nDbFileName: {:?}\nLogfile: {:?}\n",
            self.verbose, self.port, self.timeout, self.dbfilename, self.logfile
        )
    }

    /*pub fn set(&self, key: String, value: String) -> String {

    }*/
}
