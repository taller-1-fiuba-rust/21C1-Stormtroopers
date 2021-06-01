use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};
use crate::structure_string::StructureString;
//use crate::structure_simple::{StructureSimple, get_string};

use std::sync::Arc;

const INFO_RUN_COMMAND: &str = "Run command GET\n";
const CLIENT_ID: &str = "SetCommand";

pub struct GetCommand {
    logger: Logger<String>,
    id_job: u32,
}

impl GetCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> GetCommand {
        GetCommand { id_job, logger }
    }
}

impl Loggable for GetCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job.clone()
    }
}

impl Command for GetCommand {
    fn run(
        &self,
        args: Vec<&str>,
        structure: Arc<StructureString<String>>,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_RUN_COMMAND);
        match log_info_res {
            Ok(_r) => {}
            _ => {}
        }

        let mut string = structure.get_string(String::from(args[0]));
        string.push_str("\n");

        Ok(string)
    }
}
