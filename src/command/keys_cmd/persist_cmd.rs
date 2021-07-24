//! Removes the expiration time of the given key.
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::RESPONSE_SIMPLE_STRING;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_EXPIRE_COMMAND: &str = "Run command TTL\n";
const CLIENT_ID: &str = "PersistCommmand";
const CONST_CMD: &str = "persist";

const MIN_VALID_ARGS: i32 = 1;
const MAX_VALID_ARGS: i32 = 1;

pub struct PersistCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl PersistCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for PersistCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for PersistCommand {
    fn clone(&self) -> PersistCommand {
        PersistCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for PersistCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let _log_info_res = self
            .logger
            .info(self, INFO_EXPIRE_COMMAND, app_info.get_verbose());

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let key_str = args[0]; // The key for the DB
        let db = app_info.get_db_resolver();

        match db.type_key(String::from(key_str)) {
            Ok(_db_type) => {
                let ttl_scheduler = app_info.get_ttl_scheduler();
                match ttl_scheduler.delete_ttl_key(String::from(key_str)) {
                    Ok(key) => {
                        ttl_scheduler
                            .delete_ttl(key)
                            .unwrap_or_else(|_| vec![String::from("")]);
                        Ok(RESPONSE_SIMPLE_STRING.to_string())
                    }
                    Err(_) => Ok(RESPONSE_SIMPLE_STRING.to_string()),
                }
            }
            Err(e) => Err(e),
        }
    }
}
