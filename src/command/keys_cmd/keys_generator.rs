use crate::command::command_builder::CommandBuilder;
use crate::command::keys_cmd::copy_cmd::CopyCommand;
use crate::command::keys_cmd::del_cmd::DelCommand;
use crate::command::keys_cmd::exists_cmd::ExistsCommand;
use crate::command::keys_cmd::expire_cmd::ExpireCommand;
use crate::command::keys_cmd::expireat_cmd::ExpireAtCommand;
use crate::command::keys_cmd::persist_cmd::PersistCommand;
use crate::command::keys_cmd::rename_cmd::RenameCommand;
use crate::command::keys_cmd::sort_cmd::SortCommand;
use crate::command::keys_cmd::touch_cmd::TouchCommand;
use crate::command::keys_cmd::ttl_cmd::TtlCommand;
use crate::command::keys_cmd::type_cmd::TypeCommand;
use crate::server::logger::Logger;

pub fn insert_commands(command_builder: CommandBuilder, logger: Logger<String>) {
    CopyCommand::new(26, logger.clone(), command_builder.clone());
    DelCommand::new(27, logger.clone(), command_builder.clone());
    ExistsCommand::new(28, logger.clone(), command_builder.clone());
    PersistCommand::new(29, logger.clone(), command_builder.clone());
    RenameCommand::new(30, logger.clone(), command_builder.clone());
    SortCommand::new(31, logger.clone(), command_builder.clone());
    TouchCommand::new(32, logger.clone(), command_builder.clone());
    TtlCommand::new(33, logger.clone(), command_builder.clone());
    TypeCommand::new(34, logger.clone(), command_builder.clone());
    ExpireCommand::new(35, logger.clone(), command_builder.clone());
    ExpireAtCommand::new(36, logger.clone(), command_builder.clone());
    ExpireAtCommand::new(36, logger, command_builder);
}
