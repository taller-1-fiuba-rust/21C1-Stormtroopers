//! Returns the element at the given index.
//!
//! Example:
//! ```text
//! > lrange key 0 -1
//! 0) value1
//! 1) value2
//! 2) value3
//! 3) value4
//! > lindex key 0
//! value1
//! > lindex key 2
//! value3
//! > lindex key 4
//! Error running command: Position is bigger than the len of the list
//! Cause: The argument exceeds the limits of the list
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::{LINE_BREAK, TYPE_LIST};
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_COMMAND: &str = "Run command LINDEX\n";

/// Name of the command.
const CLIENT_ID: &str = "LindexCommand";

/// Code of the command.
const LINDEX_CMD: &str = "lindex";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 2;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = 2;

/// Main struct of the command.
pub struct LindexCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl LindexCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(LINDEX_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for LindexCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for LindexCommand {
    fn clone(&self) -> LindexCommand {
        LindexCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for LindexCommand {
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
        let idx = args[1];
        app_info.get_db_resolver().valid_key_type(key, TYPE_LIST)?;

        let db = app_info.get_list_db_sharding(key);

        let mut result = db.lindex(key.to_string(), idx.to_string())?;
        result.push(LINE_BREAK);

        Ok(result)
    }
}
