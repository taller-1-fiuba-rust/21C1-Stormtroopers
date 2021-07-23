//! Redirects the request to the apropiate database, and handles logic that is type agnostic.
use crate::constants::{SHARING_COUNT_DEFAULT, TYPE_LIST, TYPE_SET, TYPE_STRING};
use crate::data_base::db_list::DataBaseList;
use crate::data_base::db_set::DataBaseSet;
use crate::data_base::db_string::DataBaseString;
use crate::errors::run_error::RunError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::server::ttl_scheduler::TtlScheduler;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

const ERROR_MSG_GET_DB: &str = "Failed to retrieve db type";

#[derive(Clone)]
pub enum DataBase {
    DataBaseString(DataBaseString<String>),
    DataBaseList(DataBaseList<String>),
    DataBaseSet(DataBaseSet<String>),
}

pub struct DataBaseResolver {
    data_bases: Arc<Mutex<HashMap<String, Vec<DataBase>>>>,
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
            sharing_count_db: self.sharing_count_db,
        }
    }
}

impl DataBaseResolver {
    pub fn new(sharing_count_db: u32) -> Self {
        let data_base = Arc::new(Mutex::new(HashMap::new()));
        Self {
            data_bases: data_base,
            sharing_count_db,
        }
    }

    pub fn add_data_base(&self, key_db: String, values: Vec<DataBase>) {
        let mut data_base_general = self.data_bases.lock().unwrap();

        data_base_general.insert(key_db, values);
    }

