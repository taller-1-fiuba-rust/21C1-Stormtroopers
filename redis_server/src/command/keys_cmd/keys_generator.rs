//! Helper function that instanciate all of the key related commands.
use crate::command::command_builder::CommandBuilder;
use crate::command::keys_cmd::copy_cmd::CopyCommand;
use crate::command::keys_cmd::del_cmd::DelCommand;
use crate::command::keys_cmd::exists_cmd::ExistsCommand;
use crate::command::keys_cmd::expire_cmd::ExpireCommand;
use crate::command::keys_cmd::expireat_cmd::ExpireAtCommand;
use crate::command::keys_cmd::keys_pattern_cmd::KeysCommand;
use crate::command::keys_cmd::persist_cmd::PersistCommand;
use crate::command::keys_cmd::rename_cmd::RenameCommand;
use crate::command::keys_cmd::sort_cmd::SortCommand;
use crate::command::keys_cmd::touch_cmd::TouchCommand;
use crate::command::keys_cmd::ttl_cmd::TtlCommand;
use crate::command::keys_cmd::type_cmd::TypeCommand;
use crate::server::logger::Logger;
use std::process;

/// Each command is instanciated and stored in the command_builder Hashmap structure.
pub fn insert_commands(command_builder: CommandBuilder, logger: Logger<String>) {
    CopyCommand::new(process::id(), logger.clone(), command_builder.clone());
    DelCommand::new(process::id(), logger.clone(), command_builder.clone());
    ExistsCommand::new(process::id(), logger.clone(), command_builder.clone());
    PersistCommand::new(process::id(), logger.clone(), command_builder.clone());
    RenameCommand::new(process::id(), logger.clone(), command_builder.clone());
    SortCommand::new(process::id(), logger.clone(), command_builder.clone());
    TouchCommand::new(process::id(), logger.clone(), command_builder.clone());
    TtlCommand::new(process::id(), logger.clone(), command_builder.clone());
    TypeCommand::new(process::id(), logger.clone(), command_builder.clone());
    ExpireCommand::new(process::id(), logger.clone(), command_builder.clone());
    ExpireAtCommand::new(process::id(), logger.clone(), command_builder.clone());
    KeysCommand::new(process::id(), logger.clone(), command_builder.clone());
    ExpireAtCommand::new(process::id(), logger, command_builder);
}
