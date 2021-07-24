//! Gets the string value and then deletes it.
//! 
//! Example:
//! ```text
//! > set key "Hello World."
//! OK
//! > getdel key
//! "Hello World."
//! > get key
//! (nil)
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::LINE_BREAK;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_COMMAND: &str = "Run command GETDEL\n";

/// Name of the command.
const CLIENT_ID: &str = "GetdelCommmand";

/// Code of the command.
const CONST_CMD: &str = "getdel";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 1;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = 1;

/// Main structure of the command.
pub struct GetDelCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl GetDelCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for GetDelCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for GetDelCommand {
    fn clone(&self) -> GetDelCommand {
        GetDelCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for GetDelCommand {
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
        let mut db = app_info.get_string_db_sharding(key);

        let mut result = db.get_del(key.to_string())?;
        result.push(LINE_BREAK);

        Ok(result)
    }
}
