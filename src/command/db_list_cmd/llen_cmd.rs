//! Returns the length of the list.
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::{LINE_BREAK, TYPE_LIST};
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command LLEN\n";
const CLIENT_ID: &str = "LLenCommmand";
const LLEN_CMD: &str = "llen";

const MIN_VALID_ARGS: i32 = 1;
const MAX_VALID_ARGS: i32 = 1;

pub struct LLenCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl LLenCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(LLEN_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for LLenCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for LLenCommand {
    fn clone(&self) -> LLenCommand {
        LLenCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for LLenCommand {
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
        app_info.get_db_resolver().valid_key_type(key, TYPE_LIST)?;

        let db = app_info.get_list_db_sharding(key);

        let result_aux = db.llen(key.to_string())?;
        let mut result = result_aux.to_string();
        result.push(LINE_BREAK);

        Ok(result)
    }
}
