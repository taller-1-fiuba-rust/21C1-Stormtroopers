use std::collections::HashMap;

use std::fmt::Display;

use crate::commands::{Command, CommandPing, Executable, CommandEmpty, CommandSet, CommandGet};
use crate::logger::{Logger, Loggable};

use std::sync::Arc;
use crate::structure_string::StructureString;


static RESPONSE_COMMAND_PING: &str = "PONG\n";
static REQUEST_COMMAND_PING: &str = "PING";
static ERROR_INVALID_COMMAND: &str = "ERROR INVALID COMMAND\n";
static REQUEST_INVALID: &str = "";

/**
 ** Builder Command for &str passed for parameter. Return Command.
**/
pub struct CommandBuilder {
//    commands: Box<HashMap<String, Executable>>,
    commands: HashMap<&'static str, Box<dyn Executable> >,
    id_job_exec: u32,
}

impl Clone for CommandBuilder {
    fn clone(&self) -> Self {
        let commands = HashMap::new(); //self.commands.clone();
        let id = self.id_job_exec.clone();
        Self { commands, id_job_exec: id }
    }
}

impl CommandBuilder {
    pub fn new(id_job_exec: u32, logger: Logger<String>) -> CommandBuilder {
        let mut commands: HashMap<&'static str, Box<dyn Executable> > = HashMap::new();
        //let structure = Box::new(HashMap::new());
        let structure = StructureString::new();
        /*
        commands.insert(
            REQUEST_COMMAND_PING,
            Command::new(id_job_exec.clone(),REQUEST_COMMAND_PING, HashMap::new(), RESPONSE_COMMAND_PING),
        );
         */
        //let comm = Command::new(id_job_exec.clone(),REQUEST_INVALID, HashMap::new(), ERROR_INVALID_COMMAND);
        let comm = CommandEmpty::new(id_job_exec.clone());
        commands.insert(
            REQUEST_INVALID,
            Box::new(comm),
        );
        let comm2 = CommandPing::new(id_job_exec.clone(), logger.clone());
        commands.insert(
            REQUEST_COMMAND_PING,
            Box::new(comm2),
        );
        let comm3 = CommandSet::new(id_job_exec.clone(), logger.clone(), structure.clone());
        commands.insert(
            "SET",
            Box::new(comm3)
        );

        let comm3 = CommandGet::new(id_job_exec.clone(), logger.clone(), structure.clone());
        commands.insert(
            "GET",
            Box::new(comm3)
        );

        CommandBuilder { commands, id_job_exec }
    }

    pub fn get_command(&mut self, str_command: &mut String) -> Box<Executable> {
        let command_splited: Vec<&str> = str_command.split(" ").collect();
        let command_name = command_splited[0];
        //let arg1 = command_splited[1];
        //let arg2 = command_splited[2];
        //println!("command splited: {} {} {}",command_name,arg1,arg2);
        println!("command splited");
        if self.commands.contains_key(command_name) {
            /* This unwrap is safed ! */
            let exec = self
                .commands
                .get_mut(command_name)
                .expect("ERROR GETTING COMMAND");
            return exec.copy();
        }
        Box::new(CommandEmpty::new( 0))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_always_pass() {
        assert_eq!(true, true);
    }
}
