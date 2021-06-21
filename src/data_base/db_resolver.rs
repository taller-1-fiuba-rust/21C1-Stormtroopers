use crate::command::constants::{TYPE_LIST, TYPE_SET, TYPE_STRING};
use crate::data_base::db_list::DataBaseList;
use crate::data_base::db_set::DataBaseSet;
use crate::data_base::db_string::DataBaseString;
use crate::errors::run_error::RunError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

const DB_STRING: &str = "String";
const DB_LIST: &str = "List";
const DB_SET: &str = "Set";

const ERROR_MSG_GET_DB: &str = "Error al recuperar el tipo de la db";
#[derive(Clone)]
pub enum DataBase {
    DataBaseString(DataBaseString<String>),
    DataBaseList(DataBaseList<String>),
    DataBaseSet(DataBaseSet<String>),
}

pub struct DataBaseResolver {
    data_base: Arc<Mutex<HashMap<String, DataBase>>>,
}

impl Default for DataBaseResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for DataBaseResolver {
    fn clone(&self) -> Self {
        Self {
            data_base: self.data_base.clone(),
        }
    }
}

impl DataBaseResolver {
    pub fn new() -> Self {
        let data_base = Arc::new(Mutex::new(HashMap::new()));
        Self { data_base }
    }

    pub fn get(&self, name_type: String) -> DataBase {
        let data_base = self.data_base.lock().unwrap();
        data_base.get(&name_type).unwrap().clone()
    }

    pub fn add_data_base(&self, key: String, value: DataBase) {
        let mut data_base_general = self.data_base.lock().unwrap();

        data_base_general.insert(key, value); //no usar unwrap acá porque devuelve None
                                              //la primera vez que se inserta algo en una key, entonces pincha todo
    }

    pub fn dbsize(&self) -> usize {
        let mut cont = 0;
        let data_base = self.data_base.lock().unwrap();
        for (_key, val) in data_base.iter() {
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
        let data_base = self.data_base.lock().unwrap();
        for (_key, val) in data_base.iter() {
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

    pub fn delete_keys(&self, keys: Vec<&str>) -> u32 {
        let databases = self.data_base.lock().unwrap();
        let mut clear_count = 0_u32;
        for db in databases.values() {
            if let DataBase::DataBaseString(mut db_string) = db.clone() {
                clear_count = db_string.delete(keys.clone());
            } else if let DataBase::DataBaseList(mut db_list) = db.clone() {
                clear_count = db_list.delete(keys.clone());
            } else if let DataBase::DataBaseSet(mut db_set) = db.clone() {
                clear_count = db_set.delete(keys.clone());
            }
        }
        clear_count
    }

    pub fn clear_key(&self, key: String) {
        let databases = self.data_base.lock().unwrap();
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
            .data_base
            .lock()
            .unwrap()
            .get(DB_STRING)
            .unwrap()
            .clone();
        match db_gral {
            DataBase::DataBaseString(s) => s,
            _ => panic!("{}", ERROR_MSG_GET_DB),
        }
    }

    pub fn get_list_db(&self) -> DataBaseList<String> {
        let db_gral = self.data_base.lock().unwrap().get(DB_LIST).unwrap().clone();
        match db_gral {
            DataBase::DataBaseList(s) => s,
            _ => panic!("{}", ERROR_MSG_GET_DB),
        }
    }

    pub fn get_set_db(&self) -> DataBaseSet<String> {
        let db_gral = self.data_base.lock().unwrap().get(DB_SET).unwrap().clone();
        match db_gral {
            DataBase::DataBaseSet(s) => s,
            _ => panic!("{}", ERROR_MSG_GET_DB),
        }
    }

    //1. chequeo si existe en list o set -> si no existe, error (aunque exista en strings)
    //2. si existe, le dejo a esa db que la ordene
    pub fn sort(&self, key: String) -> Result<Vec<String>, RunError> {
        let db_list = self.get_list_db();
        let db_set = self.get_set_db();

        if db_list.contains(key.clone()) {
            return db_list.sort(key);
        } else if db_set.contains(key.clone()) {
            return db_set.sort(key);
        }

        Err(RunError {
            message: "Key is not a list or set".to_string(),
            cause: "The key may be a string or may not be in the db\n".to_string(),
        })
    }

    //TODO: Es thread safety esto?
    pub fn check_db_string(&self, key: String) -> bool {
        let db_string = self.get_string_db();
        db_string.contains(key)
    }

    pub fn check_db_list(&self, key: String) -> bool {
        let db_list = self.get_list_db();
        db_list.contains(key)
    }

    pub fn check_db_set(&self, key: String) -> bool {
        let db_set = self.get_set_db();
        db_set.contains(key)
    }

    pub fn type_key(&self, key: String) -> Result<String, RunError> {
        if self.check_db_string(key.clone()) {
            return Ok(TYPE_STRING.to_string());
        } else if self.check_db_list(key.clone()) {
            return Ok(TYPE_LIST.to_string());
        } else if self.check_db_set(key) {
            return Ok(TYPE_SET.to_string());
        }

        Err(RunError {
            message: "Key is not in db".to_string(),
            cause: "First, insert the key in the db".to_string(),
        })
    }

    pub fn touch(&self, keys: Vec<String>) -> usize {
        let db_string = self.get_string_db();
        let db_list = self.get_list_db();
        let db_set = self.get_set_db();

        let mut cont = db_string.touch(keys.clone());
        cont += db_list.touch(keys.clone());
        cont += db_set.touch(keys);

        cont
    }
}
