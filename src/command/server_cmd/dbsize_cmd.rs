//! Returns the amount of pairs key/value stored in the database.
//! 
//! Example:
//! ```text
//! > dbsize
//! 4
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::LINE_BREAK;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_DBSIZE_COMMAND: &str = "Run command DBSIZE\n";

/// Name of the command.
const CLIENT_ID: &str = "DbSizeCommand";

/// Key of the command.
const CONST_CMD: &str = "dbsize";

/// Min amount of arguments besides the command name.
const MIN_VALID_ARGS: i32 = 0;

/// Max amount of arguments besides the command name.
const MAX_VALID_ARGS: i32 = 0;

/// Main structure of the command.
pub struct DbSizeCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl DbSizeCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for DbSizeCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for DbSizeCommand {
    fn clone(&self) -> DbSizeCommand {
        DbSizeCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for DbSizeCommand {
    fn run(
        &self,
        _args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self
            .logger
            .info(self, INFO_DBSIZE_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(_args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let db = app_info.get_db_resolver();
        let size = db.dbsize();
        let mut res_str = size.to_string();
        res_str.push(LINE_BREAK);
        Ok(res_str)
    }
}
