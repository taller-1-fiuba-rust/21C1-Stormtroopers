//! Copies the contents of one key to another passed.
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::LINE_BREAK;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command COPY\n";
const CLIENT_ID: &str = "CopyCommand";
const CONST_CMD: &str = "copy";

const MIN_VALID_ARGS: i32 = 2;
const MAX_VALID_ARGS: i32 = 2;

pub struct CopyCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl CopyCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for CopyCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for CopyCommand {
    fn clone(&self) -> CopyCommand {
        CopyCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

//TODO: ver thread safety impl
impl Command for CopyCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let src_key = args[0];
        let target_key = args[1];

        let mut res = app_info
            .get_db_resolver()
            .copy(src_key, target_key, false, app_info.get_ttl_scheduler())?
            .to_string();
        res.push(LINE_BREAK);
        Ok(res)
    }
}
