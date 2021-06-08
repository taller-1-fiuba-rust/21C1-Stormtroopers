use crate::errors::run_error::RunError;
use crate::structure_list::StructureList;
use crate::structure_set::StructureSet;
use crate::structure_string::StructureString;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use std::any::Any;

#[derive(Clone)] //ver si con esto es suficiente
pub enum Structure {
    StructureString(StructureString<String>),
    StructureList(StructureList<String>),
    StructureSet(StructureSet<String>),
}

pub struct StructureGeneral {
    structure: Arc<Mutex<HashMap<String, Structure>>>,
}

impl Default for StructureGeneral {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for StructureGeneral {
    fn clone(&self) -> Self {
        Self {
            structure: self.structure.clone(),
        }
    }
}

pub fn get_string(structure: Structure) -> Result<StructureString<String>, RunError> {
    match structure {
        Structure::StructureString(a) => Ok(a),
        //#[allow(unreachable_patterns)]
        _ => Err(RunError {
            message: "Error: Not StructureString".to_string(),
            cause: " ".to_string(),
        }),
    }
}

#[allow(dead_code)]
pub fn get_list(structure: Structure) -> Result<StructureList<String>, RunError> {
    match structure {
        Structure::StructureList(a) => Ok(a),
        //#[allow(unreachable_patterns)]
        _ => Err(RunError {
            message: "Error: Not StructureString".to_string(),
            cause: " ".to_string(),
        }),
    }
}

#[allow(dead_code)]
pub fn get_set(structure: Structure) -> Result<StructureSet<String>, RunError> {
    match structure {
        Structure::StructureSet(a) => Ok(a),
        //#[allow(unreachable_patterns)]
        _ => Err(RunError {
            message: "Error: Not StructureString".to_string(),
            cause: " ".to_string(),
        }),
    }
}

impl StructureGeneral {
    pub fn new() -> Self {
        let structure = Arc::new(Mutex::new(HashMap::new()));
        Self { structure }
    }

    pub fn get(&self, name_type: String) -> Structure {
        let structure = self.structure.lock().unwrap();
        structure.get(&name_type).unwrap().clone()
    }

    pub fn add_structure(&self, key: String, value: Structure) {
        let mut structure_general = self.structure.lock().unwrap();

        structure_general.insert(key, value); //no usar unwrap acá porque devuelve None
                                              //la primera vez que se inserta algo en una key, entonces pincha todo
    }

    pub fn dbsize(&self) -> usize {
        let mut cont = 0;
        let structure = self.structure.lock().unwrap();
        for (_key, val) in structure.iter() {
            match val {
                Structure::StructureString(a) => {
                    cont += a.dbsize();
                }
                #[allow(unreachable_patterns)]
                _ => {
                    cont += 0;
                }
            }
        }

        cont
    }

    pub fn clean_all_data(&self) -> bool {
        let mut response = true;
        let structure = self.structure.lock().unwrap();
        for (_key, val) in structure.iter() {
            match val {
                Structure::StructureString(a) => {
                    response &= a.clean_all_data();
                }
                #[allow(unreachable_patterns)]
                _ => {
                    response = false;
                }
            }
        }
        response
    }

    pub fn get_db(&self, name_type: String) -> Result<Box<dyn Storeable>, RunError> {
        let db_gral = self
            .structure
            .lock()
            .unwrap()
            .get(&name_type)
            .unwrap()
            .clone();
        match db_gral {
            Structure::StructureString(s) => Ok(Box::new(s)),
            Structure::StructureList(s) => Ok(Box::new(s)),
            Structure::StructureSet(s) => Ok(Box::new(s)),
        }
    }

    pub fn get_string_db(&self) -> StructureString<String> {
        let db_gral = self
            .structure
            .lock()
            .unwrap()
            .get("String")
            .unwrap()
            .clone();
        match db_gral {
            Structure::StructureString(s) => s,
            _ => panic!("Esto no deberia pasar!"),
        }
    }
}

pub trait Storeable {
    fn as_any(&self) -> &dyn Any;
}
