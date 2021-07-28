//! Removes elements from the set given by its key.
//!
//! Example:
//! ```text
//! > srem set_key value1 value2
//! 2
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::TYPE_SET;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_COMMAND: &str = "Run command SREM\n";

/// Name of the command.
const CLIENT_ID: &str = "SremCommand";

/// Code of the command.
const CONST_CMD: &str = "srem";

/// Min amount of arguments besides the command.
const MIN_VALID_ARGS: i32 = 2;

/// Max amount of arguments besides the command.
const MAX_VALID_ARGS: i32 = -1;

/// Main struct of the command.
pub struct SremCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl SremCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for SremCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for SremCommand {
    fn clone(&self) -> SremCommand {
        SremCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for SremCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let key = args[0];
        app_info.get_db_resolver().valid_key_type(key, TYPE_SET)?;

        let db = app_info.get_set_db_sharding(key);

        //TODO: verificar si la clave encontrada no es de otro tipo. Si lo es levantar Error

        let mut items_removed = db.srem(args).to_string();

        items_removed.push('\n');

        Ok(items_removed)
    }
}