    //tiene que estar el doble for porque antes solo se fijaba en val[0]
    pub fn dbsize(&self) -> usize {
        let mut cont = 0;
        let data_base = self.data_bases.lock().unwrap();
        for (_key, val) in data_base.iter() {
            for item in val {
                match item.clone() {
                    DataBase::DataBaseString(db_string) => {
                        cont += db_string.dbsize();
                    }
                    DataBase::DataBaseList(db_list) => {
                        cont += db_list.dbsize();
                    }
                    DataBase::DataBaseSet(db_set) => {
                        cont += db_set.dbsize();
                    }
                    #[allow(unreachable_patterns)]
                    _ => {
                        panic!("no tiene que pasar");
                    }
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
        let idx = self.retrieve_index(key.as_str());
        for dbs in databases.values() {
            if let DataBase::DataBaseString(db_string) = dbs[idx].clone() {
                db_string.clear_key(key);
                break;
            } else if let DataBase::DataBaseList(db_list) = dbs[idx].clone() {
                db_list.clear_key(key);
                break;
            } else if let DataBase::DataBaseSet(db_set) = dbs[idx].clone() {
                db_set.clear_key(key);
                break;
            }
        }
    }

    fn get_string_db(&self, idx: usize) -> DataBaseString<String> {
        let dbs = self
            .data_bases
            .lock()
            .unwrap()
            .get(TYPE_STRING)
            .unwrap()
            .clone();
        match dbs[idx].clone() {
            DataBase::DataBaseString(s) => s,
            _ => panic!("{}", ERROR_MSG_GET_DB),
        }
    }

    fn get_list_db(&self, idx: usize) -> DataBaseList<String> {
        let dbs = self
            .data_bases
            .lock()
            .unwrap()
            .get(TYPE_LIST)
            .unwrap()
            .clone();
        match dbs[idx].clone() {
            DataBase::DataBaseList(s) => s,
            _ => panic!("{}", ERROR_MSG_GET_DB),
        }
    }

    fn get_set_db(&self, idx: usize) -> DataBaseSet<String> {
        let dbs = self
            .data_bases
            .lock()
            .unwrap()
            .get(TYPE_SET)
            .unwrap()
            .clone();
        match dbs[idx].clone() {
            DataBase::DataBaseSet(s) => s,
            _ => panic!("{}", ERROR_MSG_GET_DB),
        }
    }

    pub fn get_string_db_sharding(&self, key: &str) -> DataBaseString<String> {
        let index_sharing = self.retrieve_index(key);

        let dbs = self
            .data_bases
            .lock()
            .unwrap()
            .get(TYPE_STRING)
            .unwrap()
            .clone();

        match dbs[index_sharing].clone() {
            DataBase::DataBaseString(s) => s,
            _ => panic!("{}", ERROR_MSG_GET_DB),
        }
    }

    pub fn get_set_db_sharding(&self, key: &str) -> DataBaseSet<String> {
        let index_sharing = self.retrieve_index(key);

        let dbs = self
            .data_bases
            .lock()
            .unwrap()
            .get(TYPE_SET)
            .unwrap()
            .clone();

        match dbs[index_sharing].clone() {
            DataBase::DataBaseSet(s) => s,
            _ => panic!("{}", ERROR_MSG_GET_DB),
        }
    }

    pub fn get_list_db_sharding(&self, key: &str) -> DataBaseList<String> {
        let index_sharing = self.retrieve_index(key);

        let dbs = self
            .data_bases
            .lock()
            .unwrap()
            .get(TYPE_LIST)
            .unwrap()
            .clone();

        match dbs[index_sharing].clone() {
            DataBase::DataBaseList(s) => s,
            _ => panic!("{}", ERROR_MSG_GET_DB),
        }
    }

    pub fn sort(&self, key: String) -> Result<Vec<String>, RunError> {
        let db_list = self.get_list_db_sharding(key.as_str());
        let db_set = self.get_set_db_sharding(key.as_str());

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

    pub fn copy(
        &self,
        key_src: &str,
        key_target: &str,
        del_src_key: bool,
        ttl_scheduler: TtlScheduler,
    ) -> Result<u32, RunError> {
        return match self.type_key(key_src.to_string()) {
            Ok(typee) => {
                return match typee.as_str() {
                    "String" => {
                        let value_src;
                        if del_src_key {
                            value_src = match self
                                .get_string_db_sharding(key_src)
                                .get_del(key_src.to_string())
                            {
                                Ok(val) => val,
                                Err(_e) => return Ok(0),
                            };
                        } else {
                            value_src = self
                                .get_string_db_sharding(key_src)
                                .get_string(key_src.to_string());
                        }
                        self.get_string_db_sharding(key_target)
                            .set_string(key_target.to_string(), value_src);

                        //pasarlo al scheduler
                        match ttl_scheduler.get_ttl_key(key_src.to_string()) {
                            Ok(ttl_str) => {
                                match ttl_str.parse::<u64>() {
                                    Ok(ttl) => {
                                        ttl_scheduler
                                            .set_ttl(ttl, String::from(key_target))
                                            .unwrap();
                                        if del_src_key {
                                            ttl_scheduler
                                                .delete_ttl_key(key_src.to_string())
                                                .unwrap_or_else(|_| String::from(""));
                                        }
                                    }
                                    Err(_e) => {
                                        self.get_string_db_sharding(key_target)
                                            .delete(vec![key_target]);
                                        return Ok(0);
                                    }
                                };
                            }
                            Err(_e) => {}
                        }
                        Ok(1)
                    }
                    "List" => return Ok(0),
                    "Set" => return Ok(0),
                    _ => Ok(0),
                };
            }
            Err(_e) => Err(_e),
        };
    }

    pub fn valid_key_type(&self, key: &str, key_type: &str) -> Result<bool, RunError> {
        let key_type_db = key_type.to_string();
        match self.type_key(key.to_string()) {
            Ok(db_type) => {
                if db_type == key_type_db {
                    Ok(true)
                } else {
                    Err(RunError {
                        message: "ERR WRONGTYPE.".to_string(),
                        cause: "Operation against a key holding the wrong kind of value"
                            .to_string(),
                    })
                }
            }
            Err(_e) => Ok(false),
        }
    }

    pub fn valid_key_type_lock(&self, _db: &DataBaseString<String>, _key: &str) {}

    //TODO: threadsafety?
    pub fn check_db_string(&self, key: String) -> bool {
        let db_string = self.get_string_db_sharding(key.as_str());
        db_string.contains(key)
    }

    pub fn check_db_list(&self, key: String) -> bool {
        let db_list = self.get_list_db_sharding(key.as_str());
        db_list.contains(key)
    }

    pub fn check_db_set(&self, key: String) -> bool {
        let db_set = self.get_set_db_sharding(key.as_str());
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

    pub fn touch(&self, keys: Vec<String>) -> Vec<u64> {
        let mut vec = vec![];
        for key in keys.clone() {
            let mut count = 0;
            let db_string = self.get_string_db_sharding(key.as_str());
            count += db_string.touch_key(key.clone());
            let db_list = self.get_list_db_sharding(key.as_str());
            count += db_list.touch_key(key.clone());
            let db_set = self.get_set_db_sharding(key.as_str());
            count += db_set.touch_key(key.clone());

            vec.push(count);
        }
        vec
    }

    fn retrieve_index(&self, key: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        hasher.write(key.to_string().as_bytes());
        let nh = hasher.finish() as u32;

        let idx = nh % self.sharing_count_db;

        idx as usize
    }

    pub fn get_snapshot(&self) -> String {
        let dbs = self.data_bases.lock().unwrap().clone();
        let mut data = String::from("");
        for (_, value) in dbs {
            let mut aux = String::from("");
            for db in value.iter() {
                match db {
                    DataBase::DataBaseString(str_db) => {
                        aux.push_str(&str_db.get_all_data());
                    }
                    DataBase::DataBaseSet(set_db) => {
                        aux.push_str(&set_db.get_all_data());
                    }
                    DataBase::DataBaseList(list_db) => {
                        aux.push_str(&list_db.get_all_data());
                    }
                }
            }
            data.push_str(&aux);
        }
        data
    }

    pub fn keys(&self, pattern: &str) -> Vec<String> {
        let mut keys_vec = Vec::<String>::new();
        for i in 0..self.sharing_count_db {
            keys_vec.extend(self.get_string_db(i as usize).keys(pattern));
            keys_vec.extend(self.get_list_db(i as usize).keys(pattern));
            keys_vec.extend(self.get_set_db(i as usize).keys(pattern));
        }
        keys_vec
    }
}
