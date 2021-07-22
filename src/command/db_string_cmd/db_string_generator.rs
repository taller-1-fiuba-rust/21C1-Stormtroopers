//! Helper function that instanciate all of the string related commands.
use crate::command::command_builder::CommandBuilder;
use crate::command::db_string_cmd::append_cmd::AppendCommand;
use crate::command::db_string_cmd::decrby_cmd::DecrbyCommand;
use crate::command::db_string_cmd::get_cmd::GetCommand;
use crate::command::db_string_cmd::getdel_cmd::GetDelCommand;
use crate::command::db_string_cmd::getset_cmd::GetSetCommand;
use crate::command::db_string_cmd::incrby_cmd::IncrbyCommand;
use crate::command::db_string_cmd::mget_cmd::MgetCommand;
use crate::command::db_string_cmd::mset_cmd::MsetCommand;
use crate::command::db_string_cmd::set_cmd::SetCommand;
use crate::command::db_string_cmd::strlen_cmd::StrlenCommand;
use crate::server::logger::Logger;
use std::process;

pub fn insert_commands(command_builder: CommandBuilder, logger: Logger<String>) {
    AppendCommand::new(process::id(), logger.clone(), command_builder.clone());
    DecrbyCommand::new(process::id(), logger.clone(), command_builder.clone());
    GetCommand::new(process::id(), logger.clone(), command_builder.clone());
    GetDelCommand::new(process::id(), logger.clone(), command_builder.clone());
    GetSetCommand::new(process::id(), logger.clone(), command_builder.clone());
    IncrbyCommand::new(process::id(), logger.clone(), command_builder.clone());
    MgetCommand::new(process::id(), logger.clone(), command_builder.clone());
    MsetCommand::new(process::id(), logger.clone(), command_builder.clone());
    SetCommand::new(process::id(), logger.clone(), command_builder.clone());
    StrlenCommand::new(process::id(), logger, command_builder);
}
