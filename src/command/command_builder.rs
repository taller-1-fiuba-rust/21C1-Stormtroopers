use crate::command::cmd_trait::Command;
use crate::command::command_parser::obtain_str_command;
use crate::command::constants::*;
use crate::command::db_list_cmd::lindex_cmd::LindexCommand;
use crate::command::db_list_cmd::llen_cmd::LLenCommand;
use crate::command::db_list_cmd::lpop_cmd::LpopCommand;
use crate::command::db_list_cmd::lpush_cmd::LpushCommand;
use crate::command::db_list_cmd::lpushx_cmd::LpushxCommand;
use crate::command::db_list_cmd::lrange_cmd::LrangeCommmand;
use crate::command::db_list_cmd::lrem_cmd::LremCommand;
use crate::command::db_list_cmd::lset_cmd::LSetCommand;
use crate::command::db_list_cmd::rpop_cmd::RpopCommand;
use crate::command::db_list_cmd::rpush_cmd::RPushCommand;
use crate::command::db_list_cmd::rpushx_cmd::RPushxCommmand;
use crate::command::db_set_cmd::sadd_cmd::SAddCommand;
use crate::command::db_set_cmd::scard_cmd::ScardCommmand;
use crate::command::db_set_cmd::sismember_cmd::SismemberCommmand;
use crate::command::db_set_cmd::smembers_cmd::SmembersCommand;
use crate::command::db_set_cmd::srem_cmd::SremCommmand;
use crate::command::db_string_cmd::append_cmd::AppendCommmand;
use crate::command::db_string_cmd::decrby_cmd::DecrbyCommand;
use crate::command::db_string_cmd::get_cmd::GetCommand;
use crate::command::db_string_cmd::getdel_cmd::GetDelCommand;
use crate::command::db_string_cmd::getset_cmd::GetSetCommand;
use crate::command::db_string_cmd::incrby_cmd::IncrbyCommand;
use crate::command::db_string_cmd::mget_cmd::MgetCommmand;
use crate::command::db_string_cmd::mset_cmd::MsetCommmand;
use crate::command::db_string_cmd::set_cmd::SetCommand;
use crate::command::db_string_cmd::strlen_cmd::StrlenCommand;
use crate::command::keys_cmd::copy_cmd::CopyCommand;
use crate::command::keys_cmd::del_cmd::DelCommmand;
use crate::command::keys_cmd::exists_cmd::ExistsCommand;
use crate::command::keys_cmd::expire_cmd::ExpireCommand;
use crate::command::keys_cmd::expireat_cmd::ExpireAtCommand;
use crate::command::keys_cmd::persist_cmd::PersistCommand;
use crate::command::keys_cmd::rename_cmd::RenameCommmand;
use crate::command::keys_cmd::sort_cmd::SortCommand;
use crate::command::keys_cmd::touch_cmd::TouchCommand;
use crate::command::keys_cmd::ttl_cmd::TtlCommand;
use crate::command::keys_cmd::type_cmd::TypeCommand;
use crate::command::ping_cmd;
use crate::command::pubsub_cmd::pubsub_cmd::PubsubCommand;
use crate::command::server_cmd::config_cmd::ConfigCommand;
use crate::command::server_cmd::dbsize_cmd::DbsizeCommand;
use crate::command::server_cmd::exit_cmd::ExitCommand;
use crate::command::server_cmd::flushdb_cmd::FlushdbCommand;
use crate::command::server_cmd::monitor_cmd::MonitorCommand;
use crate::errors::builder_error::BuilderError;
use crate::server::logger::Logger;
use std::collections::HashMap;

pub struct CommandBuilder {
    commands: HashMap<String, Box<dyn Command>>,
    id_job_exec: u32,
}

