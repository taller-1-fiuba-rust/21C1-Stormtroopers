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

pub fn insert_commands(command_builder: CommandBuilder, logger: Logger<String>) {
    AppendCommand::new(11, logger.clone(), command_builder.clone());
    DecrbyCommand::new(12, logger.clone(), command_builder.clone());
    GetCommand::new(13, logger.clone(), command_builder.clone());
    GetDelCommand::new(14, logger.clone(), command_builder.clone());
    GetSetCommand::new(15, logger.clone(), command_builder.clone());
    IncrbyCommand::new(16, logger.clone(), command_builder.clone());
    MgetCommand::new(17, logger.clone(), command_builder.clone());
    MsetCommand::new(18, logger.clone(), command_builder.clone());
    SetCommand::new(19, logger.clone(), command_builder.clone());
    StrlenCommand::new(20, logger, command_builder);
}
