//! Get the value of the String given by the key.
//!
//! Example:
//! ```text
//! > set key value
//! OK
//! > get key
//! "value"
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::LINE_BREAK;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_RUN_COMMAND: &str = "Run command GET\n";

/// Name of the command.
const CLIENT_ID: &str = "GetCommand";

/// Code of the command.
const CONST_CMD: &str = "get";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 1;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = 1;

/// Main struct of the command.
pub struct GetCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl GetCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for GetCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for GetCommand {
    fn clone(&self) -> GetCommand {
        GetCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for GetCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self
            .logger
            .info(self, INFO_RUN_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;
        //let db = app_info.get_string_db();
        let key = args[0];
        let db = app_info.get_string_db_sharding(key);

        let mut string = db.get_string(key.to_string());
        string.push(LINE_BREAK);

        Ok(string)
    }
}
