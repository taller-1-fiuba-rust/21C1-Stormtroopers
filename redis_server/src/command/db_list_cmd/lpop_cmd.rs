//! Returns and removes the first element/s of the list, given by the user.
//!
//! Example:
//! ```text
//! > lrange key 0 -1
//! 0) value1
//! 1) value2
//! 2) value3
//! 3) value4
//! > lpop key
//! "value1"
//! > lrange key 0 -1
//! 0) value2
//! 1) value3
//! 2) value4
//! lpop key 2
//! 0) value2
//! 1) value3
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::TYPE_LIST;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_COMMAND: &str = "Run command LPOP\n";

/// Name of the command.
const CLIENT_ID: &str = "LpopCommand";
const RESPONSE_EMPTY: &str = "(nil)\n";
const LPOP_CMD: &str = "lpop";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 1;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = 2;

/// Main struct of the command.
pub struct LpopCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl LpopCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(LPOP_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for LpopCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for LpopCommand {
    fn clone(&self) -> LpopCommand {
        LpopCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for LpopCommand {
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
        let db = app_info.get_list_db_sharding(key);

        app_info.get_db_resolver().valid_key_type(key, TYPE_LIST)?;

        let items = db.lpop(args)?;

        if items.is_empty() {
            return Ok(String::from(RESPONSE_EMPTY));
        }

        let mut to_return = String::from("");
        for (i, it) in items.iter().enumerate() {
            to_return.push_str(&format!("{}) {}\n", i, it.clone()));
        }

        Ok(to_return)
    }
}
