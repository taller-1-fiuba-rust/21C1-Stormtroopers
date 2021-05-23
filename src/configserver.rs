use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

/*
 * Min redis.config props
 *  verbose: "false",
 *  port: "8080",
 *  timeout: "60",
 *  dbfilename: "dump.rdb",
 *  logfile: "redislog.log",
 */

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct ConfigServer {
    pub props: HashMap<String, String>,
}

impl ConfigServer {
    pub fn new() -> ConfigServer {
        let map = HashMap::new();
        ConfigServer {
            props: map,
        }
    }
    pub fn load_config_server_with_path(&mut self, path_file: &str) {
        println!("Init load file config ...");
        if let Ok(lines) = read_lines(path_file) {
            for line in lines {
                if let Ok(prop) = line {
                    let prop3:Vec<&str> = prop.split("=").collect();
                    if prop3.len() == 2 {
                        self.props.insert(String::from(prop3[0]),String::from(prop3[1]));
                    }
                }
            }
            println!("Load file config OK");
        }
    }

    pub fn load_config_server(&mut self) {
        self.load_config_server_with_path("./redis.config");
    }

    pub fn get_prop(&self, prop_name: &str) -> String {
        if let Some(prop) = self.props.get(prop_name){
            return String::from(prop.as_str());
        };
        String::from(prop_name)
    }
}