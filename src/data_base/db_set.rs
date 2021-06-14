//use crate::data_base::data_db::Data;
use crate::errors::run_error::RunError;
use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, Mutex};

use std::time::SystemTime;

pub struct DataSet<String> {
    value: BTreeSet<String>,
    time_touch: SystemTime,
}

impl Clone for DataSet<String> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            time_touch: self.time_touch,
        }
    }
}

impl DataSet<String> {
    pub fn new() -> Self {
        Self {
            value: BTreeSet::<String>::new(),
            time_touch: SystemTime::now(),
        }
    }

    pub fn get_value(&self) -> BTreeSet<String> {
        self.value.clone()
    }

    #[allow(dead_code)]
    pub fn get_time_touch(&self) -> SystemTime {
        self.time_touch
    }

    pub fn insert_value(&mut self, value: String) -> bool {
        if self.value.insert(value) {
            self.time_touch = SystemTime::now();
            return true;
        }
        false
    }

    pub fn update_touch(&mut self) {
        self.time_touch = SystemTime::now();
    }

    pub fn remove_value(&mut self, value: String) -> bool {
        if self.value.remove(&value) {
            self.time_touch = SystemTime::now();
            return true;
        }
        false
    }
}

pub struct DataBaseSet<String> {
    db_set: Arc<Mutex<HashMap<String, DataSet<String>>>>, //se puede cambiar por un HashSet,
                                                          //cuando se implementen los comandos se ve cuál es más fácil
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
    #[allow(dead_code)]
    pub fn new() -> Self {
        let db_set = Arc::new(Mutex::new(HashMap::new()));
        Self { db_set }
    }

    fn get_value(&self, key: String) -> DataSet<String> {
        let db = self.db_set.lock().unwrap();

        db.get(&key).unwrap().clone() //chequear que esté antes
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn clean_all_data(&self) -> bool {
        let mut db_set = self.db_set.lock().unwrap();
        db_set.clear();
        db_set.is_empty()
    }

    #[allow(dead_code)]
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

    pub fn touch_key(&self, key: String) -> usize {
        if self.contains(key.clone()) {
            self.get_value(key).update_touch();
            return 1;
        }
        0
    }

    pub fn touch(&self, keys: Vec<String>) -> usize {
        let mut cont = 0;
        for key in keys {
            cont += self.touch_key(key);
        }
        cont
    }
}
