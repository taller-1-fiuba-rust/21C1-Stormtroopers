use crate::config_server::ConfigServer;
use crate::logger::{Loggable, Logger};
use crate::pubsub::Pubsub;
use crate::structure_string::StructureString;
use crate::ttl_scheduler::TTLScheduler;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

const INFO_LOAD_FILE_CONFIG: &str = "Load file config ...\n";
const INFO_LOAD_FILE_CONFIG_DEFAULT: &str = "Load file config server default ...\n";
const ERROR_COUNT_ARGS: &str = "Error count args\n";
const LOG_NAME: &str = "log";
const LOG_PATH: &str = "./";
const ERROR_LOG_CREATE: &str = "Error creating Logger";

impl Loggable for AppInfo {
    fn get_id_client(&self) -> &str {
        "AppInfo"
    }
    fn get_id_thread(&self) -> u32 {
        0_u32
    }
}

pub struct AppInfo {
    args: Vec<String>,
    config_server: ConfigServer,
    logger: Logger<String>,
    structure: StructureString<String>,
    pubsub: Pubsub,
    ttl_scheduler: TTLScheduler,
    ids_clients: i32,
}

impl Clone for AppInfo {
    fn clone(&self) -> Self {
        let config_server = self.config_server.clone();
        let logger = self.logger.clone();
        let args = self.args.clone();
        let structure = self.structure.clone();
        let pubsub = self.pubsub.clone();
        let ttl_scheduler = self.ttl_scheduler.clone();

        Self {
            args,
            config_server,
            logger,
            structure,
            pubsub,
            ttl_scheduler,
            ids_clients: 0,
        }
    }
}

impl AppInfo {
    pub fn new(args: Vec<String>) -> Self {
        let config_server = ConfigServer::new();
        let logger =
            Logger::new(LOG_NAME.to_string(), LOG_PATH.to_string()).expect(ERROR_LOG_CREATE);
        let structure = StructureString::new();
        let pubsub = Pubsub::new();
        let ttl_scheduler = TTLScheduler::new();

        Self {
            args,
            config_server,
            logger,
            structure,
            pubsub,
            ttl_scheduler,
            ids_clients: 0,
        }
    }

    pub fn get_logger(&self) -> Logger<String> {
        self.logger.clone()
    }

    pub fn get_config_server(&self) -> ConfigServer {
        self.config_server.clone()
    }

    pub fn get_structure(&self) -> StructureString<String> {
        self.structure.clone()
    }

    pub fn get_pubsub(&self) -> Pubsub {
        self.pubsub.clone()
    }

    pub fn get_id_client(&self) -> i32 {
        self.ids_clients
    }

    pub fn inc_ids(&mut self) {
        self.ids_clients += 1;
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
        self.args[0].to_owned()
    }
}

//IMPORTANTE -> Después divido esto en file Connection
//asigno, por cada conexión nueva, un Connection nuevo
pub struct Connection<String> {
    sender: Arc<Mutex<Sender<String>>>,
    receiver: Arc<Mutex<Receiver<String>>>,
}

impl<String> Connection<String> {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            sender: Arc::new(Mutex::new(tx)),
            receiver: Arc::new(Mutex::new(rx)),
        }
    }

    pub fn send(&self, response: String) {
        let sender = self.sender.lock().unwrap();
        sender.send(response).unwrap();
    }

    pub fn get_sender(&self) -> Arc<Mutex<Sender<String>>> {
        self.sender.clone()
    }

    pub fn get_receiver(&self) -> Arc<Mutex<Receiver<String>>> {
        self.receiver.clone()
    }
}

impl<String> Clone for Connection<String> {
    fn clone(&self) -> Self {
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();
        Self { sender, receiver }
    }
}
