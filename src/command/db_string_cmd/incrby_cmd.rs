//! If the data is a string that can be parsed as a number, it increments it by the amount entered.
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::{LINE_BREAK, TYPE_STRING};
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_DBSIZE_COMMAND: &str = "Run command INCRBY\n";
const CLIENT_ID: &str = "DecrbyCommmand";
const CONST_CMD: &str = "incrby";

const MIN_VALID_ARGS: i32 = 2;
const MAX_VALID_ARGS: i32 = 2;

pub struct IncrbyCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl IncrbyCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for IncrbyCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for IncrbyCommand {
    fn clone(&self) -> IncrbyCommand {
        IncrbyCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for IncrbyCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self
            .logger
            .info(self, INFO_DBSIZE_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let key = args[0];
        app_info
            .get_db_resolver()
            .valid_key_type(key, TYPE_STRING)?;

        let db = app_info.get_string_db_sharding(key);

        let rsp = db.incrby(key.to_string(), args[1].to_string())?;
        let mut response = rsp.to_string();
        response.push(LINE_BREAK);
        Ok(response)
    }
}
