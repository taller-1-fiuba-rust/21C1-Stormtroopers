//! Returns the length of the string given a key.
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::LINE_BREAK;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_COMMAND: &str = "Run command STRLEN\n";

/// Name of the command.
const CLIENT_ID: &str = "StrlenCommmand";
const CONST_CMD: &str = "strlen";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 1;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = 1;

pub struct StrlenCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl StrlenCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for StrlenCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for StrlenCommand {
    fn clone(&self) -> StrlenCommand {
        StrlenCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for StrlenCommand {
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
        let db = app_info.get_string_db_sharding(key);
        let mut len = db.strlen(key.to_string()).to_string();
        len.push(LINE_BREAK);

        Ok(len)
    }
}
