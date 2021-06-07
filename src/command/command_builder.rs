use crate::command::cmd_trait::{
    Command, APPEND_COMMAND_STR, COPY_COMMAND_STR, DBSIZE_COMMAND_STR, DEL_COMMAND_STR,
    EXISTS_COMMAND_STR, FLUSHDB_COMMAND_STR, GET_COMMAND_STR, PING_COMMAND_STR, PUBSUB_COMMAND_STR,
    RENAME_COMMAND_STR, SET_COMMAND_STR, STRLEN_COMMAND_STR,
};
use crate::command::command_parser::obtain_str_command;
use crate::command::get_cmd::GetCommand;
use crate::command::ping_cmd;
use crate::command::pubsub_cmd::PubsubCommand;
use crate::command::set_cmd::SetCommand;

use crate::command::append_cmd::AppendCommmand;
use crate::command::copy_cmd::CopyCommand;
use crate::command::dbsize_cmd::DbsizeCommand;
use crate::command::del_cmd::DelCommand;
use crate::command::exists_cmd::ExistsCommand;
use crate::command::flushdb_cmd::FlushdbCommand;
use crate::command::rename_cmd::RenameCommmand;
use crate::command::strlen_cmd::StrlenCommand;
use crate::errors::builder_error::BuilderError;
use crate::logger::Logger;
use std::collections::HashMap;

pub struct CommandBuilder {
    commands: HashMap<String, Box<dyn Command>>,
    id_job_exec: u32,
}

impl CommandBuilder {
    pub fn new(id_job: u32, logger: Logger<String>) -> CommandBuilder {
        let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();

        commands.insert(
            String::from(PING_COMMAND_STR),
            Box::new(ping_cmd::PingCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(SET_COMMAND_STR),
            Box::new(SetCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(GET_COMMAND_STR),
            Box::new(GetCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(PUBSUB_COMMAND_STR),
            Box::new(PubsubCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(FLUSHDB_COMMAND_STR),
            Box::new(FlushdbCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(DBSIZE_COMMAND_STR),
            Box::new(DbsizeCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(DEL_COMMAND_STR),
            Box::new(DelCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(COPY_COMMAND_STR),
            Box::new(CopyCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(EXISTS_COMMAND_STR),
            Box::new(ExistsCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(APPEND_COMMAND_STR),
            Box::new(AppendCommmand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(RENAME_COMMAND_STR),
            Box::new(RenameCommmand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(STRLEN_COMMAND_STR),
            Box::new(StrlenCommand::new(id_job, logger)),
        );

        CommandBuilder {
            commands,
            id_job_exec: id_job,
        }
    }

    pub fn get_command(&self, message: &str) -> Result<Box<dyn Command>, BuilderError> {
        let parse_msg = obtain_str_command(message);
        let retrieved; // = Err(BuilderError::not_found(message));
        match parse_msg {
            Ok(parse_msg) => match self.commands.get(parse_msg.command.as_str()) {
                Some(comm) => retrieved = Ok(comm.clone()),
                None => retrieved = Err(BuilderError::not_found(message)),
            },
            _ => retrieved = Err(BuilderError::not_found(message)),
        }
        retrieved
    }
}

impl Clone for CommandBuilder {
    fn clone(&self) -> Self {
        let commands = HashMap::new();
        let id = self.id_job_exec;
        Self {
            commands,
            id_job_exec: id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structure_string::StructureString;
    use std::sync::Arc;

    #[test]
    fn return_ping_command() {
        let log = Logger::new("log".to_string(), "/tmp".to_string()).unwrap();

        let _arc_structure = Arc::new(StructureString::new());

        let command_builder = CommandBuilder::new(0, log);
        let result = command_builder.get_command("ping");

        assert_eq!(result.is_ok(), true);
        let _command = result.unwrap();
        //assert_eq!(command.run(vec!(""), & stt), Ok(String::from("PONG")));
    }
}
