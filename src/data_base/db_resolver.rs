use crate::constants::{SHARING_COUNT_DEFAULT, TYPE_LIST, TYPE_SET, TYPE_STRING};
use crate::data_base::db_list::DataBaseList;
use crate::data_base::db_set::DataBaseSet;
use crate::data_base::db_string::DataBaseString;
use crate::errors::run_error::RunError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

const ERROR_MSG_GET_DB: &str = "Error al recuperar el tipo de la db";

#[derive(Clone)]
pub enum DataBase {
    DataBaseString(DataBaseString<String>),
    DataBaseList(DataBaseList<String>),
    DataBaseSet(DataBaseSet<String>),
}

pub struct DataBaseResolver {
    data_bases: Arc<Mutex<HashMap<String, Vec<DataBase>>>>,
    data_bases2: Arc<Mutex<HashMap<String, DataBase>>>,
    sharing_count_db: u32,
}

impl Default for DataBaseResolver {
    fn default() -> Self {
        Self::new(SHARING_COUNT_DEFAULT)
    }
}

impl Clone for DataBaseResolver {
    fn clone(&self) -> Self {
        Self {
            data_bases: self.data_bases.clone(),
            data_bases2: self.data_bases2.clone(),
            sharing_count_db: self.sharing_count_db,
        }
    }
}

impl DataBaseResolver {
    pub fn new(sharing_count_db: u32) -> Self {
        let data_base = Arc::new(Mutex::new(HashMap::new()));
        let data_base2 = Arc::new(Mutex::new(HashMap::new()));
        Self {
            data_bases: data_base,
            data_bases2: data_base2,
            sharing_count_db,
        }
    }

    /*
       pub fn get(&self, name_type: String) -> DataBase {
           let data_bases = self.data_bases.lock().unwrap();
           //data_base.get(&name_type).unwrap().clone()
           data_bases.get(&name_type).unwrap()[0].clone()
       }
    */

    pub fn add_data_base(&self, key_db: String, values: Vec<DataBase>) {
        let mut data_base_general = self.data_bases.lock().unwrap();

        data_base_general.insert(key_db, values); //no usar unwrap acá porque devuelve None
                                                  //la primera vez que se inserta algo en una key, entonces pincha todo
    }

    pub fn dbsize(&self) -> usize {
        let mut cont = 0;
        let data_base = self.data_bases.lock().unwrap();
        for (_key, val) in data_base.iter() {
            let val0 = val[0].clone();
            match val0 {
                DataBase::DataBaseString(db_string) => {
                    cont += db_string.dbsize();
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
        let data_bases = self.data_bases.lock().unwrap();
        for (_key, dbs) in data_bases.iter() {
            for i in 0..self.sharing_count_db {
                match dbs[i as usize].clone() {
                    DataBase::DataBaseString(string) => {
                        response &= string.clean_all_data();
                    }
                    DataBase::DataBaseSet(set) => {
                        response &= set.clean_all_data();
                    }
                    DataBase::DataBaseList(list) => {
                        response &= list.clean_all_data();
                    }
                    #[allow(unreachable_patterns)]
                    _ => {
                        response = false;
                    }
                }
            }
        }
        response
    }

    pub fn delete_keys(&self, keys: Vec<&str>) -> u32 {
        let databases = self.data_bases.lock().unwrap();
        let mut clear_count = 0_u32;
        for key in keys.clone() {
            for dbs in databases.values() {
                let idx = self.retrieve_index(&(*key));
                if let DataBase::DataBaseString(mut db_string) = dbs[idx].clone() {
                    clear_count += db_string.delete(vec![&(*key)]);
                } else if let DataBase::DataBaseList(mut db_list) = dbs[idx].clone() {
                    clear_count += db_list.delete(vec![&(*key)]);
                } else if let DataBase::DataBaseSet(mut db_set) = dbs[idx].clone() {
                    clear_count += db_set.delete(vec![&(*key)]);
                }
            }
        }
        clear_count
    }

    pub fn clear_key(&self, key: String) {
        let databases = self.data_bases.lock().unwrap();
        for dbs in databases.values() {
            let idx = self.retrieve_index(key.as_str());
            if let DataBase::DataBaseString(db_string) = dbs[idx].clone() {
                db_string.clear_key(key.clone());
            } else if let DataBase::DataBaseList(db_list) = dbs[idx].clone() {
                db_list.clear_key(key.clone());
            } else if let DataBase::DataBaseSet(db_set) = dbs[idx].clone() {
                db_set.clear_key(key.clone());
            }
        }
    }

    pub fn get_string_db_sharding(&self, key: &str) -> DataBaseString<String> {
        let dbs = self
            .data_bases
            .lock()
            .unwrap()
            .get(TYPE_STRING)
            .unwrap()
            .clone();

        let index_sharing = self.retrieve_index(key);

        match dbs[index_sharing].clone() {
            DataBase::DataBaseString(s) => s,
            _ => panic!("{}", ERROR_MSG_GET_DB),
        }
    }

    pub fn get_set_db_sharding(&self, key: &str) -> DataBaseSet<String> {
        let dbs = self
            .data_bases
            .lock()
            .unwrap()
            .get(TYPE_SET)
            .unwrap()
            .clone();
        let index_sharing = self.retrieve_index(key);
        match dbs[index_sharing].clone() {
            DataBase::DataBaseSet(s) => s,
            _ => panic!("{}", ERROR_MSG_GET_DB),
        }
    }

    /*
        #[deprecated]
    pub fn get_string_db(&self) -> DataBaseString<String> {
        let db_gral = self
            .data_bases
            .lock()
            .unwrap()
            .get(TYPE_STRING)
            .unwrap()
            .clone();
        match db_gral {
            DataBase::DataBaseString(s) => s,
            _ => panic!("{}", ERROR_MSG_GET_DB),
        }
    }
     */

    pub fn get_list_db(&self) -> DataBaseList<String> {
        let db_gral = self
            .data_bases2
            .lock()
            .unwrap()
            .get(TYPE_LIST)
            .unwrap()
            .clone();
        match db_gral {
            DataBase::DataBaseList(s) => s,
            _ => panic!("{}", ERROR_MSG_GET_DB),
        }
    }

    pub fn get_set_db(&self) -> DataBaseSet<String> {
        let db_gral = self
            .data_bases2
            .lock()
            .unwrap()
            .get(TYPE_SET)
            .unwrap()
            .clone();
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

    //TODO: threadsafety?
    pub fn check_db_string(&self, key: String) -> bool {
        //let db_string = self.get_string_db();
        let db_string = self.get_string_db_sharding(key.as_str());
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
        //let db_string = self.get_string_db();
        let db_list = self.get_list_db();
        let db_set = self.get_set_db();

        let mut count = 0;
        for key in keys.clone() {
            let db_string = self.get_string_db_sharding(key.as_str());
            count = db_string.touch_key(key.clone());
        }

        count += db_list.touch(keys.clone());
        count += db_set.touch(keys);

        count
    }

    fn retrieve_index(&self, key: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        hasher.write(key.to_string().as_bytes());
        let nh = hasher.finish() as u32;
        println!("Hash retrieve: {}", nh);

        let idx = nh % self.sharing_count_db;

        println!("Hash index: {}", idx);

        idx as usize
    }

    pub fn get_snapshot(&self) -> String {
        String::from("This is the data!")
    }
}
