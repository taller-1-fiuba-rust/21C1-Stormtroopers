//! Changes the name of the key, keeping the ttl unchanged.
//!
//! Example:
//! ```text
//! > set key value
//! OK
//! > expire key 60
//! OK
//! > rename key other
//! OK
//! > get other
//! value
//! > ttl other
//! 60
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::RESPONSE_SIMPLE_STRING;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_COMMAND: &str = "Run command RENAME\n";

/// Name of the command.
const CLIENT_ID: &str = "RenameCommand";

/// Code of the command.
const CONST_CMD: &str = "rename";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 2;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = 2;

/// Main struct of the command.
pub struct RenameCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl RenameCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for RenameCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for RenameCommand {
    fn clone(&self) -> RenameCommand {
        RenameCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for RenameCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let key_src = args[0];
        let key_target = args[1];

        app_info
            .get_db_resolver()
            .rename(key_src, key_target, app_info.get_ttl_scheduler())?;
        Ok(RESPONSE_SIMPLE_STRING.to_string())
    }
}
