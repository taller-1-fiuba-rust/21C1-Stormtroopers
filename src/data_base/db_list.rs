use crate::errors::run_error::RunError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

const SUCCESS: &str = "OK";
const EMPTY_LIST: usize = 0;

pub struct DataBaseList<String> {
    db_list: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl Default for DataBaseList<String> {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for DataBaseList<String> {
    fn clone(&self) -> Self {
        let db_list = self.db_list.clone();
        Self { db_list }
    }
}

impl DataBaseList<String> {
    pub fn new() -> Self {
        let db_list = Arc::new(Mutex::new(HashMap::new()));
        Self { db_list }
    }

    #[allow(dead_code)]
    pub fn lpush(&self, key: String, value: String) {
        let mut db_list = self.db_list.lock().unwrap();

        let vec_values = db_list.entry(key).or_insert_with(Vec::<String>::new);
        vec_values.push(value);
    }

    pub fn clear_key(&self, key: String) {
        let mut db = self.db_list.lock().unwrap().clone();
        if db.contains_key(&key) {
            db.remove(&key);
        }
    }

    #[allow(dead_code)]
    pub fn clean_all_data(&self) -> bool {
        let mut db_list = self.db_list.lock().unwrap();
        db_list.clear();
        db_list.is_empty()
    }

    #[allow(dead_code)]
    pub fn dbsize(&self) -> usize {
        let db_list = self.db_list.lock().unwrap();
        db_list.len()
    }

    fn get_list(&self, key: String) -> Result<Vec<String>, RunError> {
        self.validate_or_insert_key(key.clone());

        let db = self.db_list.lock().unwrap();
        if let Some(list) = db.get(&key) {
            return Ok(list.to_vec());
        }

        Err(RunError {
            message: "Key not in db".to_string(),
            cause: "First, insert the key in db\n".to_string(),
        })
    }

    fn get_in_pos(&self, key: String, pos: usize) -> Result<String, RunError> {
        let list = self.get_list(key)?;
        Ok(list[pos].clone())
    }

    pub fn llen(&self, key: String) -> Result<usize, RunError> {
        let db = self.db_list.lock().unwrap();
        if let Some(list) = db.get(&key) {
            return Ok(list.len());
        }

        Ok(EMPTY_LIST) //lista vacía por no existir
    }

    fn validate_pos(&self, pos: String) -> Result<i32, RunError> {
        if let Ok(val) = pos.parse::<i32>() {
            Ok(val)
        } else {
            Err(RunError {
                message: "Position is not an integer".to_string(),
                cause: "The argument cannot be interpreted as an integer\n".to_string(),
            })
        }
    }

    fn validate_pos_and_len(&self, key: String, position: &i32) -> Result<(), RunError> {
        let list = self.get_list(key)?;
        if (*position as usize) >= list.len() {
            return Err(RunError {
                message: "Position is bigger than the len of the list".to_string(),
                cause: "The argument exceeds the limits of the list\n".to_string(),
            });
        }
        Ok(())
    }

    pub fn lindex(&self, key: String, pos: String) -> Result<String, RunError> {
        let position = self.validate_pos(pos)?;

        if position >= 0 {
            self.validate_pos_and_len(key.clone(), &position)?;
            self.get_in_pos(key, position as usize)
        } else {
            self.validate_pos_and_len(key.clone(), &(-position))?;
            let len = self.llen(key.clone()).unwrap();
            self.get_in_pos(key, ((len as i32) + position) as usize)
        }
    }

    fn validate_or_insert_key(&self, key: String) -> bool {
        //devuelve true si existía previamente
        let mut db = self.db_list.lock().unwrap();
        if db.contains_key(&key) {
            return true;
        }

        db.insert(key, Vec::<String>::new());
        false
    }

    pub fn lset(&self, key: String, pos: String, value: String) -> Result<String, RunError> {
        let position = self.validate_pos(pos)?;
        self.validate_pos_and_len(key.clone(), &position)?;
        self.validate_or_insert_key(key.clone());

        self.insert_and_remove_value_in_pos(key, position as usize, value);
        Ok(SUCCESS.to_string())
    }

    fn insert(&self, key: String, value: Vec<String>) {
        let mut db = self.db_list.lock().unwrap();
        db.insert(key, value);
    }

    fn insert_value_in_pos(&self, key: String, pos: usize, value: String) {
        let mut list = self.get_list(key.clone()).unwrap();
        list.insert(pos, value);
        self.insert(key, list.to_vec());
    }

    fn remove_value_in_pos(&self, key: String, pos: usize) {
        let mut list = self.get_list(key.clone()).unwrap();
        list.remove(pos);
        self.insert(key, list.to_vec());
    }

    fn insert_and_remove_value_in_pos(&self, key: String, pos: usize, value: String) {
        self.remove_value_in_pos(key.clone(), pos);
        self.insert_value_in_pos(key, pos, value);
    }

    fn insert_value(&self, key: String, value: String) {
        self.validate_or_insert_key(key.clone());
        let mut db = self.db_list.lock().unwrap();
        let mut list = db.get(&key).unwrap().clone(); //sé que existe, porque la validé o inserté antes
        list.push(value);
        db.insert(key, list.to_vec());
    }

    pub fn rpush(&self, key: String, values: Vec<&str>) -> Result<String, RunError> {
        for value in values {
            self.insert_value(key.clone(), value.to_string().clone());
        }
        Ok(SUCCESS.to_string())
    }
}
