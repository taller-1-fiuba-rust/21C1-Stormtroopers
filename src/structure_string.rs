use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;

use crate::errors::run_error::RunError;
use crate::structure_general::Storeable;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use std::any::Any;

const RESPONSE_NIL: &str = "(nil)";

pub struct StructureString<String> {
    structure: Arc<Mutex<HashMap<String, String>>>,
    sender: Arc<SyncSender<String>>,
    receiver: Arc<Mutex<Receiver<String>>>,
}

impl Default for StructureString<String> {
    fn default() -> Self {
        StructureString::new()
    }
}

impl Clone for StructureString<String> {
    fn clone(&self) -> Self {
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();
        let structure = self.structure.clone();
        Self {
            structure,
            sender,
            receiver,
        }
    }
}

impl<String> Drop for StructureString<String> {
    fn drop(&mut self) {
        drop(self.sender.clone());
    }
}

impl StructureString<String> {
    pub fn new() -> Self {
        let structure = Arc::new(Mutex::new(HashMap::new()));
        let (sender, receiver) = sync_channel(10000);
        let sender = Arc::new(sender);
        let receiver = Arc::new(Mutex::new(receiver));
        Self {
            structure,
            sender,
            receiver,
        }
    }

    pub fn set_string(&self, key: String, value: String) {
        let mut key_val_sender = key;
        key_val_sender.push(':');
        key_val_sender.push_str(&value);
        let mut structure = self.clone();
        let mut data = self.structure.clone();
        thread::spawn(move || {
            structure.sender.send(key_val_sender).unwrap();
            structure.save(&mut data);
        })
        .join()
        .unwrap();
    }

    pub fn get_string(&self, key: String) -> String {
        let mut structure = self.clone();

        let mut data = self.structure.clone();

        let return_res = thread::spawn(move || {
            structure.sender.send(key).unwrap();
            structure.get(&mut data)
        })
        .join()
        .unwrap();

        return_res
    }

    pub fn mset(&self, keys: Vec<&str>) {
        let mut structure = self.clone();
        let keys_sender = StructureString::vec_to_str(keys);

        thread::spawn(move || {
            structure.sender.send(keys_sender).unwrap();
            structure.mset_string();
        })
        .join()
        .unwrap();
    }

    pub fn mget(&self, keys: Vec<&str>) -> Vec<String> {
        let mut structure = self.clone();
        let mut keys_sender = String::from("");
        for key in keys.iter() {
            keys_sender.push_str(key);
            keys_sender.push(':');
        }
        keys_sender.pop();
        let to_return = thread::spawn(move || {
            structure.sender.send(keys_sender).unwrap();
            structure.mget_string()
        })
        .join()
        .unwrap();

        to_return
    }

    #[allow(dead_code)]
    pub fn clean_all_data(&self) -> bool {
        let mut structure_string = self.clone();
        let mut data = self.structure.clone();
        thread::spawn(move || {
            structure_string.sender.send(String::from("")).unwrap();
            structure_string.clean(&mut data);
        })
        .join()
        .unwrap();
        self.structure.lock().unwrap().is_empty()
    }

    #[allow(dead_code)]
    //TODO: ver esta impl
    pub fn dbsize(&self) -> usize {
        self.structure.lock().unwrap().len()
    }

    pub fn delete(&mut self, args: Vec<&str>) -> u32 {
        let mut count = 0_u32;
        let mut structure = self.structure.lock().unwrap();
        for key in args.iter() {
            if let Some(_v) = structure.remove(*key) {
                count += 1
            }
        }
        count
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
        let structure = self.structure.lock().unwrap();
        for key in keys.iter() {
            if structure.contains_key(*key) {
                count += 1
            }
        }
        count
    }

