use crate::command::command_builder::CommandBuilder;
use crate::command::db_set_cmd::sadd_cmd::SaddCommand;
use crate::command::db_set_cmd::scard_cmd::ScardCommand;
use crate::command::db_set_cmd::sismember_cmd::SismemberCommand;
use crate::command::db_set_cmd::smembers_cmd::SmembersCommand;
use crate::command::db_set_cmd::srem_cmd::SremCommand;
use crate::server::logger::Logger;

pub fn insert_commands(command_builder: CommandBuilder, logger: Logger<String>) {
    SaddCommand::new(21, logger.clone(), command_builder.clone());
    ScardCommand::new(22, logger.clone(), command_builder.clone());
    SismemberCommand::new(23, logger.clone(), command_builder.clone());
    SmembersCommand::new(24, logger.clone(), command_builder.clone());
    SremCommand::new(25, logger, command_builder);
}
