//! Given the index and a value, it sets the value at the index position of the list.
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::{LINE_BREAK, TYPE_LIST};
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_COMMAND: &str = "Run command LSET\n";

/// Name of the command.
const CLIENT_ID: &str = "LsetCommand";

/// Code of the command
const LSET_CMD: &str = "lset";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 3;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = 3;

pub struct LsetCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl LsetCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(LSET_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for LsetCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for LsetCommand {
    fn clone(&self) -> LsetCommand {
        LsetCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for LsetCommand {
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
        let idx = args[1];
        let value = args[2];

        app_info.get_db_resolver().valid_key_type(key, TYPE_LIST)?;

        let db = app_info.get_list_db_sharding(key);
        let mut result = db.lset(key.to_string(), idx.to_string(), value.to_string())?;
        result.push(LINE_BREAK);

        Ok(result)
    }
}
