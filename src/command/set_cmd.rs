use crate::logger::{Logger, Loggable};
use crate::structure_string2::StructureString;
use crate::errors::run_error::RunError;
use crate::command::cmd_trait::Command;
use crate::structure_simple::{StructureSimple, set_string};

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

const INFO_RUN_COMMAND: &str = "Run command SET\n";
const CLIENT_ID: &str = "SetCommand";
const RESPONSE_COMMAND: &str = "OK\n";

pub struct SetCommand {
    logger: Logger<String>,
    //structure: Box<StructureString<String>>,
    id_job: u32,
}

impl SetCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> SetCommand {
        SetCommand {  id_job, logger }
    }
}

impl Loggable for SetCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job.clone()
    }
}

impl Command for SetCommand {
    fn run(&self, args: Vec<&str>, structure: & Arc<Mutex<HashMap<String,String>>>) -> Result<String, RunError> {
        self.logger.info(self, INFO_RUN_COMMAND);
        println!("setcommand::{},{}",args[0],args[1]);
        //structure.set_string(String::from(args[0]),String::from(args[1]));

        set_string(structure, String::from(args[0]),String::from(args[1]));

        return Ok(String::from(RESPONSE_COMMAND));
    }
}