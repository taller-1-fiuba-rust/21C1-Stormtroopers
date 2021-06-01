use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};
use crate::structure_string::StructureString;

use std::sync::Arc;

pub struct PingCommand {
    logger: Logger<String>,
    id_job: u32,
}

impl PingCommand {
    pub fn new<'a>(id_job: u32, logger: Logger<String>) -> PingCommand {
        PingCommand { id_job, logger }
    }
}
impl Command for PingCommand {
    fn run(
        &self,
        _args: Vec<&str>,
        _structure: Arc<StructureString<String>>,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, "Run command PING\n");

        match log_info_res {
            _ => {}
        }

        return Ok(String::from("PONG\n"));
    }
}

impl Loggable for PingCommand {
    fn get_id_client(&self) -> &str {
        "PingCommand"
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job.clone()
    }
}

#[cfg(test)]
#[test]
fn test_ping_command_return() {
    let log = Logger::new(String::from("log"), "./".to_string()).unwrap();
    let arc_structure = Arc::new(StructureString::new());

    let ping = PingCommand::new(0, log);
    assert_eq!(
        Command::run(&ping, vec!(""), arc_structure),
        Ok(String::from("PONG\n"))
    );
}
