use crate::config_server::ConfigServer;
use crate::db_list::DataBaseList;
use crate::db_resolver::*;
use crate::db_set::DataBaseSet;
use crate::db_string::DataBaseString;
use crate::logger::{Loggable, Logger};
use crate::pubsub::Pubsub;

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
    db: DataBaseResolver,
    pubsub: Pubsub,
}

impl Clone for AppInfo {
    fn clone(&self) -> Self {
        let config_server = self.config_server.clone();
        let logger = self.logger.clone();
        let args = self.args.clone();
        let db = self.db.clone();
        let pubsub = self.pubsub.clone();

        Self {
            args,
            config_server,
            logger,
            db,
            pubsub,
        }
    }
}

fn add_string(db_resolver: &DataBaseResolver) {
    let db_string = DataBase::DataBaseString(DataBaseString::new());
    db_resolver.add_structure("String".to_string(), db_string);
}

fn add_list(db_resolver: &DataBaseResolver) {
    let db_list = DataBase::DataBaseList(DataBaseList::new());
    db_resolver.add_structure("List".to_string(), db_list);
}

fn add_set(db_resolver: &DataBaseResolver) {
    let db_set = DataBase::DataBaseSet(DataBaseSet::new());
    db_resolver.add_structure("Set".to_string(), db_set);
}

fn create_structure() -> DataBaseResolver {
    let db_resolver = DataBaseResolver::new();
    add_string(&db_resolver);
    add_list(&db_resolver);
    add_set(&db_resolver);

    db_resolver
}

impl AppInfo {
    pub fn new(args: Vec<String>) -> Self {
        let config_server = ConfigServer::new();
        let logger =
            Logger::new(LOG_NAME.to_string(), LOG_PATH.to_string()).expect(ERROR_LOG_CREATE);
        let db = create_structure();

        let pubsub = Pubsub::new();

        Self {
            args,
            config_server,
            logger,
            db,
            pubsub,
        }
    }

    pub fn get_logger(&self) -> Logger<String> {
        self.logger.clone()
    }

    pub fn get_config_server(&self) -> ConfigServer {
        self.config_server.clone()
    }

    pub fn get_db_resolver(&self) -> DataBaseResolver {
        self.db.clone()
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

    pub fn get_string_db(&self) -> DataBaseString<String> {
        self.db.get_string_db()
    }

    pub fn get_list_db(&self) -> DataBaseList<String> {
        self.db.get_list_db()
    }

    pub fn get_set_db(&self) -> DataBaseSet<String> {
        self.db.get_set_db()
    }
}
