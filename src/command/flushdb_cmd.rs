use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};
use crate::app_info::AppInfo;
use crate::RESP_SIMPLE_STRING;

const INFO_FLUSHDB_COMMAND: &str = "Run command FLUSHDB\n";
const CLIENT_ID: &str = "FlushdbCommand";

pub struct FlushdbCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl FlushdbCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> FlushdbCommand {
        FlushdbCommand { id_job, logger }
    }
}

impl Loggable for FlushdbCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Command for FlushdbCommand {
    fn run(&self, _args: Vec<&str>, app_info: &AppInfo) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_FLUSHDB_COMMAND);
        if let Ok(_r) = log_info_res {}

        let structure = app_info.get_structure();
        let res = match structure.clean_all_data(){
            true => String::from(RESP_SIMPLE_STRING),
            false => panic!("Esto no deberia pasar!"),
        };

        Ok(res)
    }
}
