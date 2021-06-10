use crate::errors::run_error::RunError;
use std::collections::HashMap;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;

const RESPONSE_NIL: &str = "(nil)";

pub struct DataBaseString<String> {
    db: Arc<Mutex<HashMap<String, String>>>,
    sender: Arc<SyncSender<String>>,
    receiver: Arc<Mutex<Receiver<String>>>,
}

impl Default for DataBaseString<String> {
    fn default() -> Self {
        DataBaseString::new()
    }
}

impl Clone for DataBaseString<String> {
    fn clone(&self) -> Self {
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();
        let db = self.db.clone();
        Self {
            db,
            sender,
            receiver,
        }
    }
}

impl<String> Drop for DataBaseString<String> {
    fn drop(&mut self) {
        drop(self.sender.clone());
    }
}

impl DataBaseString<String> {
    pub fn new() -> Self {
        let db = Arc::new(Mutex::new(HashMap::new()));
        let (sender, receiver) = sync_channel(10000);
        let sender = Arc::new(sender);
        let receiver = Arc::new(Mutex::new(receiver));
        Self {
            db,
            sender,
            receiver,
        }
    }

    pub fn set_string(&self, key: String, value: String) {
        let mut key_val_sender = key;
        key_val_sender.push(':');
        key_val_sender.push_str(&value);
        let mut db = self.clone();
        let mut data = self.db.clone();
        thread::spawn(move || {
            db.sender.send(key_val_sender).unwrap();
            db.save(&mut data);
        })
        .join()
        .unwrap();
    }

    pub fn get_string(&self, key: String) -> String {
        let mut db = self.clone();

        let mut data = self.db.clone();

        let return_res = thread::spawn(move || {
            db.sender.send(key).unwrap();
            db.get(&mut data)
        })
        .join()
        .unwrap();

        return_res
    }

    pub fn clear_key(&self, key: String) {
        let mut db = self.db.lock().unwrap().clone();
        if db.contains_key(&key) {
            db.remove(&key);
        }
    }

    pub fn mset(&self, keys: Vec<&str>) {
        let mut db = self.clone();
        let keys_sender = DataBaseString::vec_to_str(keys);

        thread::spawn(move || {
            db.sender.send(keys_sender).unwrap();
            db.mset_string();
        })
        .join()
        .unwrap();
    }

    pub fn mget(&self, keys: Vec<&str>) -> Vec<String> {
        let mut db = self.clone();
        let mut keys_sender = String::from("");
        for key in keys.iter() {
            keys_sender.push_str(key);
            keys_sender.push(':');
        }
        keys_sender.pop();
        let to_return = thread::spawn(move || {
            db.sender.send(keys_sender).unwrap();
            db.mget_string()
        })
        .join()
        .unwrap();

        to_return
    }

    #[allow(dead_code)]
    pub fn clean_all_data(&self) -> bool {
        let mut db_string = self.clone();
        let mut data = self.db.clone();
        thread::spawn(move || {
            db_string.sender.send(String::from("")).unwrap();
            db_string.clean(&mut data);
        })
        .join()
        .unwrap();
        self.db.lock().unwrap().is_empty()
    }

    #[allow(dead_code)]
    //TODO: ver esta impl
    pub fn dbsize(&self) -> usize {
        self.db.lock().unwrap().len()
    }

    pub fn delete(&mut self, args: Vec<&str>) -> u32 {
        let mut count = 0_u32;
        let mut db = self.db.lock().unwrap();
        for key in args.iter() {
            if let Some(_v) = db.remove(*key) {
                count += 1
            }
        }
        count
    }

    pub fn get_del(&mut self, key: String) -> Result<String, RunError> {
        if self.contains(key.clone()) {
            let mut db = self.db.lock().unwrap();
            return Ok(db.remove(&key).unwrap()); //ya sabemos que está, ese unwrap está bien
        }
        Err(RunError {
            message: "Error getting the key".to_string(),
            cause: "Key doesn't exist".to_string(),
        })
    }

