use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command GETDEL\n";
const CLIENT_ID: &str = "GetdelCommmand";

pub struct GetDelCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl GetDelCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> GetDelCommand {
        GetDelCommand { id_job, logger }
    }
}

impl Loggable for GetDelCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for GetDelCommand {
    fn clone(&self) -> GetDelCommand {
        GetDelCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for GetDelCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        let mut db = app_info.get_string_db();

        let mut result = db.get_del(args[0].to_string())?;
        result.push('\n');

        Ok(result)
    }
}
