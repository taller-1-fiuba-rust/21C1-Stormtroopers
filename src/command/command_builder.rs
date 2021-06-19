use crate::command::append_cmd::AppendCommmand;
use crate::command::cmd_trait::Command;
use crate::command::command_parser::obtain_str_command;
use crate::command::constants::{
    APPEND_COMMAND_STR, COPY_COMMAND_STR, DBSIZE_COMMAND_STR, DECRBY_COMMAND_STR,
    EXISTS_COMMAND_STR, EXPIREAT_COMMAND_STR, EXPIRE_COMMAND_STR, FLUSHDB_COMMAND_STR,
    GETDEL_COMMAND_STR, GETSET_COMMAND_STR, GET_COMMAND_STR, INCRBY_COMMAND_STR,
    LINDEX_COMMAND_STR, LLEN_COMMAND_STR, LPOP_COMMAND_STR, LPUSH_COMMAND_STR, LRANGE_COMMAND_STR,
    LSET_COMMAND_STR, MGET_COMMAND_STR, MONITOR_COMMAND_STR, MSET_COMMAND_STR, PERSIST_COMMAND_STR,
    PING_COMMAND_STR, PUBSUB_COMMAND_STR, RENAME_COMMAND_STR, RPUSH_COMMAND_STR, SADD_COMMAND_STR,
    SCARD_COMMAND_STR, SET_COMMAND_STR, SISMEMBER_COMMAND_STR, SMEMBERS_COMMAND_STR,
    SORT_COMMAND_STR, SREM_COMMAND_STR, STRLEN_COMMAND_STR, TOUCH_COMMAND_STR, TTL_COMMAND_STR,
    TYPE_COMMAND_STR,
};
use crate::command::copy_cmd::CopyCommand;
use crate::command::dbsize_cmd::DbsizeCommand;
use crate::command::decrby_cmd::DecrbyCommand;
use crate::command::exists_cmd::ExistsCommand;
use crate::command::expire_cmd::ExpireCommand;
use crate::command::expireat_cmd::ExpireAtCommand;
use crate::command::flushdb_cmd::FlushdbCommand;
use crate::command::get_cmd::GetCommand;
use crate::command::getdel_cmd::GetDelCommand;
use crate::command::getset_cmd::GetSetCommand;
use crate::command::incrby_cmd::IncrbyCommand;
use crate::command::lindex_cmd::LindexCommand;
use crate::command::llen_cmd::LLenCommand;
use crate::command::lpop_cmd::LpopCommand;
use crate::command::lpush_cmd::LpushCommand;
use crate::command::lrange_cmd::LrangeCommmand;
use crate::command::lset_cmd::LSetCommand;
use crate::command::mget_cmd::MgetCommmand;
use crate::command::monitor_cmd::MonitorCommand;
use crate::command::mset_cmd::MsetCommmand;
use crate::command::persist_cmd::PersistCommand;
use crate::command::ping_cmd;
use crate::command::pubsub_cmd::PubsubCommand;
use crate::command::rename_cmd::RenameCommmand;
use crate::command::rpush_cmd::RPushCommand;
use crate::command::sadd_cmd::SAddCommand;
use crate::command::scard_cmd::ScardCommmand;
use crate::command::set_cmd::SetCommand;
use crate::command::sismember_cmd::SismemberCommmand;
use crate::command::smembers_cmd::SmembersCommand;
use crate::command::sort_cmd::SortCommand;
use crate::command::srem_cmd::SremCommmand;
use crate::command::strlen_cmd::StrlenCommand;
use crate::command::touch_cmd::TouchCommand;
use crate::command::ttl_cmd::TtlCommand;
use crate::command::type_cmd::TypeCommand;
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
            String::from(SET_COMMAND_STR),
            Box::new(SetCommand::new(id_job, logger.clone())),
        );
        commands.insert(
            String::from(GET_COMMAND_STR),
            Box::new(GetCommand::new(id_job, logger.clone())),
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
            Box::new(MonitorCommand::new(id_job, logger.clone())), //OJO, tienen 3 m, después modificar
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
            Box::new(LpopCommand::new(id_job, logger)),
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