    pub fn get_set(&mut self, key: String, new_value: String) -> Result<String, RunError> {
        if self.contains(key.clone()) {
            let mut db = self.db.lock().unwrap();
            let value = db.remove(&key).unwrap(); //ya sabemos que está, ese unwrap está bien
            db.insert(key, new_value);

            return Ok(value);
        }
        Err(RunError {
            message: "Error getting the key".to_string(),
            cause: "Key doesn't exist".to_string(),
        })
    }

    pub fn copy(&mut self, src_key: String, target: String) -> u32 {
        let src_val = self.get_string(src_key);
        if src_val == *RESPONSE_NIL {
            return 0;
        }
        self.set_string(target, src_val);
        1
    }

    pub fn exists(&self, keys: Vec<&str>) -> u32 {
        let mut count = 0_u32;
        let db = self.db.lock().unwrap();
        for key in keys.iter() {
            if db.contains_key(*key) {
                count += 1
            }
        }
        count
    }

    pub fn contains(&self, key: String) -> bool {
        let db = self.db.lock().unwrap().clone();
        db.contains_key(&key)
    }

    fn key_incr(&self, key: String, incr: i32) -> Result<i32, RunError> {
        if !self.contains(key.clone()) {
            self.set_string(key.clone(), "0".to_string());
        }

        let mut value;
        if let Ok(val) = self.get_string(key.clone()).parse::<i32>() {
            value = val;
        } else {
            return Err(RunError {
                message: "Error when increment/decrement a value".to_string(),
                cause: "The value for that key cannot be interpreted as an integer".to_string(),
            });
        }

        value += incr;

        self.set_string(key, value.to_string());

        Ok(value)
    }

    pub fn decrby(&self, key: String, decrement: String) -> Result<i32, RunError> {
        let dec: i32;
        if let Ok(val) = decrement.parse::<i32>() {
            dec = val;
        } else {
            return Err(RunError {
                message: "Error when increment/decrement a value".to_string(),
                cause: "The argument cannot be interpreted as an integer".to_string(),
            });
        }

        self.key_incr(key, -dec)
    }

    pub fn incrby(&self, key: String, increment: String) -> Result<i32, RunError> {
        let inc: i32;
        if let Ok(val) = increment.parse::<i32>() {
            inc = val;
        } else {
            return Err(RunError {
                message: "Error when increment/decrement a value".to_string(),
                cause: "The argument cannot be interpreted as an integer\n".to_string(),
            });
        }

        self.key_incr(key, inc)
    }

    #[allow(dead_code)]
    pub fn append(&self, key: String, value_append: String) -> u32 {
        let mut value = self.get_string(key.clone());
        if value == *RESPONSE_NIL {
            value = value_append;
            self.set_string(key, value.clone());
        } else {
            value.push_str(&value_append);
            self.set_string(key, value.clone());
        }
        value.chars().count() as u32
    }

    pub fn rename(&mut self, key: String, new_key: String) -> Result<(), RunError> {
        let value = self.get_string(key.clone());
        if value == *RESPONSE_NIL {
            Err(RunError {
                message: "Error Command rename".to_string(),
                cause: "Key does not exist\n".to_string(),
            })
        } else {
            self.delete(vec![key.as_str()]);
            self.set_string(new_key, value);
            Ok(())
        }
    }

    pub fn strlen(&self, key: String) -> u32 {
        let value = self.get_string(key);
        if value == *RESPONSE_NIL {
            return 0;
        }
        value.chars().count() as u32
    }

    fn mget_string(&mut self) -> Vec<String> {
        let keys_sender = self.receiver.lock().unwrap().recv().unwrap();
        let keys_splited: Vec<&str> = keys_sender.split(':').collect();

        let db = self.db.lock().unwrap();
        let mut to_ret = Vec::new();
        let mut resp_get;
        for key in keys_splited.iter() {
            resp_get = match db.get(*key) {
                Some(value) => value.clone(),
                None => String::from(RESPONSE_NIL),
            };
            to_ret.push(resp_get);
        }
        to_ret
    }

