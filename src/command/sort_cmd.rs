use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};
use crate::LINE_BREAK;

const INFO_DBSIZE_COMMAND: &str = "Run command SORT\n";
const CLIENT_ID: &str = "   SortCommmand";

pub struct SortCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl SortCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> SortCommand {
        SortCommand { id_job, logger }
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

        let db = app_info.get_db_resolver();
        let response = db.sort(args[0].to_string())?;
        let mut response_str = "".to_string();
        //response.push(LINE_BREAK);
        for elem in response {
            response_str.push_str(&elem);
            response_str.push(LINE_BREAK);
        }
        Ok(response_str)
    }
}
