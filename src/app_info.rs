use crate::config_server::ConfigServer;
use crate::logger::{Loggable, Logger};
use crate::pubsub::Pubsub;
use crate::structure_general::Structure;
use crate::structure_general::StructureGeneral;
use crate::structure_list::StructureList;
use crate::structure_set::StructureSet;
use crate::structure_string::StructureString;

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
    structure: StructureGeneral,
    pubsub: Pubsub,
}

impl Clone for AppInfo {
    fn clone(&self) -> Self {
        let config_server = self.config_server.clone();
        let logger = self.logger.clone();
        let args = self.args.clone();
        let structure = self.structure.clone();
        let pubsub = self.pubsub.clone();

        Self {
            args,
            config_server,
            logger,
            structure,
            pubsub,
        }
    }
}

fn add_string(structure_general: &StructureGeneral) {
    let structure_string = Structure::StructureString(StructureString::new());
    structure_general.add_structure("String".to_string(), structure_string);
}

fn add_list(structure_general: &StructureGeneral) {
    let structure_list = Structure::StructureList(StructureList::new());
    structure_general.add_structure("List".to_string(), structure_list);
}

fn add_set(structure_general: &StructureGeneral) {
    let structure_set = Structure::StructureSet(StructureSet::new());
    structure_general.add_structure("Set".to_string(), structure_set);
}

fn create_structure() -> StructureGeneral {
    let structure_general = StructureGeneral::new();
    add_string(&structure_general);
    add_list(&structure_general);
    add_set(&structure_general);

    structure_general
}

impl AppInfo {
    pub fn new(args: Vec<String>) -> Self {
        let config_server = ConfigServer::new();
        let logger =
            Logger::new(LOG_NAME.to_string(), LOG_PATH.to_string()).expect(ERROR_LOG_CREATE);
        let structure = create_structure();

        let pubsub = Pubsub::new();

        Self {
            args,
            config_server,
            logger,
            structure,
            pubsub,
        }
    }

    pub fn get_logger(&self) -> Logger<String> {
        self.logger.clone()
    }

    pub fn get_config_server(&self) -> ConfigServer {
        self.config_server.clone()
    }

    pub fn get_structure(&self) -> StructureGeneral {
        self.structure.clone()
    }

    pub fn get_pubsub(&self) -> Pubsub {
        self.pubsub.clone()
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
