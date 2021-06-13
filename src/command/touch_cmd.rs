use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command TOUCH\n";
const CLIENT_ID: &str = "TouchCommand";

pub struct TouchCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl TouchCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> TouchCommand {
        TouchCommand { id_job, logger }
    }
}

impl Loggable for TouchCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for TouchCommand {
    fn clone(&self) -> TouchCommand {
        TouchCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for TouchCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_db_resolver();

        let mut vec_args = Vec::<String>::new();
        for arg in args {
            vec_args.push(arg.to_string());
        }
        let mut response = db.touch(vec_args).to_string();
        response.push('\n');

        Ok(response)
    }
}
