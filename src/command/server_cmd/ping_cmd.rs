use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_PING_COMMAND: &str = "Run command PING\n";
const RESPONSE_PING_COMMAND: &str = "PONG\n";
const CLIENT_ID: &str = "PingCommand";
const CONST_CMD: &str = "ping";

pub struct PingCommand {
    id_job: u32,
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
