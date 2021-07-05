use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::{LINE_BREAK, TYPE_LIST};
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command RPUSH\n";
const CLIENT_ID: &str = "RpushCommand";
const RPUSH_CMD: &str = "rpush";

const MIN_VALID_ARGS: i32 = 2;
const MAX_VALID_ARGS: i32 = -1;

pub struct RpushCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl RpushCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(RPUSH_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for RpushCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for RpushCommand {
    fn clone(&self) -> RpushCommand {
        RpushCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for RpushCommand {
    fn run(
        &self,
        mut args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let key = args.remove(0);
        app_info.get_db_resolver().valid_key_type(key, TYPE_LIST)?;

        let db = app_info.get_list_db_sharding(key);

        let mut result = db.rpush(key.to_string(), args)?;
        result.push(LINE_BREAK);

        Ok(result)
    }
}
