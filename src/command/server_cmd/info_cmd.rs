//! Returns useful information of the system and the process running.
//!
//! Example:
//! ```text
//! > info
//! -> connected_clients: 1
//! -> tcp_port: 127.0.0.1:8081
//! -> process_id: 4872
//! -> max_clients: 2
//! -> uptime: 181
//! -> uptime_days: 0
//! -> actives_threads: 2
//! -> Config Server:
//! 0) "port": "8081"
//! 1) "timeout": "600"
//! 2) "server": "127.0.0.1"
//! 3) "verbose": "false"
//! 4) "dbfilename": "dump.rdb"
//! 5) "logfile": "redis.log"
//! 6) "sharing_count": "2"
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};
use std::process;

/// Information string to log.
const INFO_RUN_COMMAND: &str = "Run command INFO\n";

/// Name of the command.
const CLIENT_ID: &str = "InfoCommand";

/// Code of the command.
const CONST_CMD: &str = "info";

/// Min amount of arguments besides the command name.
const MIN_VALID_ARGS: i32 = 0;

/// Max amount of arguments besides the command name.
const MAX_VALID_ARGS: i32 = 0;

/// Main structure of the command.
pub struct InfoCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl InfoCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for InfoCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for InfoCommand {
    fn clone(&self) -> InfoCommand {
        InfoCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for InfoCommand {
    fn run(
        &self,
        mut _args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self
            .logger
            .info(self, INFO_RUN_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(_args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let response = app_info.system_info(process::id() as usize);

        Ok(response)
    }
}
