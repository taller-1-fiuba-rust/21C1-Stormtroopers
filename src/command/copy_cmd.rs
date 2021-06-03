use crate::app_info::AppInfo;
use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command COPY\n";
const CLIENT_ID: &str = "CopyCommmand";

pub struct CopyCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl CopyCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> CopyCommand {
        CopyCommand { id_job, logger }
    }
}

impl Loggable for CopyCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

//TODO: ver por que si vienen varias claves hay que borrar todas
impl Command for CopyCommand {
    fn run(&self, _args: Vec<&str>, _app_info: &AppInfo) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        let structure = app_info.get_structure();
        let resp = structure.copy(_args[0],_args[1]);

        Ok(resp.to_string())
    }
}
