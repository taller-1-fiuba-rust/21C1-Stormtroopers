use crate::logger::{Logger, Loggable};
use crate::structure_string::StructureString;
use crate::errors::run_error::RunError;
use crate::command::cmd_trait::Command;

pub struct GetCommand {
    logger: Logger<String>,
    structure: StructureString<String>,
    id_job: u32,
}

impl GetCommand {
    pub fn new(id_job: u32, logger: Logger<String>, structure: StructureString<String>) -> GetCommand {
        GetCommand {  id_job,logger,structure }
    }
}

impl Command for GetCommand {
    fn run(&self, _args: Vec<&str>) -> Result<String, RunError> {
        self.logger.info(self, "Run command GET\n");
        return Ok(String::from("OK\n"));
    }
}

impl Loggable for GetCommand {
    fn get_id_client(&self) -> &str {
        "GetCommand"
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job.clone()
    }
}