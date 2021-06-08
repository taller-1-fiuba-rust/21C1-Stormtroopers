use crate::app_info::AppInfo;
use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};
use crate::structure_general::get_string;

const INFO_COMMAND: &str = "Run command STRLEN\n";
const CLIENT_ID: &str = "StrlenCommmand";

pub struct StrlenCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl StrlenCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> StrlenCommand {
        StrlenCommand { id_job, logger }
    }
}

impl Loggable for StrlenCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for StrlenCommand {
    fn clone(&self) -> StrlenCommand {
        StrlenCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for StrlenCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        let structure_general = app_info.get_structure();
        let structure = structure_general.get("String".to_string());

        let structure = get_string(structure)?;
        let mut len = structure.strlen(String::from(args[0])).to_string();
        len.push('\n');

        Ok(len)
    }
}
