use std::collections::HashMap;
use crate::errors::builder_error::BuilderError;
use crate::command::cmd_trait::{Command, PING_COMMAND_STR, SET_COMMAND_STR, GET_COMMAND_STR};
use crate::command::ping_cmd;
use crate::logger::Logger;
use crate::structure_string2::StructureString;
use crate::command::cmd_trait;
use crate::command;
use crate::command::set_cmd;
use crate::command::get_cmd;
use crate::command::get_cmd::GetCommand;
use crate::command::set_cmd::SetCommand;
use crate::command::command_parser::obtain_str_command;
use crate::errors::parse_error::ParseError;

pub struct CommandBuilder {
    commands: HashMap<String, Box<Command>>,
    id_job_exec: u32,
}

impl CommandBuilder {
    pub fn new(id_job: u32, logger: Logger<String>) -> CommandBuilder {
        let mut commands: HashMap<String, Box<Command>> = HashMap::new();
        //let mut structure = StructureString::new();
        //let mut structure = Box::new(StructureString::new());
        commands.insert(
            String::from(PING_COMMAND_STR),
            Box::new(ping_cmd::PingCommand::new(id_job.clone(),logger.clone())),
        );
        commands.insert(
            String::from(SET_COMMAND_STR),
            Box::new(SetCommand::new(id_job.clone(), logger.clone()))
        );
        commands.insert(
            String::from(GET_COMMAND_STR),
            Box::new(GetCommand::new(id_job.clone(), logger.clone()))
        );
        CommandBuilder { commands, id_job_exec: id_job.clone() }
    }
/*
let args = command_parser::obtain_str_command(message);
            let mut retrieved;
            match args {
                Ok(args) => {
                    retrieved = self.commands.get(args.command.as_str());
                }
                /*
                Err(args) => {
                    return Err(args);
                }
                 */
            }
            return match retrieved {
                Some(retrieved) => {
                    retrieved.set_args(args.arguments);
                    retrieved
                    //Ok(retrieved)
                }

                None => {
                    Err(&Box::new(BuilderError::not_found(message)))
                }

            }
 */
    pub fn get_command(&self, message: &str) -> Result<&Box<Command>, BuilderError> {
        let parse_msg = obtain_str_command(message);
        let mut retrieved = Err(BuilderError::not_found(message));
        match parse_msg {
            Ok(parse_msg) => {
                match self.commands.get(parse_msg.command.as_str()) {
                    Some(comm) => retrieved = Ok(comm),
                    None       => retrieved = Err(BuilderError::not_found(message)),
                }
            }
            _ => retrieved = Err(BuilderError::not_found(message))
        }
        retrieved
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
    use crate::structure_string2;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    #[test]
    fn return_ping_command() {
        let log = Logger::new(
            "prueba.txt".to_string(),
            "/home/gonzalosabatino/Escritorio".to_string(), //no sé qué otro path ponerle
        ).unwrap();

        let mut stt = Arc::new(Mutex::new(HashMap::new()));
        let mut structure = Box::new(structure_string2::StructureString::new(&mut stt));

        let command_builder = CommandBuilder::new(0, log);
        let result = command_builder.get_command("ping");

        assert_eq!(result.is_ok(), true);
        let command = result.unwrap();
        //assert_eq!(command.run(vec!(""), & stt), Ok(String::from("PONG")));
    }
}
