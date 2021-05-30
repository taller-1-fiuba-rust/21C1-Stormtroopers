use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;

pub static PING_COMMAND_STR: &str = "ping";

pub struct PingCommand {
    logger: Logger<String>,
    id_job: u32,
}

impl PingCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> PingCommand {
        PingCommand {  id_job,logger }
    }
}
impl Command for PingCommand {
    fn run(&self, _args: Vec<&str>) -> Result<String, RunError> {
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

#[test]
    fn test_ping_command_return() {
    let log = Logger::new(
        String::from(""),
        "".to_string(),
    )
        .unwrap();
        let ping = PingCommand::new(0,log);
        assert_eq!(Command::run(&ping, vec!("")), Ok(String::from("PONG")));
    }