//! Copies the contents of one key to another passed.
//!
//! Example:
//! ```text
//! > set key value
//! OK
//! > copy key other
//! OK
//! > get other
//! value
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_COMMAND: &str = "Run command COPY\n";

/// Name of the command.
const CLIENT_ID: &str = "CopyCommand";

// Code of the command.
const CONST_CMD: &str = "copy";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 2;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = 2;

/// Main struct of the command.
pub struct CopyCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl CopyCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for CopyCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for CopyCommand {
    fn clone(&self) -> CopyCommand {
        CopyCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

//TODO: ver thread safety impl
impl Command for CopyCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let src_key = args[0];
        let target_key = args[1];

        let res =
            app_info
                .get_db_resolver()
                .copy(src_key, target_key, app_info.get_ttl_scheduler())?;
        Ok(res)
    }
}