    fn mset_string(&mut self) {
        let keys_sender = self.receiver.lock().unwrap().recv().unwrap();
        let keys_splited: Vec<&str> = keys_sender.split(':').collect();
        let mut db = self.db.lock().unwrap();
        for idx in 0..keys_splited.len() / 2 {
            //println!("{} {}", keys_splited[idx], keys_splited[(idx * 2) + 1]);
            db.insert(
                keys_splited[idx * 2].trim().to_string(),
                keys_splited[(idx * 2) + 1].trim().to_string(),
            );
        }
    }

    fn save(&mut self, data: &mut Arc<Mutex<HashMap<String, String>>>) {
        let key_val_sender = self.receiver.lock().unwrap().recv().unwrap();
        let key_val_splited: Vec<&str> = key_val_sender.split(':').collect();

        let mut db = data.lock().unwrap();
        db.insert(
            String::from(key_val_splited[0].trim()),
            String::from(key_val_splited[1].trim()),
        );
    }

    fn get(&mut self, db: &mut Arc<Mutex<HashMap<String, String>>>) -> String {
        let key_val = self.receiver.lock().unwrap().recv().unwrap();

        let data = db.lock().unwrap();
        match data.get(&key_val) {
            Some(value) => value.clone(),
            None => String::from(RESPONSE_NIL),
        }
    }

    #[allow(dead_code)]
    fn clean(&mut self, data: &mut Arc<Mutex<HashMap<String, String>>>) -> bool {
        self.receiver.lock().unwrap().recv().unwrap();
        let mut db = data.lock().unwrap();
        db.clear();
        db.is_empty()
    }

