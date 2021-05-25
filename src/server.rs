use crate::config_server::ConfigServer;
use crate::command_builder::CommandBuilder;
use crate::{LOG_NAME, LOG_PATH, ERROR_LOG_CREATE};

use crate::logger::{Logger, Loggable};
use std::time::SystemTime;

impl Loggable for Server {
    fn get_id_client(&self) -> i32 {
        3
    }
    fn get_id_thread(&self) -> i32 {
        0
    }
    fn get_timestamp(&self) -> SystemTime {
        SystemTime::now()
    }
}

pub struct Server {
    server_args: Vec<String>,
    config_server: ConfigServer,
    logger: Logger<String>,
    command_builder: CommandBuilder,
}

impl Clone for Server {
    fn clone(&self) -> Server {
        let config_server = self.config_server.clone();
        let logger = self.logger.clone();
        let command_builder = self.command_builder.clone();
        let server_args = self.server_args.clone();
        Self {
            server_args,
            config_server,
            logger,
            command_builder,
        }
    }
}

impl Server {
    pub fn new(args: Vec<String>) -> Self {
        let server_args = args;
        let config_server = ConfigServer::new();
        let logger =
            Logger::new(LOG_NAME.to_string(), LOG_PATH.to_string()).expect(ERROR_LOG_CREATE);
        let command_builder = CommandBuilder::new();
        Self {
            server_args,
            config_server,
            logger,
            command_builder,
        }
    }

    pub fn get_logger(&self) -> Logger<String> {
        self.logger.clone()
    }

    pub fn get_config_server(&self) -> ConfigServer {
        self.config_server.clone()
    }

    pub fn load_config(&mut self, argv: Vec<String>) -> Result<(), std::io::Error> {
        match argv.len() {
            2 => {
                self.logger.info(self, "Load file config ...")?;
                self
                    .config_server
                    .load_config_server_with_path(argv[1].as_str(), self.get_logger())?;
                Ok(())
            }
            1 => {
                self.logger.info(self,"Load file config server default ...")?;
                self
                    .config_server
                    .load_config_server(self.get_logger())?;
                Ok(())
            }
            _ => {
                self.logger.info(self,"Error count args")
            }
        }
    }

    pub fn server_name(&self) -> String {
        self.server_args[0].to_owned()
    }
}