//! Returns the range of values in the list given by the indexes.
//!
//! Example:
//! ```text
//! > lpush key value1 value2 value3
//! 3
//! > lrange key 0 -1
//! 0) value1
//! 1) value2
//! 2) value3
//! > lrange key 1 1
//! 0) value2
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::TYPE_LIST;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_COMMAND: &str = "Run command LRANGE\n";

/// Name of the command.
const CLIENT_ID: &str = "LrangeCommand";

/// Response string when the list is empty.
const RESPONSE_EMPTY: &str = "(empty list or set)\n";

/// Code of the command.
const LRANGE_CMD: &str = "lrange";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 3;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = 3;

/// Main struct of the command.
pub struct LrangeCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl LrangeCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(LRANGE_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for LrangeCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for LrangeCommand {
    fn clone(&self) -> LrangeCommand {
        LrangeCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for LrangeCommand {
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
        app_info.get_db_resolver().valid_key_type(key, TYPE_LIST)?;

        let db = app_info.get_list_db_sharding(key);
        args.remove(0);

        let items = db.lrange(key.to_string(), args)?;

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
