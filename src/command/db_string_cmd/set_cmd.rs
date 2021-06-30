use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_RUN_COMMAND: &str = "Run command SET\n";
const CLIENT_ID: &str = "SetCommand";
const RESPONSE_COMMAND: &str = "OK\n";

pub struct SetCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl SetCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> SetCommand {
        SetCommand { id_job, logger }
    }
}

impl Loggable for SetCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for SetCommand {
    fn clone(&self) -> SetCommand {
        SetCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for SetCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self
            .logger
            .info(self, INFO_RUN_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        let db_resolver = app_info.get_db_resolver();

        db_resolver.clear_key(String::from(args[0]));

        db_resolver
            .get_string_db()
            .set_string(String::from(args[0]), String::from(args[1]));

        Ok(String::from(RESPONSE_COMMAND))
    }
}
