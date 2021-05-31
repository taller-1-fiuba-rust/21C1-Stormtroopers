use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;

pub struct PingCommand {
    logger: Logger<String>,
    id_job: u32,
    //structure:Box<StructureString<String>>,
}

impl PingCommand{
    pub fn new<'a>(id_job: u32, logger: Logger<String>) -> PingCommand {
        PingCommand {  id_job, logger }
    }
}
impl Command for PingCommand {
    fn run(&self, _args: Vec<&str>, structure: &mut Box<StructureString<String>>) -> Result<String, RunError> {
        self.logger.info(self, "Run command PING\n");
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
    use super::*;
use crate::logger::{Logger, Loggable};
use crate::structure_string::StructureString;

#[test]
fn test_ping_command_return() {
    let log = Logger::new(
        String::from(""),
        "".to_string(),
    ).unwrap();
    let mut structure = Box::new(StructureString::new());

    let ping = PingCommand::new(0,log);
    assert_eq!(Command::run(&ping, vec!(""), &mut structure), Ok(String::from("PONG")));
}