//! Helper function that instanciate all of the list related commands.
use crate::command::command_builder::CommandBuilder;
use crate::command::db_list_cmd::lindex_cmd::LindexCommand;
use crate::command::db_list_cmd::llen_cmd::LLenCommand;
use crate::command::db_list_cmd::lpop_cmd::LpopCommand;
use crate::command::db_list_cmd::lpush_cmd::LpushCommand;
use crate::command::db_list_cmd::lpushx_cmd::LpushxCommand;
use crate::command::db_list_cmd::lrange_cmd::LrangeCommand;
use crate::command::db_list_cmd::lrem_cmd::LremCommand;
use crate::command::db_list_cmd::lset_cmd::LsetCommand;
use crate::command::db_list_cmd::rpop_cmd::RpopCommand;
use crate::command::db_list_cmd::rpush_cmd::RpushCommand;
use crate::command::db_list_cmd::rpushx_cmd::RpushxCommand;
use crate::server::logger::Logger;
use std::process;

pub fn insert_commands(command_builder: CommandBuilder, logger: Logger<String>) {
    LLenCommand::new(process::id(), logger.clone(), command_builder.clone());
    LindexCommand::new(process::id(), logger.clone(), command_builder.clone());
    LpopCommand::new(process::id(), logger.clone(), command_builder.clone());
    LpushCommand::new(process::id(), logger.clone(), command_builder.clone());
    LpushxCommand::new(process::id(), logger.clone(), command_builder.clone());
    LrangeCommand::new(process::id(), logger.clone(), command_builder.clone());
    LremCommand::new(process::id(), logger.clone(), command_builder.clone());
    LsetCommand::new(process::id(), logger.clone(), command_builder.clone());
    RpopCommand::new(process::id(), logger.clone(), command_builder.clone());
    RpushCommand::new(process::id(), logger.clone(), command_builder.clone());
    RpushxCommand::new(process::id(), logger, command_builder);
}
