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

pub fn insert_commands(command_builder: CommandBuilder, logger: Logger<String>) {
    LLenCommand::new(0, logger.clone(), command_builder.clone());
    LindexCommand::new(1, logger.clone(), command_builder.clone());
    LpopCommand::new(2, logger.clone(), command_builder.clone());
    LpushCommand::new(3, logger.clone(), command_builder.clone());
    LpushxCommand::new(4, logger.clone(), command_builder.clone());
    LrangeCommand::new(5, logger.clone(), command_builder.clone());
    LremCommand::new(6, logger.clone(), command_builder.clone());
    LsetCommand::new(7, logger.clone(), command_builder.clone());
    RpopCommand::new(8, logger.clone(), command_builder.clone());
    RpushCommand::new(9, logger.clone(), command_builder.clone());
    RpushxCommand::new(10, logger, command_builder);
}
