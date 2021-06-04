use crate::app_info::AppInfo;
use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};

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
    fn run(&self, args: Vec<&str>, app_info: &AppInfo) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        let structure = app_info.get_structure();
        let key = String::from(args[0]);
        let mut value = structure.get_string(key.clone());
        let val_append = String::from(args[1]);
        let mut len_value_str;
        if value == *"EMPTY_STRING" {
            value = val_append.clone();
            structure.set_string(key, val_append);
        } else {
            value.push_str(args[1]);
            structure.set_string(key, value.clone());
        }
        len_value_str = value.chars().count().to_string();
        len_value_str.push('\n');
        Ok(len_value_str)
    }
}
