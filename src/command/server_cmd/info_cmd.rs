use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};
use std::process;

const INFO_RUN_COMMAND: &str = "Run command INFO\n";
const CLIENT_ID: &str = "InfoCommand";
const CONST_CMD: &str = "info";

const MIN_VALID_ARGS: i32 = 0;
const MAX_VALID_ARGS: i32 = 0;

pub struct InfoCommand {
    id_job: u32,
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
