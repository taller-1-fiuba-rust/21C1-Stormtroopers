//! Returns the elements contained in the List or Set asociated to the given key, sorted.
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::LINE_BREAK;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_DBSIZE_COMMAND: &str = "Run command SORT\n";
const CLIENT_ID: &str = "   SortCommmand";
const CONST_CMD: &str = "sort";

const MIN_VALID_ARGS: i32 = 1;
const MAX_VALID_ARGS: i32 = 1;

pub struct SortCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl SortCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for SortCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for SortCommand {
    fn clone(&self) -> SortCommand {
        SortCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for SortCommand {
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

        let db = app_info.get_db_resolver();
        let response = db.sort(args[0].to_string())?;
        let mut response_str = "".to_string();

        for (i, elem) in response.iter().enumerate() {
            response_str.push_str(&format!("{}) ", i + 1));
            response_str.push_str(&elem);
            response_str.push(LINE_BREAK);
        }

        Ok(response_str)
    }
}
