use std::collections::HashMap;

static RESPONSE_COMMAND_PING: &str = "PONG\n";
static REQUEST_COMMAND_PING: &str = "PING";
static ERROR_INVALID_COMMAND: &str = "ERROR INVALID COMMAND\n";

pub struct Command {
    name: &'static str,
    args: HashMap<&'static str, &'static str>,
    response: &'static str,
}

impl Command {
    pub fn new(name: &'static str, args: HashMap<&'static str, &'static str>, response: &'static str) -> Command {
        Command {
            name,
            args,
            response,
        }
    }
}
/**
 ** Build command for string passed for parameter
**/
pub struct CommandBuilder {
    commands: HashMap<&'static str, Command>,
}

impl  CommandBuilder {
    pub fn new() -> CommandBuilder {
        let mut commands: HashMap<&'static str, Command> = HashMap::new();
        commands.insert(REQUEST_COMMAND_PING, Command::new(REQUEST_COMMAND_PING, HashMap::new(), RESPONSE_COMMAND_PING));

        CommandBuilder {
            commands
        }
    }

    pub fn get_command(&mut self, command_name: &mut String) -> &'static str {
        if self.commands.contains_key(command_name.as_str()){
            println!("Command found");
            return (self.commands.get_mut(command_name.as_str()).unwrap()).response.clone();
        }
        println!("Invalid command");
        ERROR_INVALID_COMMAND
    }
}