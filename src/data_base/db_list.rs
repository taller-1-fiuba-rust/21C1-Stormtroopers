use crate::data_base::data_db::data_list::DataList;
use crate::errors::run_error::RunError;
use regex::Regex;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use std::cmp::Ordering;

const SUCCESS: &str = "OK";
const EMPTY_LIST: usize = 0;

pub struct DataBaseList<String> {
    db_list: Arc<Mutex<HashMap<String, DataList<String>>>>,
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

    pub fn delete(&mut self, keys: Vec<&str>) -> u32 {
        let mut count = 0_u32;
        let mut db = self.db_list.lock().unwrap();
        for key in keys.iter() {
            if let Some(_v) = db.remove(*key) {
                count += 1
            }
        }
        count
    }

    pub fn lpush(&self, args: Vec<&str>) -> u32 {
        self.lpush_common(false, args)
    }

    pub fn lpushx(&self, args: Vec<&str>) -> u32 {
        self.lpush_common(true, args)
    }

    pub fn rpop(&self, key_list: String, count: u32) -> Vec<String> {
        let mut db_list = self.db_list.lock().unwrap();
        let mut list: Vec<String> = match db_list.get(&key_list) {
            Some(l) => l.get_value(),
            None => vec![],
        };
        let mut result = vec![];
        for _i in 0..count {
            if let Some(elem) = list.pop() {
                result.push(elem)
            }
        }
        //        result.reverse();
        let mut data = DataList::new();
        data.insert_values(list);
        db_list.insert(key_list, data);

        result
    }

    pub fn lrem(&self, args: Vec<&str>) -> u32 {
        let key_list = args[0];
        let mut count = args[1].parse::<i32>().unwrap();
        let val_rem = args[2];

        let mut db_list = self.db_list.lock().unwrap();

        let mut list: Vec<String> = match db_list.get(key_list) {
            Some(l) => l.get_value(),
            None => vec![],
        };

        match count.cmp(&0) {
            Ordering::Greater => {}
            Ordering::Less => {
                list.reverse();
                count = count.abs();
            }
            Ordering::Equal => count = -1,
        }

        let mut rem = 0;
        let list_iter = list.clone();
        let mut i = 0;
        for val in list_iter.iter() {
            if val == val_rem {
                list.remove(i);
                rem += 1;
                if rem == count {
                    break;
                }
            } else {
                i += 1;
            }
        }
        if count < 0 {
            list.reverse()
        };
        let mut data = DataList::new();
        data.insert_values(list);
        db_list.insert(String::from(key_list), data);

        rem as u32
    }

    //TODO: ver impl con nros negativos! ???
    pub fn lrange(&self, args: Vec<&str>) -> Vec<String> {
        let key = args[0];
        let db_list = self.db_list.lock().unwrap();
        let list: Vec<String> = match db_list.get(key) {
            Some(l) => l.get_value(),
            None => return vec![],
        };

        let mut rini_i32 = args[1].parse::<i32>().unwrap();
        let mut rend_i32 = args[2].parse::<i32>().unwrap();
        if rini_i32 < 0 {
            rini_i32 += list.len() as i32;
        }
        if rend_i32 < 0 {
            rend_i32 += list.len() as i32;
        }
        if rini_i32 > rend_i32 || (rini_i32 < 0 && rend_i32 < 0) {
            return vec![];
        }
        if rend_i32 > (list.len() as i32 - 1) {
            rend_i32 = list.len() as i32 - 1;
        }
        let rini = rini_i32 as usize;
        let rend = rend_i32 as usize;

        list[rini..=rend].to_vec()
    }

