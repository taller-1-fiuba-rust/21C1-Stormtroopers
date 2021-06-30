use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};
use crate::LINE_BREAK;

const INFO_DBSIZE_COMMAND: &str = "Run command DBSIZE\n";
const CLIENT_ID: &str = "DbSizeCommmand";

pub struct DbsizeCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl DbsizeCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> DbsizeCommand {
        DbsizeCommand { id_job, logger }
    }
}

impl Loggable for DbsizeCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for DbsizeCommand {
    fn clone(&self) -> DbsizeCommand {
        DbsizeCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for DbsizeCommand {
    fn run(
        &self,
        _args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self
            .logger
            .info(self, INFO_DBSIZE_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_db_resolver();
        let size = db.dbsize();
        let mut res_str = size.to_string();
        res_str.push(LINE_BREAK);
        Ok(res_str)
    }
}
