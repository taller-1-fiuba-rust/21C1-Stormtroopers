//! Removes from the list the given amount of ocurrences of the given value.
//!
//! Example:
//! ```text
//! > lrange key 0 -1
//! 0) 0
//! 1) 1
//! 2) 2
//! 3) 1
//! 4) 1
//! 5) 1
//! > lrem key 1 1
//! 1
//! > lrange key 0 -1
//! 0) 0
//! 1) 2
//! 2) 1
//! 3) 1
//! 4) 1
//! > lrem key 1 0
//! 3
//! > lrange key 0 -1
//! 0) 0
//! 1) 2
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::{LINE_BREAK, TYPE_LIST};
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_COMMAND: &str = "Run command LREM\n";

/// Name of the command.
const CLIENT_ID: &str = "LremCommand";

/// Code of the command.
const LREM_CMD: &str = "lrem";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 3;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = 3;

/// Main struct of the command.
pub struct LremCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl LremCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(LREM_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for LremCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for LremCommand {
    fn clone(&self) -> LremCommand {
        LremCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for LremCommand {
    fn run(
        &self,
        mut args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let key = args[0];
        args.remove(0);
        app_info.get_db_resolver().valid_key_type(key, TYPE_LIST)?;

        let db = app_info.get_list_db_sharding(key);
        let mut result = db
            .lrem(key.to_string(), args[0].to_string(), args[1].to_string())?
            .to_string();

        result.push(LINE_BREAK);

        Ok(result)
    }
}
