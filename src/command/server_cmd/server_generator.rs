//! Helper function that instanciate all of the server related commands.
use crate::command::command_builder::CommandBuilder;
use crate::command::server_cmd::clear_cmd::ClearCommand;
use crate::command::server_cmd::config_cmd::ConfigCommand;
use crate::command::server_cmd::dbsize_cmd::DbSizeCommand;
use crate::command::server_cmd::exit_cmd::ExitCommand;
use crate::command::server_cmd::flushdb_cmd::FlushdbCommand;
use crate::command::server_cmd::info_cmd::InfoCommand;
use crate::command::server_cmd::monitor_cmd::MonitorCommand;
use crate::command::server_cmd::ping_cmd::PingCommand;
use crate::command::server_cmd::publish::PublishCommand;
use crate::command::server_cmd::pubsub_cmd::PubsubCommand;
use crate::command::server_cmd::suscribe::SuscribeCommand;
use crate::command::server_cmd::unsuscribe::UnsuscribeCommand;
use crate::server::logger::Logger;
use std::process;

/// Each command is instanciated and stored in the command_builder Hashmap structure.
pub fn insert_commands(command_builder: CommandBuilder, logger: Logger<String>) {
    PubsubCommand::new(process::id(), logger.clone(), command_builder.clone());
    ConfigCommand::new(process::id(), logger.clone(), command_builder.clone());
    DbSizeCommand::new(process::id(), logger.clone(), command_builder.clone());
    ExitCommand::new(process::id(), logger.clone(), command_builder.clone());
    FlushdbCommand::new(process::id(), logger.clone(), command_builder.clone());
    MonitorCommand::new(process::id(), logger.clone(), command_builder.clone());
    PingCommand::new(process::id(), logger.clone(), command_builder.clone());
    InfoCommand::new(process::id(), logger.clone(), command_builder.clone());
    PublishCommand::new(process::id(), logger.clone(), command_builder.clone());
    SuscribeCommand::new(process::id(), logger.clone(), command_builder.clone());
    UnsuscribeCommand::new(process::id(), logger.clone(), command_builder.clone());
    ClearCommand::new(process::id(), logger, command_builder);
}
