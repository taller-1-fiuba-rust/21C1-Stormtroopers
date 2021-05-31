use std::collections::HashMap;
use crate::errors::builder_error::BuilderError;
use crate::command::cmd_trait::{Command, PING_COMMAND_STR, SET_COMMAND_STR, GET_COMMAND_STR};
use crate::command::ping_cmd;
use crate::logger::Logger;
use crate::structure_string::StructureString;
use crate::command::cmd_trait;
use crate::command;
use crate::command::set_cmd;
use crate::command::get_cmd;
use crate::command::get_cmd::GetCommand;
use crate::command::set_cmd::SetCommand;

pub struct CommandBuilder {
    commands: HashMap<&'static str, Box<Command>>,
    id_job_exec: u32,
}

impl CommandBuilder {
    pub fn new(id_job: u32, logger: Logger<String>) -> CommandBuilder {
        let mut commands: HashMap<&'static str, Box<Command>> = HashMap::new();
        let mut structure = StructureString::new();
        commands.insert(
            PING_COMMAND_STR,
            Box::new(ping_cmd::PingCommand::new(id_job.clone(),logger.clone(),structure.clone())),
        );
        commands.insert(
            SET_COMMAND_STR,
            Box::new(SetCommand::new(id_job.clone(), logger.clone(), structure.clone()))
        );
        commands.insert(
            GET_COMMAND_STR,
            Box::new(GetCommand::new(id_job.clone(), logger.clone(), structure.clone()))
        );
        CommandBuilder { commands, id_job_exec: id_job.clone() }
    }

    pub fn get_command(&self, cmd: &str) -> Result<&Box<Command>, BuilderError> {
        let retrieved = self.commands.get(cmd);
        if retrieved.is_some() {
            return Ok(retrieved.unwrap());
        }
        return Err(BuilderError::not_found(cmd));
    }
}

impl Clone for CommandBuilder {
    fn clone(&self) -> Self {
        let commands = HashMap::new();
        let id = self.id_job_exec.clone();
        Self { commands, id_job_exec: id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_ping_command() {
        let log = Logger::new(
            "prueba.txt".to_string(),
            "/home/gonzalosabatino/Escritorio".to_string(), //no sé qué otro path ponerle
        )
            .unwrap();
        let command_builder = CommandBuilder::new(0, log);
        let result = command_builder.get_command("ping");

        assert_eq!(result.is_ok(), true);
        let command = result.unwrap();
        assert_eq!(command.run(vec!("")), Ok(String::from("PONG")));
    }
}
