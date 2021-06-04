use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;

use crate::errors::run_error::RunError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct StructureString<String> {
    pub structure: Arc<Mutex<HashMap<String, String>>>,
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
    pub fn new() -> StructureString<String> {
        let structure = Arc::new(Mutex::new(HashMap::new()));
        let (sender, receiver) = sync_channel(1);
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
            structure.load(&mut data)
        })
        .join()
        .unwrap();

        return_res
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
    pub fn dbsize(&self) -> u32 {
        self.structure.lock().unwrap().len() as u32
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
        if src_val == *"EMPTY_STRING" {
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

    fn save(&mut self, data: &mut Arc<Mutex<HashMap<String, String>>>) {
        let key_val_sender = self.receiver.lock().unwrap().recv().unwrap();
        let key_val_splited: Vec<&str> = key_val_sender.split(':').collect();

        let mut structure = data.lock().unwrap();
        structure.insert(
            String::from(key_val_splited[0].trim()),
            String::from(key_val_splited[1].trim()),
        );
    }

    fn load(&mut self, data: &mut Arc<Mutex<HashMap<String, String>>>) -> String {
        let key_val = self.receiver.lock().unwrap().recv().unwrap();

        let d = data.lock().unwrap();
        match d.get(&key_val) {
            Some(value) => value.clone(),
            None => String::from("EMPTY_STRING"),
        }
    }
    #[allow(dead_code)]
    fn clean(&mut self, data: &mut Arc<Mutex<HashMap<String, String>>>) -> bool {
        self.receiver.lock().unwrap().recv().unwrap();
        let mut structure = data.lock().unwrap();
        structure.clear();
        structure.is_empty()
    }

    #[allow(dead_code)]
    pub fn append(&self, key: String, value_append: String) -> u32 {
        let mut value = self.get_string(key.clone());
        if value == *"EMPTY_STRING" {
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
        if value == *"EMPTY_STRING" {
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
    fn del_test() {
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
}
