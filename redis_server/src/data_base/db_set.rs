//! Database structure in charge of storing and processing Sets.
use crate::data_base::data_db::data_set::DataSet;
use crate::data_base::db_list::{list_can_be_parsed, parse_list, sort_parsed_list};
use crate::errors::run_error::RunError;
use regex::Regex;
use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, Mutex};

pub struct DataBaseSet<String> {
    db_set: Arc<Mutex<HashMap<String, DataSet<String>>>>,
}

impl Default for DataBaseSet<String> {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for DataBaseSet<String> {
    fn clone(&self) -> Self {
        let db_set = self.db_set.clone();
        Self { db_set }
    }
}

impl DataBaseSet<String> {
    pub fn new() -> Self {
        let db_set = Arc::new(Mutex::new(HashMap::new()));
        Self { db_set }
    }

    //ya está antes, es un método que sirve para modularizar (por eso el unwrap)
    fn get_value(&self, key: String) -> DataSet<String> {
        let db = self.db_set.lock().unwrap();

        db.get(&key).unwrap().clone()
    }

    pub fn delete(&mut self, args: Vec<&str>) -> u32 {
        let mut count = 0_u32;
        let mut db = self.db_set.lock().unwrap();
        for key in args.iter() {
            if let Some(_v) = db.remove(*key) {
                count += 1
            }
        }
        count
    }

    pub fn get_del(&mut self, key: String) -> Result<BTreeSet<String>, RunError> {
        let set = self.get_set(key.clone())?;
        self.delete(vec![&key]);
        Ok(set)
    }

    pub fn sadd(&self, mut args: Vec<&str>) -> u32 {
        let mut db_set = self.db_set.lock().unwrap();
        let key = args.remove(0);

        let mut insertion;
        let set_st = db_set.entry(String::from(key)).or_insert_with(DataSet::new);
        let mut i = 0_u32;
        for val in args {
            insertion = set_st.insert_value(val.to_string());
            if insertion {
                i += 1;
            }
        }
        i
    }

    pub fn smembers(&self, mut args: Vec<&str>) -> Vec<String> {
        let db_set = self.db_set.lock().unwrap();
        let key = args.remove(0);
        let set_st = match db_set.get(&key.to_string()) {
            Some(set) => set,
            None => return vec![],
        };

        let mut values: Vec<String> = vec![];
        for val in set_st.get_value().iter() {
            values.push(val.clone());
        }
        values
    }

    pub fn scard(&self, mut args: Vec<&str>) -> u32 {
        let db_set = self.db_set.lock().unwrap();
        let key = args.remove(0);

        return match db_set.get(&key.to_string()) {
            Some(set) => set.get_value().len() as u32,
            None => 0_u32,
        };
    }

    pub fn sismember(&self, args: Vec<&str>) -> u8 {
        let db_set = self.db_set.lock().unwrap();
        let key = args[0];
        let value = args[1];

        return match db_set.get(&key.to_string()) {
            Some(set) => match set.get_value().get(value) {
                Some(_v) => 1,
                None => 0,
            },
            None => return 0,
        };
    }

    pub fn srem(&self, mut args: Vec<&str>) -> u32 {
        let mut db_set = self.db_set.lock().unwrap();
        let key = args.remove(0);

        let set_st = db_set.entry(String::from(key)).or_insert_with(DataSet::new);
        let mut rem;
        let mut i = 0_u32;
        for val in args {
            rem = set_st.remove_value(val.to_string());
            if rem {
                i += 1;
            }
        }
        i
    }

    pub fn clear_key(&self, key: String) {
        let mut db = self.db_set.lock().unwrap().clone();
        if db.contains_key(&key) {
            db.remove(&key);
        }
    }

    pub fn get_set(&self, key: String) -> Result<BTreeSet<String>, RunError> {
        let db = self.db_set.lock().unwrap();
        if let Some(set) = db.get(&key) {
            return Ok(set.get_value());
        }

        Err(RunError {
            message: "Key is not a set".to_string(),
            cause: "The key may be a string/list or may not be in the db\n".to_string(),
        })
    }

    pub fn clean_all_data(&self) -> bool {
        let mut db_set = self.db_set.lock().unwrap();
        db_set.clear();
        db_set.is_empty()
    }

    pub fn dbsize(&self) -> usize {
        let db_set = self.db_set.lock().unwrap();
        db_set.len()
    }

    pub fn contains(&self, key: String) -> bool {
        let db = self.db_set.lock().unwrap().clone();
        db.contains_key(&key)
    }

    fn set_to_list_sorted(&self, set: BTreeSet<String>) -> Vec<String> {
        let mut list = Vec::<String>::new();
        for elem in set {
            list.push(elem);
        }
        if list_can_be_parsed(list.clone()) {
            let response = parse_list(sort_parsed_list(list));
            return response;
        }
        list.sort();
        list
    }

    pub fn sort(&self, key: String) -> Result<Vec<String>, RunError> {
        if let Ok(set_client) = self.get_set(key) {
            return Ok(self.set_to_list_sorted(set_client));
        }

        Err(RunError {
            message: "Key is not a set".to_string(),
            cause: "The key may be a string/list or may not be in the db\n".to_string(),
        })
    }

    pub fn touch_key(&self, key: String) -> u64 {
        if self.contains(key.clone()) {
            return self.get_value(key).update_touch();
        }
        0
    }

    fn parse_data(&self, set: BTreeSet<String>) -> String {
        let mut parsed_data = String::from("");
        for item in set.iter() {
            parsed_data.push_str(&(format!("{}\t", item)));
        }
        parsed_data
    }

    pub fn get_all_data(&self) -> String {
        let db = self.db_set.lock().unwrap().clone();
        let mut data = String::from("");
        for (key, value) in &db {
            let set = value.get_value();
            let aux = format!("Set\t{}\t{}\n", key, self.parse_data(set.clone()));
            data.push_str(aux.as_str());
        }
        data
    }

    fn return_all_keys(&self) -> Result<Vec<String>, RunError> {
        let mut response = vec![];
        let hash;
        if let Ok(val) = self.db_set.lock() {
            hash = val;
        } else {
            return Err(RunError {
                message: "Could not lock the data base".to_string(),
                cause: "Race condition\n".to_string(),
            });
        }

        for key in hash.keys() {
            response.push(key.clone());
        }

        Ok(response)
    }

    pub fn keys(&self, pattern: &str) -> Result<Vec<String>, RunError> {
        if pattern == "*" {
            return self.return_all_keys();
        }
        let mut keys_vec = Vec::<String>::new();
        let db = self.db_set.lock().unwrap();
        match Regex::new(pattern) {
            Ok(re) => {
                for key in db.keys() {
                    if re.is_match(&key) {
                        keys_vec.push((*(key.clone())).to_string());
                    }
                }

                Ok(keys_vec)
            }
            Err(_) => Err(RunError {
                message: "Could not find match".to_string(),
                cause: "Malformed pattern\n".to_string(),
            }),
        }
    }
}
