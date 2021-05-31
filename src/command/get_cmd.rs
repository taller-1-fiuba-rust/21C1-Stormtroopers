use crate::logger::{Logger, Loggable};
use crate::structure_string::StructureString;
use crate::errors::run_error::RunError;
use crate::command::cmd_trait::Command;


const INFO_RUN_COMMAND: &str = "Run command GET\n";
const CLIENT_ID: &str = "SetCommand";
const RESPONSE_COMMAND: &str = "OK\n";

pub struct GetCommand {
    logger: Logger<String>,
    //structure: Box<StructureString<String>>,
    id_job: u32,
}

impl GetCommand {
    pub fn new<'a>(id_job: u32, logger: Logger<String>) -> GetCommand {
        GetCommand {  id_job, logger }
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

impl Command for GetCommand {
    fn run(&self, args: Vec<&str>, structure: &mut Box<StructureString<String>>) -> Result<String, RunError> {
        self.logger.info(self, INFO_RUN_COMMAND);

        Ok(structure.get_string(String::from(args[0])))
    }
}