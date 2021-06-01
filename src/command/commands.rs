/*
use std::collections::HashMap;

use crate::logger::{Logger, Loggable};
use crate::structure_string::StructureString;

pub trait Executable: Send {
    fn run(&mut self) -> String;
    fn run_with_args(&mut self, args: &mut Vec<&str>) -> String;
    fn copy(&self) -> Box<dyn Executable>;
}

impl Loggable for Command {
    fn get_id_client(&self) -> &str {
        "Command"
    }
    fn get_id_thread(&self) -> u32 {
        self.id_job_exec.clone()
    }
}

#[derive(Debug, Copy)]
pub struct Command {
    id_job_exec: u32,
    name: &'static str,
    //args: HashMap<String, String>,
    response: &'static str,
}

impl Clone for Command {
    fn clone(&self) -> Self {
        let id_job_exec = self.id_job_exec.clone();
        let name = self.name.clone();
       // let args = self.args.clone();
        let response = self.response.clone();
        Self {
            id_job_exec,
            name,
          //  args,
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
        args: HashMap<String, String>,
        response: &'static str,
    ) -> Command {
        Command {
            id_job_exec,
            name,
            //args,
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

impl Executable for Command {
    fn run(&mut self) -> String {
        String::from("Soy un executable for Command\n")
    }

    fn run_with_args(&mut self, args: &mut Vec<&str>) -> String {
        String::from("Soy un executable for Command\n")
    }

    fn copy(&self) -> Box<dyn Executable> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CommandPing {

}

impl CommandPing {
    pub fn new(id_job: u32, logger: Logger<String>) -> CommandPing {
        let comm = CommandPing {

        };
        comm
    }
}

impl Executable for CommandPing {
    fn run(&mut self) -> String {
        String::from("PONG\n")
    }

    fn run_with_args(&mut self, args: &mut Vec<&str>) -> String {
        String::from("PONG\n")
    }

    fn copy(&self) -> Box<dyn Executable> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CommandEmpty {}
impl CommandEmpty {
    pub fn new(id_job: u32) -> CommandEmpty {
        CommandEmpty {}
    }
}
impl Executable for CommandEmpty {
    fn run(&mut self) -> String {
        String::from("ERROR COMMAND\n")
    }

    fn run_with_args(&mut self, args: &mut Vec<&str>) -> String {
        String::from("ERROR COMMAND: EMPTY\n")
    }

    fn copy(&self) -> Box<dyn Executable> {
        Box::new(self.clone())
    }
}
/*
impl Copy for EmptyCommand { }
impl Clone for EmptyCommand {
    fn clone(&self) -> EmptyCommand {
        self.clone()
    }
}
 */
impl Loggable for CommandSet {
    fn get_id_client(&self) -> &str {
        "CommandSet"
    }
    fn get_id_thread(&self) -> u32 {
        self.id_job.clone()
    }
}

#[derive(Clone)]
pub struct CommandSet {
    id_job: u32,
    name: &'static str,
    pub key: &'static str,
    pub value: &'static str,
    logger: Logger<String>,
    //structure: &'static mut HashMap<&'static str, &'static str>
    structure: StructureString<String>
}
impl CommandSet {
    pub fn new(id_job: u32, logger: Logger<String>, structure: StructureString<String>) -> CommandSet {
        CommandSet {
            id_job,
            name: "SET",
            key: "",
            value: "",
            logger,
            structure: structure,
        }
    }
}
impl Executable for CommandSet {
    fn run(&mut self) -> String {
        self.logger.info(self, "Executing command SET ...");
       // self.structure.insert(self.key, self.value);
        String::from("OK\n")
    }

    fn run_with_args(&mut self, args: &mut Vec<&str>) -> String {
        self.logger.info(self, "Executing command SET ...");

        //self.structure.insert(args[0].clone(), args[1].clone());
        self.structure.set_string(String::from(args[0]),String::from(args[1]));
        String::from("OK\n")
    }

    fn copy(&self) -> Box<dyn Executable> {
        Box::new(self.clone())
    }
}
/********************************/
impl Loggable for CommandGet {
    fn get_id_client(&self) -> &str {
        "CommandGet"
    }
    fn get_id_thread(&self) -> u32 {
        self.id_job.clone()
    }
}

#[derive(Clone)]
pub struct CommandGet {
    id_job: u32,
    name: &'static str,
    pub key: &'static str,
    pub value: &'static str,
    logger: Logger<String>,
    structure: StructureString<String>,
}
impl CommandGet {
    pub fn new(id_job: u32, logger: Logger<String>, structure: StructureString<String>) -> CommandGet {
        CommandGet {
            id_job: id_job,
            name: "GET",
            key: "",
            value: "",
            logger,
            structure,
        }
    }
}
impl Executable for CommandGet {
    fn run(&mut self) -> String { String::from("")}

    fn run_with_args(&mut self, args: &mut Vec<&str>) -> String {
        self.logger.info(self, "Executing command GET ...");

        /*
        if let Some(value) = self.structure.get_mut(&args[0]) {
            let v = value.clone();
            return String::from(v)
        }
         */
        let value = self.structure.get_string(String::from(args[0]));
        return value.clone();

        //return String::from("ERR GET\n")
    }

    fn copy(&self) -> Box<dyn Executable> {
        Box::new(self.clone())
    }
}
 */