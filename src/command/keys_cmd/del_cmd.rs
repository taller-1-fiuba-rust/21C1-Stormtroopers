//! Removes a key from the database.
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_COMMAND: &str = "Run command DEL\n";

/// Name of the command.
const CLIENT_ID: &str = "DelCommand";
const CONST_CMD: &str = "del";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 1;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = -1;

pub struct DelCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl DelCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for DelCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for DelCommand {
    fn clone(&self) -> DelCommand {
        DelCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for DelCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let db_resolver = app_info.get_db_resolver();

        let res = db_resolver.delete_keys(args);
        let mut result = res.to_string();

        result.push('\n');
        Ok(result)
    }
}
