//! Adds element to the given set, or creates a new one.
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::{LINE_BREAK, TYPE_SET};
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command SADD\n";
const CLIENT_ID: &str = "SAddCommmand";
const CONST_CMD: &str = "sadd";

const MIN_VALID_ARGS: i32 = 2;
const MAX_VALID_ARGS: i32 = -1;

pub struct SaddCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl SaddCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for SaddCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for SaddCommand {
    fn clone(&self) -> SaddCommand {
        SaddCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for SaddCommand {
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
        app_info.get_db_resolver().valid_key_type(key, TYPE_SET)?;

        let db = app_info.get_set_db_sharding(key);

        let mut result = db.sadd(args).to_string();
        result.push(LINE_BREAK);

        Ok(result)
    }
}