    /*
    fn save(&mut self, data: &mut Arc<Mutex<HashMap<String, String>>>) {
        let key_val_sender = self.receiver.lock().unwrap().recv().unwrap();
        let key_val_splited: Vec<&str> = key_val_sender.split(':').collect();

        let mut structure = data.lock().unwrap();
        structure.insert(
            String::from(key_val_splited[0].trim()),
            String::from(key_val_splited[1].trim()),
        );
    }

    fn get(&mut self, data: &mut Arc<Mutex<HashMap<String, String>>>) -> String {
        let key_val = self.receiver.lock().unwrap().recv().unwrap();

        let d = data.lock().unwrap();
        match d.get(&key_val) {
            Some(value) => value.clone(),
            None => String::from(RESPONSE_NIL),
        }
    }

    fn clean(&mut self, data: &mut Arc<Mutex<HashMap<String, String>>>) -> bool {
        self.receiver.lock().unwrap().recv().unwrap();
        let mut structure = data.lock().unwrap();
        structure.clear();
        structure.is_empty()
    }
     */

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

        let structure = self.structure.lock().unwrap();
        let mut to_ret = Vec::new();
        let mut resp_get;
        for key in keys_splited.iter() {
            resp_get = match structure.get(*key) {
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
        let mut structure = self.structure.lock().unwrap();
        for idx in 0..keys_splited.len() / 2 {
            println!("{} {}", keys_splited[idx], keys_splited[(idx * 2) + 1]);
            structure.insert(
                keys_splited[idx * 2].trim().to_string(),
                keys_splited[(idx * 2) + 1].trim().to_string(),
            );
        }
    }

    fn save(&mut self, data: &mut Arc<Mutex<HashMap<String, String>>>) {
        let key_val_sender = self.receiver.lock().unwrap().recv().unwrap();
        let key_val_splited: Vec<&str> = key_val_sender.split(':').collect();

        let mut structure = data.lock().unwrap();
        structure.insert(
            String::from(key_val_splited[0].trim()),
            String::from(key_val_splited[1].trim()),
        );
    }

    fn get(&mut self, structure: &mut Arc<Mutex<HashMap<String, String>>>) -> String {
        let key_val = self.receiver.lock().unwrap().recv().unwrap();

        let data = structure.lock().unwrap();
        match data.get(&key_val) {
            Some(value) => value.clone(),
            None => String::from(RESPONSE_NIL),
        }
    }

    #[allow(dead_code)]
    fn clean(&mut self, data: &mut Arc<Mutex<HashMap<String, String>>>) -> bool {
        self.receiver.lock().unwrap().recv().unwrap();
        let mut structure = data.lock().unwrap();
        structure.clear();
        structure.is_empty()
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

impl Storeable for StructureString<String> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structure_string_test() {
        let structure = StructureString::new();
        structure.set_string(String::from("test"), String::from("1"));
        let res = structure.get_string(String::from("test"));

        assert_eq!(res, String::from("1"));
    }

    #[test]
    fn get_test() {
        let structure = StructureString::new();
        let resp = structure.get_string(String::from("key0"));
        assert_eq!(resp, String::from(RESPONSE_NIL));

        structure.set_string(String::from("key0"), String::from("val0"));
        let resp2 = structure.get_string(String::from("key0"));
        assert_eq!(resp2, String::from("val0"));
    }

    #[test]
    fn clean_all_test() {
        let structure_string = StructureString::new();
        structure_string.set_string(String::from("test"), String::from("1"));
        {
            assert!(!structure_string.structure.lock().unwrap().is_empty());
        }

        structure_string.clean_all_data();
        {
            assert!(structure_string.structure.lock().unwrap().is_empty());
        }
    }

    #[test]
    fn dbsize_test() {
        let structure_string = StructureString::new();
        structure_string.set_string(String::from("test"), String::from("1"));
        {
            assert!(structure_string.dbsize() == 1);
        }
    }

    #[test]
    fn copy_test() {
        let mut structure_string = StructureString::new();
        let value0 = String::from("value0");
        structure_string.set_string(String::from("key0"), value0.clone());
        let res = structure_string.copy(String::from("key0"), String::from("key0.bis"));

        assert!(res == 1);
        let value0bis = structure_string.get_string(String::from("key0.bis"));
        assert_eq!(value0, value0bis.clone());
    }

    #[test]
    fn exists_test() {
        let structure_string = StructureString::new();

        let res = structure_string.exists(vec!["key"]);

        assert!(res == 0);

        let structure_string2 = StructureString::new();
        structure_string2.set_string(String::from("one_key"), String::from("1"));
        let res = structure_string2.exists(vec!["one_key"]);

        assert!(res == 1);

        let structure_string2 = StructureString::new();
        structure_string2.set_string(String::from("one_key"), String::from("1"));
        structure_string2.set_string(String::from("two_key"), String::from("2"));
        let res = structure_string2.exists(vec!["one_key", "two_key", "other_key"]);

        assert!(res == 2);
    }

    #[test]
    fn delete_test() {
        let mut structure_string = StructureString::new();

        let mut count;
        count = structure_string.delete(vec!["key0"]);
        assert!(count == 0);

        structure_string.set_string(String::from("key0"), String::from("val0"));
        count = structure_string.delete(vec!["key0"]);

        assert!(count == 1);

        structure_string.set_string(String::from("key0"), String::from("val0"));
        structure_string.set_string(String::from("key1"), String::from("val1"));
        count = structure_string.delete(vec!["key0", "key1", "key2"]);

        assert!(count == 2);
    }

    #[test]
    fn append_test() {
        let structure_string = StructureString::new();
        let mut len_val;
        len_val = structure_string.append(String::from("k0"), String::from("v0"));

        assert!(len_val == 2);

        len_val = structure_string.append(String::from("k0"), String::from("v1"));

        assert!(len_val == 4);
    }

    #[test]
    fn rename_test() {
        let mut structure_string = StructureString::new();
        let mut res;
        let error = Err(RunError {
            message: "Error Command rename".to_string(),
            cause: "Key does not exist\n".to_string(),
        });

        structure_string.set_string(String::from("key0"), String::from("val0"));

        res = structure_string.rename(String::from("key0"), String::from("key1"));

        assert!(res == Ok(()));
        let res1 = structure_string.get_string(String::from("key1"));
        assert_eq!(res1, String::from("val0"));

        res = structure_string.rename(String::from("keyX"), String::from("keyXX"));

        assert!(res == error)
    }

    #[test]
    fn strlen_test() {
        let structure_string = StructureString::new();
        let len = structure_string.strlen(String::from("key0"));
        assert_eq!(len, 0);

        structure_string.set_string(String::from("key0"), String::from("val0"));
        let len = structure_string.strlen(String::from("key0"));
        assert_eq!(len, 4);

        structure_string.set_string(String::from("key1"), String::from("val0 val1"));
        let len = structure_string.strlen(String::from("key1"));
        assert_eq!(len, 9);

        structure_string.set_string(String::from("key2"), String::from(""));
        let len = structure_string.strlen(String::from("key2"));
        assert_eq!(len, 0);
    }

    #[test]
    fn mget_test() {
        let structure_string = StructureString::new();
        let res = structure_string.mget(vec!["key0"]);
        assert!(res.len() == 1);
        assert_eq!(res[0], String::from("(nil)"));

        let res2 = structure_string.mget(vec!["key0", "key1"]);
        assert!(res2.len() == 2);
        assert_eq!(res2[0], String::from("(nil)"));
        assert_eq!(res2[1], String::from("(nil)"));

        structure_string.set_string(String::from("key0"), String::from("val0"));
        structure_string.set_string(String::from("key1"), String::from("val1"));

        let res3 = structure_string.mget(vec!["key0", "key1", "key2"]);
        assert!(res3.len() == 3);
        assert_eq!(res3[0], String::from("val0"));
        assert_eq!(res3[1], String::from("val1"));
        assert_eq!(res3[2], String::from("(nil)"));
    }

    #[test]
    fn mset_test() {
        let structure_string = StructureString::new();
        structure_string.mset(vec!["key0", "val0"]);
        let res = structure_string.get_string("key0".to_string());
        assert_eq!(res, String::from("val0"));

        structure_string.mset(vec!["key1", "val1", "key2", "val2"]);
        let res1 = structure_string.get_string("key1".to_string());
        let res2 = structure_string.get_string("key2".to_string());
        assert_eq!(res1, String::from("val1"));
        assert_eq!(res2, String::from("val2"));
    }
}