impl CommandBuilder {
    pub fn new(id_job: u32, logger: Logger<String>) -> CommandBuilder {
        let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();

        commands.insert(
            String::from(PING_COMMAND_STR),
            Box::new(ping_cmd::PingCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(EXIT_COMMAND_STR),
            Box::new(ExitCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(SET_COMMAND_STR),
            Box::new(SetCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(GET_COMMAND_STR),
            Box::new(GetCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(CONFIG_COMMAND_STR),
            Box::new(ConfigCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(PUBSUB_COMMAND_STR),
            Box::new(PubsubCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(FLUSHDB_COMMAND_STR),
            Box::new(FlushdbCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(DBSIZE_COMMAND_STR),
            Box::new(DbsizeCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(DEL_COMMAND_STR),
            Box::new(DelCommmand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(GETDEL_COMMAND_STR),
            Box::new(GetDelCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(TYPE_COMMAND_STR),
            Box::new(TypeCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(COPY_COMMAND_STR),
            Box::new(CopyCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(EXISTS_COMMAND_STR),
            Box::new(ExistsCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(TOUCH_COMMAND_STR),
            Box::new(TouchCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(APPEND_COMMAND_STR),
            Box::new(AppendCommmand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(RENAME_COMMAND_STR),
            Box::new(RenameCommmand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(SORT_COMMAND_STR),
            Box::new(SortCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(STRLEN_COMMAND_STR),
            Box::new(StrlenCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(EXPIRE_COMMAND_STR),
            Box::new(ExpireCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(EXPIREAT_COMMAND_STR),
            Box::new(ExpireAtCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(TTL_COMMAND_STR),
            Box::new(TtlCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(PERSIST_COMMAND_STR),
            Box::new(PersistCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(MGET_COMMAND_STR),
            Box::new(MgetCommmand::new(id_job, logger.clone())), //OJO, tienen 3 m, después modificar
        );
        commands.insert(
            String::from(MONITOR_COMMAND_STR),
            Box::new(MonitorCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(MSET_COMMAND_STR),
            Box::new(MsetCommmand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(INCRBY_COMMAND_STR),
            Box::new(IncrbyCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(GETSET_COMMAND_STR),
            Box::new(GetSetCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(LINDEX_COMMAND_STR),
            Box::new(LindexCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(LSET_COMMAND_STR),
            Box::new(LSetCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(DECRBY_COMMAND_STR),
            Box::new(DecrbyCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(RPUSH_COMMAND_STR),
            Box::new(RPushCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(LLEN_COMMAND_STR),
            Box::new(LLenCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(SADD_COMMAND_STR),
            Box::new(SAddCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(SMEMBERS_COMMAND_STR),
            Box::new(SmembersCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(SCARD_COMMAND_STR),
            Box::new(ScardCommmand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(SISMEMBER_COMMAND_STR),
            Box::new(SismemberCommmand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(SREM_COMMAND_STR),
            Box::new(SremCommmand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(LPUSH_COMMAND_STR),
            Box::new(LpushCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(LRANGE_COMMAND_STR),
            Box::new(LrangeCommmand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(LPOP_COMMAND_STR),
            Box::new(LpopCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(LPUSHX_COMMAND_STR),
            Box::new(LpushxCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(LREM_COMMAND_STR),
            Box::new(LremCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(RPOP_COMMAND_STR),
            Box::new(RpopCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(RPUSHX_COMMAND_STR),
            Box::new(RPushxCommmand::new(id_job, logger)),
        );
        CommandBuilder {
            commands,
            id_job_exec: id_job,
        }
    }

    pub fn get_command(&self, message: &str) -> Result<Box<dyn Command>, BuilderError> {
        let parse_msg = obtain_str_command(message);
        let retrieved; // = Err(BuilderError::not_found(message));
        match parse_msg {
            Ok(parse_msg) => match self.commands.get(parse_msg.command.as_str()) {
                Some(comm) => retrieved = Ok(comm.clone()),
                None => retrieved = Err(BuilderError::not_found(message)),
            },
            _ => retrieved = Err(BuilderError::not_found(message)),
        }
        retrieved
    }
}

impl Clone for CommandBuilder {
    fn clone(&self) -> Self {
        let commands = HashMap::new();
        let id = self.id_job_exec;
        Self {
            commands,
            id_job_exec: id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_base::db_string::DataBaseString;
    use std::sync::Arc;

    #[test]
    fn return_ping_command() {
        let log = Logger::new("log".to_string(), "/tmp".to_string()).unwrap();

        let _arc_structure = Arc::new(DataBaseString::new());

        let command_builder = CommandBuilder::new(0, log);
        let result = command_builder.get_command("ping");

        assert_eq!(result.is_ok(), true);
        let _command = result.unwrap();
        //assert_eq!(command.run(vec!(""), & stt), Ok(String::from("PONG")));
    }
}
