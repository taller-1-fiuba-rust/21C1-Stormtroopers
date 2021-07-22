//! Wipes the entire database.
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::RESPONSE_SIMPLE_STRING;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_FLUSHDB_COMMAND: &str = "Run command FLUSHDB\n";
const CLIENT_ID: &str = "FlushdbCommand";
const CONST_CMD: &str = "flushdb";

const MIN_VALID_ARGS: i32 = 0;
const MAX_VALID_ARGS: i32 = 0;

pub struct FlushdbCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl FlushdbCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for FlushdbCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for FlushdbCommand {
    fn clone(&self) -> FlushdbCommand {
        FlushdbCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for FlushdbCommand {
    fn run(
        &self,
        _args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self
            .logger
            .info(self, INFO_FLUSHDB_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(_args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let db = app_info.get_db_resolver();
        let res = match db.clean_all_data() {
            true => String::from(RESPONSE_SIMPLE_STRING),
            false => panic!("Error borrando información de toda la base de datos"),
        };

        Ok(res)
    }
}
