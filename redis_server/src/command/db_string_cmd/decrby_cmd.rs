//! If the data is a string that can be parsed as a number, it decrements it by the amount entered.
//!
//! Example:
//! ```text
//! > set key 1
//! OK
//! > decrby key 2
//! -1
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::{LINE_BREAK, TYPE_STRING};
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_DBSIZE_COMMAND: &str = "Run command DECRBY\n";

/// Name of the command.
const CLIENT_ID: &str = "DecrbyCommand";

/// Code of the command.
const CONST_CMD: &str = "decrby";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 1;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = 2;

/// Main struct of the command.
pub struct DecrbyCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl DecrbyCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for DecrbyCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for DecrbyCommand {
    fn clone(&self) -> DecrbyCommand {
        DecrbyCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for DecrbyCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self
            .logger
            .info(self, INFO_DBSIZE_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let key = args[0];
        app_info
            .get_db_resolver()
            .valid_key_type(key, TYPE_STRING)?;

        let db = app_info.get_string_db_sharding(key);

        let rsp = db.decrby(key.to_string(), args[1].to_string())?;
        let mut response = rsp.to_string();
        response.push(LINE_BREAK);
        Ok(response)
    }
}
