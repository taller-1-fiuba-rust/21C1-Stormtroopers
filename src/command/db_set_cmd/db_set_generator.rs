//! Helper function that instanciate all of the set related commands.
use crate::command::command_builder::CommandBuilder;
use crate::command::db_set_cmd::sadd_cmd::SaddCommand;
use crate::command::db_set_cmd::scard_cmd::ScardCommand;
use crate::command::db_set_cmd::sismember_cmd::SismemberCommand;
use crate::command::db_set_cmd::smembers_cmd::SmembersCommand;
use crate::command::db_set_cmd::srem_cmd::SremCommand;
use crate::server::logger::Logger;
use std::process;

/// Each command is instanciated and stored in the command_builder Hashmap structure.
pub fn insert_commands(command_builder: CommandBuilder, logger: Logger<String>) {
    SaddCommand::new(process::id(), logger.clone(), command_builder.clone());
    ScardCommand::new(process::id(), logger.clone(), command_builder.clone());
    SismemberCommand::new(process::id(), logger.clone(), command_builder.clone());
    SmembersCommand::new(process::id(), logger.clone(), command_builder.clone());
    SremCommand::new(process::id(), logger, command_builder);
}
