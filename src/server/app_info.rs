use crate::data_base::db_list::DataBaseList;
use crate::data_base::db_resolver::*;
use crate::data_base::db_set::DataBaseSet;
use crate::data_base::db_string::DataBaseString;
use crate::server::config_server::ConfigServer;
use crate::server::logger::{Loggable, Logger};
use crate::server::pubsub::Pubsub;
use crate::server::ttl_scheduler::TtlScheduler;

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
    ttl_scheduler: TtlScheduler,
    ids_clients: i32,
    private_pubsub: Pubsub,
}

impl Clone for AppInfo {
    fn clone(&self) -> Self {
        let config_server = self.config_server.clone();
        let logger = self.logger.clone();
        let args = self.args.clone();
        let db = self.db.clone();
        let pubsub = self.pubsub.clone();
        let ttl_scheduler = self.ttl_scheduler.clone();
        let private_pubsub = self.private_pubsub.clone();

        Self {
            args,
            config_server,
            logger,
            db,
            pubsub,
            ttl_scheduler,
            ids_clients: 0,
            private_pubsub,
        }
    }
}

fn add_string(db_resolver: &DataBaseResolver) {
    let db_string = DataBase::DataBaseString(DataBaseString::new());
    db_resolver.add_data_base("String".to_string(), db_string);
}

fn add_list(db_resolver: &DataBaseResolver) {
    let db_list = DataBase::DataBaseList(DataBaseList::new());
    db_resolver.add_data_base("List".to_string(), db_list);
}

fn add_set(db_resolver: &DataBaseResolver) {
    let db_set = DataBase::DataBaseSet(DataBaseSet::new());
    db_resolver.add_data_base("Set".to_string(), db_set);
}

fn create_structure() -> DataBaseResolver {
    let db_resolver = DataBaseResolver::new();
    add_string(&db_resolver);
    add_list(&db_resolver);
    add_set(&db_resolver);

    db_resolver
}

fn create_private_pubsub() -> Pubsub {
    //ver si lo necesito, o con los sucribe de los comandos ya basta
    let mut pubsub = Pubsub::new();
    pubsub.create_channel("MONITOR".to_string());
    pubsub
}

impl AppInfo {
    pub fn new(args: Vec<String>) -> Self {
        let config_server = ConfigServer::new();
        let logger =
            Logger::new(LOG_NAME.to_string(), LOG_PATH.to_string()).expect(ERROR_LOG_CREATE);
        let db = create_structure();

        let pubsub = Pubsub::new();
        let ttl_scheduler = TtlScheduler::new();
        let private_pubsub = create_private_pubsub();

        Self {
            args,
            config_server,
            logger,
            db,
            pubsub,
            ttl_scheduler,
            ids_clients: 0,
            private_pubsub,
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

    pub fn get_id_client(&self) -> i32 {
        self.ids_clients
    }

    pub fn get_ttl_scheduler(&self) -> TtlScheduler {
        self.ttl_scheduler.clone()
    }

    pub fn inc_ids(&mut self) {
        self.ids_clients += 1;
    }

    pub fn get_private_pubsub(&self) -> Pubsub {
        self.private_pubsub.clone()
    }

    pub fn load_config(&mut self, argv: Vec<String>) -> Result<(), std::io::Error> {
        match argv.len() {
            2 => {
                self.logger.info(self, INFO_LOAD_FILE_CONFIG, false)?;
                self.config_server
                    .load_config_server_with_path(argv[1].as_str(), self.get_logger())?;
                Ok(())
            }
            1 => {
                self.logger
                    .info(self, INFO_LOAD_FILE_CONFIG_DEFAULT, false)?;
                self.config_server.load_config_server(self.get_logger())?;
                Ok(())
            }
            _ => self.logger.info(self, ERROR_COUNT_ARGS, false),
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

    pub fn get_server_port(&self) -> String {
        self.config_server.get_server_port(self.logger.clone())
    }

    pub fn get_verbose(&self) -> bool {
        self.config_server.get_verbose()
    }

    pub fn get_timeout(&self) -> u64 {
        self.config_server.get_timeout()
    }
}
