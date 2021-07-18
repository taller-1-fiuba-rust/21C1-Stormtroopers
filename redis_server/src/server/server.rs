use crate::config_server::ConfigServer;
use crate::{ERROR_LOG_CREATE, LOG_NAME, LOG_PATH};

use crate::logger::{Loggable, Logger};
use crate::structure_string::StructureString;

const INFO_LOAD_FILE_CONFIG: &str = "Load file config ...\n";
const INFO_LOAD_FILE_CONFIG_DEFAULT: &str = "Load file config server default ...\n";
const ERROR_COUNT_ARGS: &str = "Error count args\n";

impl Loggable for Server {
    fn get_id_client(&self) -> &str {
        "Server"
    }
    fn get_id_thread(&self) -> u32 {
        0_u32
    }
}

pub struct Server {
    server_args: Vec<String>,
    config_server: ConfigServer,
    logger: Logger<String>,
    pub structure: StructureString<String>,
}

impl Clone for Server {
    fn clone(&self) -> Server {
        let config_server = self.config_server.clone();
        let logger = self.logger.clone();
        let server_args = self.server_args.clone();
        let structure = self.structure.clone();
        Self {
            server_args,
            config_server,
            logger,
            structure,
        }
    }
}

impl Server {
    pub fn new(args: Vec<String>) -> Self {
        let server_args = args;
        let config_server = ConfigServer::new();
        let logger =
            Logger::new(LOG_NAME.to_string(), LOG_PATH.to_string()).expect(ERROR_LOG_CREATE);
        let structure = StructureString::new();
        Self {
            server_args,
            config_server,
            logger,
            structure,
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
                self.logger.info(self, INFO_LOAD_FILE_CONFIG)?;
                self.config_server
                    .load_config_server_with_path(argv[1].as_str(), self.get_logger())?;
                Ok(())
            }
            1 => {
                self.logger.info(self, INFO_LOAD_FILE_CONFIG_DEFAULT)?;
                self.config_server.load_config_server(self.get_logger())?;
                Ok(())
            }
            _ => self.logger.info(self, ERROR_COUNT_ARGS),
        }
    }

    pub fn server_name(&self) -> String {
        self.server_args[0].to_owned()
    }
}
