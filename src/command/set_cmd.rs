use crate::logger::{Logger, Loggable};
use crate::structure_string::StructureString;
use crate::errors::run_error::RunError;
use crate::command::cmd_trait::Command;

pub struct SetCommand {
    logger: Logger<String>,
    structure: StructureString<String>,
    id_job: u32,
}

impl SetCommand {
    pub fn new(id_job: u32, logger: Logger<String>, structure: StructureString<String>) -> SetCommand {
        SetCommand {  id_job,logger,structure }
    }
}

impl Command for SetCommand {
    fn run(&self, _args: Vec<&str>) -> Result<String, RunError> {
        self.logger.info(self, "Run command SET\n");
        return Ok(String::from("OK\n"));
    }
}

impl Loggable for SetCommand {
    fn get_id_client(&self) -> &str {
        "SetCommand"
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job.clone()
    }
}