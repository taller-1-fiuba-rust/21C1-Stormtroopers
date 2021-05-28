use std::collections::HashMap;

//use super::{LOG_NAME, LOG_PATH};
use crate::logger::{Loggable, Logger};
use std::time::SystemTime;

static RESPONSE_COMMAND_PING: &str = "PONG\n";
static REQUEST_COMMAND_PING: &str = "PING";
static ERROR_INVALID_COMMAND: &str = "ERROR INVALID COMMAND\n";
static REQUEST_INVALID: &str = "";

impl Loggable for Command {
    fn get_id_client(&self) -> &str {
        "CommandBuilder"
    }
    fn get_id_thread(&self) -> u32 {
        self.id_job_exec.clone()
    }

    fn get_timestamp(&self) -> SystemTime {
        SystemTime::now()
    }
}

pub struct Command {
    id_job_exec: u32,
    name: &'static str,
    args: HashMap<&'static str, &'static str>,
    response: &'static str,
}

impl Clone for Command {
    fn clone(&self) -> Self {
        let id_job_exec = self.id_job_exec.clone();
        let name = self.name.clone();
        let args = self.args.clone();
        let response = self.response.clone();
        Self {
            id_job_exec,
            name,
            args,
            response,
        }
    }
}

/**
 ** Representation of Command with name, args and response.
**/
impl Command {
    pub fn new(
        id_job_exec: u32,
        name: &'static str,
        args: HashMap<&'static str, &'static str>,
        response: &'static str,
    ) -> Command {
        Command {
            id_job_exec,
            name,
            args,
            response,
        }
    }

    pub fn str_response(&self, logger: Logger<String>) -> &'static str {
        let response = self.response.clone();
        logger
            .info(self, &format!("Response command: {}", response))
            .expect("ERROR RESPONSE COMMAND");

        response
    }
}
/**
 ** Builder Command for &str passed for parameter. Return Command.
**/
pub struct CommandBuilder {
    commands: HashMap<&'static str, Command>,
    id_job_exec: u32,
}

impl Clone for CommandBuilder {
    fn clone(&self) -> Self {
        let commands = self.commands.clone();
        let id = self.id_job_exec;
        Self { commands, id_job_exec: id }
    }
}

impl CommandBuilder {
    pub fn new(id_job_exec: u32) -> CommandBuilder {
        let mut commands: HashMap<&'static str, Command> = HashMap::new();
        commands.insert(
            REQUEST_COMMAND_PING,
            Command::new(id_job_exec.clone(),REQUEST_COMMAND_PING, HashMap::new(), RESPONSE_COMMAND_PING),
        );
        commands.insert(
            REQUEST_INVALID,
            Command::new(id_job_exec.clone(),REQUEST_INVALID, HashMap::new(), ERROR_INVALID_COMMAND),
        );
        CommandBuilder { commands, id_job_exec }
    }

    /*
       pub fn get_command_response(&mut self, command_name: &mut String) -> &'static str {
           if self.commands.contains_key(command_name.as_str()){
               /* This unwrap is safed ! */
               return (self.commands.get_mut(command_name.as_str()).unwrap()).response.clone();
           }
           return (self.commands.get_mut("").unwrap()).response.clone();
       }
    */

    pub fn get_command(&mut self, command_name: &mut String) -> &mut Command {
        if self.commands.contains_key(command_name.as_str()) {
            /* This unwrap is safed ! */
            return self
                .commands
                .get_mut(command_name.as_str())
                .expect("ERROR GETTING COMMAND");
        }
        return self.commands.get_mut("").expect("ERROR GETTING COMMAND");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_always_pass() {
        assert_eq!(true, true);
    }
}
