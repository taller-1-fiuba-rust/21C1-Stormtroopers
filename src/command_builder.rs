use std::collections::HashMap;

//use super::{LOG_NAME, LOG_PATH};
use crate::logger::{Loggable, Logger};
use std::time::SystemTime;

static RESPONSE_COMMAND_PING: &str = "PONG\n";
static REQUEST_COMMAND_PING: &str = "PING";
static ERROR_INVALID_COMMAND: &str = "ERROR INVALID COMMAND\n";
static REQUEST_INVALID: &str = "";

impl Loggable for Command {
    fn get_id_client(&self) -> i32 {
        2
    }
    fn get_id_thread(&self) -> i32 {
        -1
    }

    fn get_timestamp(&self) -> SystemTime {
        SystemTime::now()
    }
}

pub struct Command {
    name: &'static str,
    args: HashMap<&'static str, &'static str>,
    response: &'static str,
}

impl Clone for Command {
    fn clone(&self) -> Self {
        let name = self.name.clone();
        let args = self.args.clone();
        let response = self.response.clone();
        Self {
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
        name: &'static str,
        args: HashMap<&'static str, &'static str>,
        response: &'static str,
    ) -> Command {
        Command {
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
}

impl Clone for CommandBuilder {
    fn clone(&self) -> Self {
        let commands = self.commands.clone();
        Self { commands }
    }
}

impl CommandBuilder {
    pub fn new() -> CommandBuilder {
        let mut commands: HashMap<&'static str, Command> = HashMap::new();
        commands.insert(
            REQUEST_COMMAND_PING,
            Command::new(REQUEST_COMMAND_PING, HashMap::new(), RESPONSE_COMMAND_PING),
        );
        commands.insert(
            REQUEST_INVALID,
            Command::new(REQUEST_INVALID, HashMap::new(), ERROR_INVALID_COMMAND),
        );
        CommandBuilder { commands }
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