    pub fn lpop(&self, args: Vec<&str>) -> Vec<String> {
        let key = args[0];
        let mut db_list = self.db_list.lock().unwrap();
        let mut list: Vec<String> = match db_list.get(key) {
            Some(l) => l.get_value(),
            None => return vec![],
        };

        let item = list.remove(0);

        let mut data = DataList::new();
        data.insert_values(list);

        db_list.insert(String::from(key), data);

        vec![item]
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

    pub fn llen(&self, key: String) -> Result<usize, RunError> {
        let db = self.db_list.lock().unwrap();
        if let Some(list) = db.get(&key) {
            return Ok(list.get_value().len());
        }

        Ok(EMPTY_LIST) //lista vacía por no existir
    }

    pub fn lset(&self, key: String, pos: String, value: String) -> Result<String, RunError> {
        let position = self.validate_pos(pos)?;
        self.validate_pos_and_len(key.clone(), &position)?;
        self.validate_or_insert_key(key.clone());

        self.insert_and_remove_value_in_pos(key, position as usize, value);
        Ok(SUCCESS.to_string())
    }

    //TODO: falta implementar bien el retorno del size de la lista un vez insertado los valores.
    //TODO: OJO que no es completamente thread safety!
    pub fn rpushx(&self, key_list: String, values: Vec<&str>) -> Result<u32, RunError> {
        {
            let db_list = self.db_list.lock().unwrap();
            let list: Vec<String> = match db_list.get(&key_list) {
                Some(l) => l.get_value(),
                None => return Ok(0_u32),
            };
            let inserted = 0_u32;
            if list.is_empty() {
                return Ok(inserted);
            }
        }

        self.rpush(key_list, values.clone())?;

        Ok(values.len() as u32)
    }

    pub fn rpush(&self, key: String, values: Vec<&str>) -> Result<String, RunError> {
        for value in values {
            self.insert_value(key.clone(), value.to_string().clone());
        }
        Ok(SUCCESS.to_string())
    }

    pub fn contains(&self, key: String) -> bool {
        let db = self.db_list.lock().unwrap().clone();
        db.contains_key(&key)
    }

    pub fn sort(&self, key: String) -> Result<Vec<String>, RunError> {
        if let Ok(list) = self.get_list(key) {
            let mut list_client = list;
            list_client.sort();
            return Ok(list_client);
        }

        Err(RunError {
            message: "Key is not a list".to_string(),
            cause: "The key may be a string/set or may not be in the db\n".to_string(),
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

    fn lpush_common(&self, use_x: bool, mut args: Vec<&str>) -> u32 {
        let mut db_list = self.db_list.lock().unwrap();

        let key = args.remove(0);
        if use_x && !db_list.contains_key(&String::from(key)) {
            return 0;
        }
        args.reverse();
        let vec_values = db_list
            .entry(String::from(key))
            .or_insert_with(DataList::new);
        let mut insertions = 0_u32;
        for val in args.iter() {
            vec_values.insert_value(String::from(*val));
            insertions += 1;
        }
        insertions
    }

    fn get_value(&self, key: String) -> DataList<String> {
        let db = self.db_list.lock().unwrap();
        db.get(&key).unwrap().clone() //chequear que este OK
    }

    fn get_list(&self, key: String) -> Result<Vec<String>, RunError> {
        self.validate_or_insert_key(key.clone());

        let db = self.db_list.lock().unwrap();
        if let Some(list) = db.get(&key) {
            return Ok(list.get_value().to_vec());
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

    fn validate_pos(&self, pos: String) -> Result<i32, RunError> {
        if let Ok(val) = pos.parse::<i32>() {
            Ok(val)
        } else {
            Err(RunError {
                message: "Position is not an integer".to_string(),
                cause: "The argument cannot be interpreted as an integer".to_string(),
            })
        }
    }

    fn validate_pos_and_len(&self, key: String, position: &i32) -> Result<(), RunError> {
        let list = self.get_list(key)?;
        if (*position as usize) >= list.len() {
            return Err(RunError {
                message: "Position is bigger than the len of the list".to_string(),
                cause: "The argument exceeds the limits of the list".to_string(),
            });
        }
        Ok(())
    }

    fn validate_or_insert_key(&self, key: String) -> bool {
        //devuelve true si existía previamente
        let mut db = self.db_list.lock().unwrap();
        if db.contains_key(&key) {
            return true;
        }

        db.insert(key, DataList::new());
        false
    }

    fn insert_values(&self, key: String, values: Vec<String>) {
        let mut db = self.db_list.lock().unwrap();
        let mut list = DataList::new();
        list.insert_values(values);
        db.insert(key, list);
    }

    fn insert_value_in_pos(&self, key: String, pos: usize, value: String) {
        let mut list = self.get_list(key.clone()).unwrap();
        list.insert(pos, value);
        self.insert_values(key, list.to_vec());
    }

    fn remove_value_in_pos(&self, key: String, pos: usize) {
        let mut list = self.get_list(key.clone()).unwrap();
        list.remove(pos);
        self.insert_values(key, list.to_vec());
    }

    fn insert_and_remove_value_in_pos(&self, key: String, pos: usize, value: String) {
        self.remove_value_in_pos(key.clone(), pos);
        self.insert_value_in_pos(key, pos, value);
    }

    fn insert_value(&self, key: String, value: String) {
        self.validate_or_insert_key(key.clone());
        let mut db = self.db_list.lock().unwrap();
        let mut list = db.get(&key).unwrap().clone(); //sé que existe, porque la validé o inserté antes
        list.insert_value(value);
        db.insert(key, list);
    }

    fn parse_data(&self, list: Vec<String>) -> String {
        let mut parsed_data = String::from("");
        for item in list.iter() {
            parsed_data.push_str(&(format!("{}\t", item)));
        }
        parsed_data
    }

    pub fn get_all_data(&self) -> String {
        let db = self.db_list.lock().unwrap().clone();
        let mut data = String::from("");
        for (key, value) in &db {
            let list = value.get_value();
            let aux = format!("List\t{}\t{}\n", key, self.parse_data(list.clone()));
            data.push_str(&aux);
        }
        data
    }
  
    pub fn keys(&self, pattern: &str) -> Vec<String> {
        let mut keys_vec = Vec::<String>::new();
        let db = self.db_list.lock().unwrap();
        let re = Regex::new(pattern).unwrap();

        for key in db.keys() {
            if re.is_match(&key) {
                keys_vec.push((*(key.clone())).to_string());
            }
        }

        keys_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lpush_test() {
        let db = DataBaseList::new();
        let mut count;
        count = db.lpush(vec!["key0", "val0", "val1"]);

        assert!(count == 2);

        count = db.lpush(vec!["key1", "val0"]);
        assert!(count == 1);

        count = db.lpush(vec!["key1"]);
        assert!(count == 0);
    }

    #[test]
    fn delete_test() {
        let mut db = DataBaseList::new();

        let mut count;
        let mut cpush;
        count = db.delete(vec!["key0"]);
        assert!(count == 0);

        cpush = db.lpush(vec!["key0", "val0"]);
        assert!(cpush == 1);
        count = db.delete(vec!["key0"]);
        assert!(count == 1);
        let crange = db.lrange(vec!["key0", "0", "-1"]);
        assert!(crange.len() == 0);

        cpush = db.lpush(vec!["key0", "val0", "val1"]);
        assert!(cpush == 2);
        cpush = db.lpush(vec!["key1", "val0", "val1"]);
        assert!(cpush == 2);
        count = db.delete(vec!["key0", "key1"]);
        assert!(count == 2);
    }

    #[test]
    fn rpop_test() {
        let db = DataBaseList::new();
        let mut vec;
        vec = db.rpop("empty".to_string(), 0);
        assert!(vec.len() == 0);

        db.lpush(vec!["key0", "val2", "val1", "val0"]);
        vec = db.rpop("key0".to_string(), 2);
        assert!(vec.len() == 2);
        println!("{}", vec[0]);
        println!("{}", vec[1]);
        assert!(vec[0].eq("val2"));
        assert!(vec[1].eq(&"val1"));
    }

    #[test]
    fn lrem_test() {
        let db = DataBaseList::new();
        let mut res;
        res = db.lrem(vec!["key0", "0", "val0"]);
        assert!(res == 0);

        db.lpush(vec!["key0", "val", "val2", "val", "val0", "val"]);
        res = db.lrem(vec!["key0", "-2", "val"]);
        assert!(res == 2);
    }
}
