//! Returns the data type asociated to a key.
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::LINE_BREAK;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command TYPE\n";
const CLIENT_ID: &str = "TypeCommand";
const CONST_CMD: &str = "type";

const MIN_VALID_ARGS: i32 = 1;
const MAX_VALID_ARGS: i32 = 1;

pub struct TypeCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl TypeCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for TypeCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for TypeCommand {
    fn clone(&self) -> TypeCommand {
        TypeCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for TypeCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let db = app_info.get_db_resolver();

        let mut response = db.type_key(args[0].to_string())?;
        response.push(LINE_BREAK);

        Ok(response)
    }
}
