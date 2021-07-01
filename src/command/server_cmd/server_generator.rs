use crate::command::command_builder::CommandBuilder;
use crate::command::server_cmd::config_cmd::ConfigCommand;
use crate::command::server_cmd::dbsize_cmd::DbSizeCommand;
use crate::command::server_cmd::exit_cmd::ExitCommand;
use crate::command::server_cmd::flushdb_cmd::FlushdbCommand;
use crate::command::server_cmd::monitor_cmd::MonitorCommand;
use crate::command::server_cmd::ping_cmd::PingCommand;
use crate::command::server_cmd::pubsub_cmd::PubsubCommand;
use crate::server::logger::Logger;

pub fn insert_commands(command_builder: CommandBuilder, logger: Logger<String>) {
    PubsubCommand::new(35, logger.clone(), command_builder.clone());
    ConfigCommand::new(36, logger.clone(), command_builder.clone());
    DbSizeCommand::new(37, logger.clone(), command_builder.clone());
    ExitCommand::new(38, logger.clone(), command_builder.clone());
    FlushdbCommand::new(39, logger.clone(), command_builder.clone());
    MonitorCommand::new(40, logger.clone(), command_builder.clone());
    PingCommand::new(41, logger, command_builder);
}
