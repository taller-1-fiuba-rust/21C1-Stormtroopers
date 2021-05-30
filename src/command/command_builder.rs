use std::collections::HashMap;
use crate::errors::builder_error::BuilderError;
use crate::command::cmd_trait::Command;
use crate::command::ping_cmd;
use crate::logger::Logger;

pub struct CommandBuilder {
    commands: HashMap<&'static str, Box<Command>>,
    id_job_exec: u32,
}

impl CommandBuilder {
    pub fn new(id_job: u32, logger: Logger<String>) -> CommandBuilder {
        let mut commands: HashMap<&'static str, Box<Command>> = HashMap::new();
        commands.insert(
            ping_cmd::PING_COMMAND_STR,
            Box::new(ping_cmd::PingCommand::new(id_job,logger)),
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
