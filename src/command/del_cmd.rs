use crate::command::cmd_trait::Command;
use crate::command::command_parser::ParsedMessage;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command DEL\n";
const CLIENT_ID: &str = "DelCommmand";

const MIN_VALID_ARGS: i32 = 1;
const MAX_VALID_ARGS: i32 = -1;

pub struct DelCommmand {
    id_job: u32,
    logger: Logger<String>,
}

impl DelCommmand {
    pub fn new(id_job: u32, logger: Logger<String>) -> DelCommmand {
        DelCommmand { id_job, logger }
    }
}

impl Loggable for DelCommmand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for DelCommmand {
    fn clone(&self) -> DelCommmand {
        DelCommmand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for DelCommmand {
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
