use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;

pub static PING_COMMAND_STR: &str = "ping";

pub struct PingCommand;

impl Command for PingCommand {
    fn run(&self, args: &str) -> Result<String, RunError> {
        return Ok(String::from("PONG"));
    }
}

#[cfg(test)]
    use super::*;

    #[test]
    fn test_ping_command_return() {
        let ping = PingCommand;
        assert_eq!(Command::run(&ping, ""), Ok(String::from("PONG")));
    }