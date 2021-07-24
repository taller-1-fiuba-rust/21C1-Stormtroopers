//! Inserts the given values at the beginning of the list, if it exists.
//! 
//! Example:
//! ```text
//! > lrange lkey 0 -1
//! 0) value1
//! > lpushx lkey value2
//! 2
//! > lrange lkey 0 -1
//! 0) value2
//! 1) value1
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::{LINE_BREAK, TYPE_LIST};
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_COMMAND: &str = "Run command LPUSHX\n";

/// Name of the command.
const CLIENT_ID: &str = "LpushxCommand";
const LPUSHX_CMD: &str = "lpushx";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 2;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = -1;

pub struct LpushxCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl LpushxCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(LPUSHX_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for LpushxCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for LpushxCommand {
    fn clone(&self) -> LpushxCommand {
        LpushxCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for LpushxCommand {
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
        app_info.get_db_resolver().valid_key_type(key, TYPE_LIST)?;

        let db = app_info.get_list_db_sharding(key);
        let mut result = db.lpushx(args).to_string();

        result.push(LINE_BREAK);

        Ok(result)
    }
}
