//! This is to get a simple response from the server, useful to test if the connection is alive.
//!
//! Example:
//! ```text
//! > ping
//! PONG
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_PING_COMMAND: &str = "Run command PING\n";

/// Response of the command.
const RESPONSE_PING_COMMAND: &str = "PONG\n";

/// Name of the command.
const CLIENT_ID: &str = "PingCommand";

/// Code of the command.
const CONST_CMD: &str = "ping";

/// Min amount of arguments besides the command name.
const MIN_VALID_ARGS: i32 = 0;

/// Max amount of arguments besides the command name.
const MAX_VALID_ARGS: i32 = 0;

/// Main structure of the command.
pub struct PingCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl PingCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Clone for PingCommand {
    fn clone(&self) -> PingCommand {
        PingCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for PingCommand {
    fn run(
        &self,
        _args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let _log_info_res = self
            .logger
            .info(self, INFO_PING_COMMAND, app_info.get_verbose());

        ParsedMessage::validate_args(_args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        Ok(String::from(RESPONSE_PING_COMMAND))
    }
}

impl Loggable for PingCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}
