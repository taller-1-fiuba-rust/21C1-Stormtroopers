use crate::app_info::AppInfo;
use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};
use crate::structure_general::get_string;

const INFO_COMMAND: &str = "Run command DEL\n";
const CLIENT_ID: &str = "DelCommmand";

pub struct DelCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl DelCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> DelCommand {
        DelCommand { id_job, logger }
    }
}

impl Loggable for DelCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for DelCommand {
    fn clone(&self) -> DelCommand {
        DelCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for DelCommand {
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

        let mut structure = get_string(structure)?;
        let mut result_del = structure.delete(args).to_string();
        result_del.push('\n');

        Ok(result_del)
    }
}
