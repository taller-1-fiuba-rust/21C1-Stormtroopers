//! If the list exits, then insert all given values at the end.
//! 
//! Example:
//! ```text
//! > lrange lkey 0 -1
//! 0) value1
//! > rpushx lkey value2
//! 2
//! > lrange lkey 0 -1
//! 0) value1
//! 1) value2
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::LINE_BREAK;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_COMMAND: &str = "Run command RPUSHX\n";

/// Name of the command.
const CLIENT_ID: &str = "RpushxCommand";

/// Code of the command.
const RPUSHX_CMD: &str = "rpushx";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 2;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = -1;

pub struct RpushxCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl RpushxCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(RPUSHX_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for RpushxCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for RpushxCommand {
    fn clone(&self) -> RpushxCommand {
        RpushxCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for RpushxCommand {
    fn run(
        &self,
        mut args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let key = args.remove(0);
        let db = app_info.get_list_db_sharding(key);

        let res = db.rpushx(key.to_string(), args)?;

        let mut result = res.to_string();
        result.push(LINE_BREAK);

        Ok(result)
    }
}
