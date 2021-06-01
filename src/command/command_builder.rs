use std::collections::HashMap;
use crate::errors::builder_error::BuilderError;
use crate::command::cmd_trait::{Command, PING_COMMAND_STR, SET_COMMAND_STR, GET_COMMAND_STR};
use crate::command::ping_cmd;
use crate::logger::Logger;
use crate::command::set_cmd::SetCommand;
use crate::command::command_parser::obtain_str_command;
use crate::command::get_cmd::GetCommand;

pub struct CommandBuilder {
    commands: HashMap<String, Box<dyn Command>>,
    id_job_exec: u32,
}

impl CommandBuilder {
    pub fn new(id_job: u32, logger: Logger<String>) -> CommandBuilder {
        let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();
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
    pub fn get_command(&self, message: &str) -> Result<&Box<dyn Command>, BuilderError> {
        let parse_msg = obtain_str_command(message);
        let retrieved;// = Err(BuilderError::not_found(message));
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
    use std::sync::Arc;
    use crate::structure_string::StructureString;

    #[test]
    fn return_ping_command() {
        let log = Logger::new(
            "prueba.txt".to_string(),
            "/home/gonzalosabatino/Escritorio".to_string(), //no sé qué otro path ponerle
        ).unwrap();

        let _arc_structure = Arc::new(StructureString::new());

        let command_builder = CommandBuilder::new(0, log);
        let result = command_builder.get_command("ping");

        assert_eq!(result.is_ok(), true);
        let _command = result.unwrap();
        //assert_eq!(command.run(vec!(""), & stt), Ok(String::from("PONG")));
    }
}