    fn vec_to_str(vec: Vec<&str>) -> String {
        let mut str = String::from("");
        for item in vec.iter() {
            str.push_str(item);
            str.push(':');
        }
        str.pop();
        str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn db_string_test() {
        let db = DataBaseString::new();
        db.set_string(String::from("test"), String::from("1"));
        let res = db.get_string(String::from("test"));

        assert_eq!(res, String::from("1"));
    }

    #[test]
    fn get_test() {
        let db = DataBaseString::new();
        let resp = db.get_string(String::from("key0"));
        assert_eq!(resp, String::from(RESPONSE_NIL));

        db.set_string(String::from("key0"), String::from("val0"));
        let resp2 = db.get_string(String::from("key0"));
        assert_eq!(resp2, String::from("val0"));
    }

    #[test]
    fn clean_all_test() {
        let db_string = DataBaseString::new();
        db_string.set_string(String::from("test"), String::from("1"));
        {
            assert!(!db_string.db.lock().unwrap().is_empty());
        }

        db_string.clean_all_data();
        {
            assert!(db_string.db.lock().unwrap().is_empty());
        }
    }

    #[test]
    fn dbsize_test() {
        let db_string = DataBaseString::new();
        db_string.set_string(String::from("test"), String::from("1"));
        {
            assert!(db_string.dbsize() == 1);
        }
    }

    #[test]
    fn copy_test() {
        let mut db_string = DataBaseString::new();
        let value0 = String::from("value0");
        db_string.set_string(String::from("key0"), value0.clone());
        let res = db_string.copy(String::from("key0"), String::from("key0.bis"));

        assert!(res == 1);
        let value0bis = db_string.get_string(String::from("key0.bis"));
        assert_eq!(value0, value0bis.clone());
    }

    #[test]
    fn exists_test() {
        let db_string = DataBaseString::new();

        let res = db_string.exists(vec!["key"]);

        assert!(res == 0);

        let db_string2 = DataBaseString::new();
        db_string2.set_string(String::from("one_key"), String::from("1"));
        let res = db_string2.exists(vec!["one_key"]);

        assert!(res == 1);

        let db_string2 = DataBaseString::new();
        db_string2.set_string(String::from("one_key"), String::from("1"));
        db_string2.set_string(String::from("two_key"), String::from("2"));
        let res = db_string2.exists(vec!["one_key", "two_key", "other_key"]);

        assert!(res == 2);
    }

    #[test]
    fn delete_test() {
        let mut db_string = DataBaseString::new();

        let mut count;
        count = db_string.delete(vec!["key0"]);
        assert!(count == 0);

        db_string.set_string(String::from("key0"), String::from("val0"));
        count = db_string.delete(vec!["key0"]);

        assert!(count == 1);

        db_string.set_string(String::from("key0"), String::from("val0"));
        db_string.set_string(String::from("key1"), String::from("val1"));
        count = db_string.delete(vec!["key0", "key1", "key2"]);

        assert!(count == 2);
    }

    #[test]
    fn append_test() {
        let db_string = DataBaseString::new();
        let mut len_val;
        len_val = db_string.append(String::from("k0"), String::from("v0"));

        assert!(len_val == 2);

        len_val = db_string.append(String::from("k0"), String::from("v1"));

        assert!(len_val == 4);
    }

    #[test]
    fn rename_test() {
        let mut db_string = DataBaseString::new();
        let mut res;
        let error = Err(RunError {
            message: "Error Command rename".to_string(),
            cause: "Key does not exist\n".to_string(),
        });

        db_string.set_string(String::from("key0"), String::from("val0"));

        res = db_string.rename(String::from("key0"), String::from("key1"));

        assert!(res == Ok(()));
        let res1 = db_string.get_string(String::from("key1"));
        assert_eq!(res1, String::from("val0"));

        res = db_string.rename(String::from("keyX"), String::from("keyXX"));

        assert!(res == error)
    }

    #[test]
    fn strlen_test() {
        let db_string = DataBaseString::new();
        let len = db_string.strlen(String::from("key0"));
        assert_eq!(len, 0);

        db_string.set_string(String::from("key0"), String::from("val0"));
        let len = db_string.strlen(String::from("key0"));
        assert_eq!(len, 4);

        db_string.set_string(String::from("key1"), String::from("val0 val1"));
        let len = db_string.strlen(String::from("key1"));
        assert_eq!(len, 9);

        db_string.set_string(String::from("key2"), String::from(""));
        let len = db_string.strlen(String::from("key2"));
        assert_eq!(len, 0);
    }

    #[test]
    fn mget_test() {
        let db_string = DataBaseString::new();
        let res = db_string.mget(vec!["key0"]);
        assert!(res.len() == 1);
        assert_eq!(res[0], String::from("(nil)"));

        let res2 = db_string.mget(vec!["key0", "key1"]);
        assert!(res2.len() == 2);
        assert_eq!(res2[0], String::from("(nil)"));
        assert_eq!(res2[1], String::from("(nil)"));

        db_string.set_string(String::from("key0"), String::from("val0"));
        db_string.set_string(String::from("key1"), String::from("val1"));

        let res3 = db_string.mget(vec!["key0", "key1", "key2"]);
        assert!(res3.len() == 3);
        assert_eq!(res3[0], String::from("val0"));
        assert_eq!(res3[1], String::from("val1"));
        assert_eq!(res3[2], String::from("(nil)"));
    }

    #[test]
    fn mset_test() {
        let db_string = DataBaseString::new();
        db_string.mset(vec!["key0", "val0"]);
        let res = db_string.get_string("key0".to_string());
        assert_eq!(res, String::from("val0"));

        db_string.mset(vec!["key1", "val1", "key2", "val2"]);
        let res1 = db_string.get_string("key1".to_string());
        let res2 = db_string.get_string("key2".to_string());
        assert_eq!(res1, String::from("val1"));
        assert_eq!(res2, String::from("val2"));
    }
}
