use crate::command::cmd_trait::Command;
use crate::command::command_parser::obtain_str_command;
use std::sync::{Arc, Mutex};

use crate::errors::builder_error::BuilderError;
use std::collections::HashMap;

pub struct CommandBuilder {
    commands: Arc<Mutex<HashMap<String, Box<dyn Command>>>>,
    id_job_exec: u32,
}

impl CommandBuilder {
    pub fn new(id_job: u32) -> CommandBuilder {
        let commands = Arc::new(Mutex::new(HashMap::new()));
        Self {
            commands,
            id_job_exec: id_job,
        }
    }

    pub fn insert(&mut self, key: String, cmd: Box<dyn Command>) {
        let mut commands = self.commands.lock().unwrap();
        commands.insert(key, cmd);
    }

    pub fn get_command(&self, message: &str) -> Result<Box<dyn Command>, BuilderError> {
        let parse_msg = obtain_str_command(message);
        let retrieved;
        let commands = self.commands.lock().unwrap();
        match parse_msg {
            Ok(parse_msg) => match commands.get(parse_msg.command.as_str()) {
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
        let commands = self.commands.clone();
        let id = self.id_job_exec;
        Self {
            commands,
            id_job_exec: id,
        }
    }
}
