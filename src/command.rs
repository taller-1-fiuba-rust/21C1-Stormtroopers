use std::collections::HashMap;

static RESPONSE_COMMAND_PING: &str = "PONG\n";
static REQUEST_COMMAND_PING: &str = "PING";
static ERROR_INVALID_COMMAND: &str = "ERROR INVALID COMMAND\n";
static REQUEST_INVALID: &str = "";

pub struct Command {
    name: &'static str,
    args: HashMap<&'static str, &'static str>,
    response: &'static str,
}
/**
 ** Representation of Command with name, args and response.
**/
impl Command {
    pub fn new(name: &'static str, args: HashMap<&'static str, &'static str>, response: &'static str) -> Command {
        Command {
            name,
            args,
            response,
        }
    }

    pub fn str_response(&self) -> &'static str {
        self.response.clone()
    }

}
/**
 ** Builder Command for &str passed for parameter. Return Command.
**/
pub struct CommandBuilder {
    commands: HashMap<&'static str, Command>,
}

impl  CommandBuilder {
    pub fn new() -> CommandBuilder {
        let mut commands: HashMap<&'static str, Command> = HashMap::new();
        commands.insert(REQUEST_COMMAND_PING, Command::new(REQUEST_COMMAND_PING, HashMap::new(), RESPONSE_COMMAND_PING));
        commands.insert(REQUEST_INVALID, Command::new(REQUEST_INVALID, HashMap::new(), ERROR_INVALID_COMMAND));
        CommandBuilder {
            commands
        }
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
        if self.commands.contains_key(command_name.as_str()){
            /* This unwrap is safed ! */
            return self.commands.get_mut(command_name.as_str()).unwrap();
        }
        return self.commands.get_mut("").unwrap();
    }
}