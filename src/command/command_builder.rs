use std::collections::HashMap;
use crate::errors::builder_error::BuilderError;
use crate::command::cmd_trait::Command;
use crate::command::ping_cmd;

pub struct CommandBuilder {
    commands: HashMap<&'static str, Box<Command>>,
}

impl CommandBuilder {
    pub fn new() -> CommandBuilder {
        let mut commands: HashMap<&'static str, Box<Command>> = HashMap::new();
        commands.insert(
            ping_cmd::PING_COMMAND_STR,
            Box::new(ping_cmd::PingCommand),
        );
        CommandBuilder { commands }
    }

    pub fn get_command(&self, cmd: &str) -> Result<&Box<Command>, BuilderError> {
        let retrieved = self.commands.get(cmd);
        if retrieved.is_some() {
            return Ok(retrieved.unwrap());
        }
        return Err(BuilderError::not_found(cmd));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_ping_command() {
        let command_builder = CommandBuilder::new();
        let result = command_builder.get_command("ping");

        assert_eq!(result.is_ok(), true);
        let command = result.unwrap();
        assert_eq!(command.run(""), Ok(String::from("PONG")));
    }
}
