use crate::db_list::DataBaseList;
use crate::db_set::DataBaseSet;
use crate::db_string::DataBaseString;
//use crate::errors::run_error::RunError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

const DB_STRING: &str = "String";
const DB_LIST: &str = "List";
const DB_SET: &str = "Set";

#[derive(Clone)]
pub enum DataBase {
    DataBaseString(DataBaseString<String>),
    DataBaseList(DataBaseList<String>),
    DataBaseSet(DataBaseSet<String>),
}

pub struct DataBaseResolver {
    structure: Arc<Mutex<HashMap<String, DataBase>>>,
}

impl Default for DataBaseResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for DataBaseResolver {
    fn clone(&self) -> Self {
        Self {
            structure: self.structure.clone(),
        }
    }
}

impl DataBaseResolver {
    pub fn new() -> Self {
        let structure = Arc::new(Mutex::new(HashMap::new()));
        Self { structure }
    }

    pub fn get(&self, name_type: String) -> DataBase {
        let structure = self.structure.lock().unwrap();
        structure.get(&name_type).unwrap().clone()
    }

    pub fn add_structure(&self, key: String, value: DataBase) {
        let mut structure_general = self.structure.lock().unwrap();

        structure_general.insert(key, value); //no usar unwrap acá porque devuelve None
                                              //la primera vez que se inserta algo en una key, entonces pincha todo
    }

    pub fn dbsize(&self) -> usize {
        let mut cont = 0;
        let structure = self.structure.lock().unwrap();
        for (_key, val) in structure.iter() {
            match val {
                DataBase::DataBaseString(a) => {
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
                DataBase::DataBaseString(a) => {
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

    pub fn clear_key(&self, key: String) {
        let databases = self.structure.lock().unwrap();
        for db in databases.values() {
            if let DataBase::DataBaseString(db_string) = db {
                db_string.clear_key(key.clone());
            } else if let DataBase::DataBaseList(db_list) = db {
                db_list.clear_key(key.clone());
            } else if let DataBase::DataBaseSet(db_set) = db {
                db_set.clear_key(key.clone());
            }
        }
    }

    pub fn get_string_db(&self) -> DataBaseString<String> {
        let db_gral = self
            .structure
            .lock()
            .unwrap()
            .get(DB_STRING)
            .unwrap()
            .clone();
        match db_gral {
            DataBase::DataBaseString(s) => s,
            _ => panic!("Esto no deberia pasar!"),
        }
    }

    pub fn get_list_db(&self) -> DataBaseList<String> {
        let db_gral = self.structure.lock().unwrap().get(DB_LIST).unwrap().clone();
        match db_gral {
            DataBase::DataBaseList(s) => s,
            _ => panic!("Esto no deberia pasar!"),
        }
    }

    pub fn get_set_db(&self) -> DataBaseSet<String> {
        let db_gral = self.structure.lock().unwrap().get(DB_SET).unwrap().clone();
        match db_gral {
            DataBase::DataBaseSet(s) => s,
            _ => panic!("Esto no deberia pasar!"),
        }
    }
}
