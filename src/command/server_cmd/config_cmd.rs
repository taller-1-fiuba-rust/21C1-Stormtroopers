//! Consists of two parts, get and set. This make this command to retrieve and modify some settings in redis.conf file.
use crate::command::cmd_trait::Command;

use crate::command::command_builder::CommandBuilder;
use crate::constants::LINE_BREAK;

use crate::command::command_parser::ParsedMessage;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_RUN_COMMAND: &str = "Run command CONFIG\n";
const CLIENT_ID: &str = "ConfigCommand";
const CONST_CMD: &str = "config";

const SUB_CMD_GET: &str = "get";
const SUB_CMD_SET: &str = "set";

const MIN_VALID_ARGS: i32 = 1;
const MAX_VALID_ARGS: i32 = 3;

pub struct ConfigCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl ConfigCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for ConfigCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for ConfigCommand {
    fn clone(&self) -> ConfigCommand {
        ConfigCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for ConfigCommand {
    fn run(
        &self,
        mut args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self
            .logger
            .info(self, INFO_RUN_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let config_server = app_info.get_config_server();
        let cmd = args.remove(0);

        let mut response: String;
        if cmd == SUB_CMD_GET {
            response = config_server.get();
        } else if cmd == SUB_CMD_SET {
            response = config_server.set(args[0].to_string(), args[1].to_string())?;
        } else {
            return Err(RunError {
                message: "Value is not allowed".to_string(),
                cause: "The value must be: get or set\n".to_string(),
            });
        }

        response.push(LINE_BREAK);
        Ok(response)
    }
}
