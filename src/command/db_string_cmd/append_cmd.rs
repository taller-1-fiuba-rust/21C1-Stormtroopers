use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command APPEND\n";
const CLIENT_ID: &str = "AppendCommmand";

pub struct AppendCommmand {
    id_job: u32,
    logger: Logger<String>,
}

impl AppendCommmand {
    pub fn new(id_job: u32, logger: Logger<String>) -> AppendCommmand {
        AppendCommmand { id_job, logger }
    }
}

impl Loggable for AppendCommmand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for AppendCommmand {
    fn clone(&self) -> AppendCommmand {
        AppendCommmand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for AppendCommmand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_string_db();

        let mut len_val_str = db
            .append(String::from(args[0]), String::from(args[1]))
            .to_string();
        len_val_str.push('\n');

        Ok(len_val_str)
    }
}
