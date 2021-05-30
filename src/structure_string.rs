use std::thread;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};

use std::collections::HashMap;
use std::io::Error;

pub struct StructureString<String> {
    structure: HashMap<String,String>,
    sender: Arc<SyncSender<String>>,
    receiver: Arc<Mutex<Receiver<String>>>,
}

impl Clone for StructureString<String> {
    fn clone(&self) -> Self {
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();
        let mut structure = HashMap::new();
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
        let structure = HashMap::new();
        let (sender, receiver) = sync_channel(1);
        let sender = Arc::new(sender);
        let receiver = Arc::new(Mutex::new(receiver));
        Self {
            structure,
            sender,
            receiver,
        }
    }

    pub fn set_string(&self,key: String, value: String) {
        let mut key_val_sender = key;
        key_val_sender.push_str(":");
        key_val_sender.push_str(&value);
        let mut structure = self.clone();
        thread::spawn(move || {
            structure.sender.send(key_val_sender).unwrap();
            structure.save();
        })
        .join()
        .unwrap();

    }

    fn save(&mut self) {
        let mut key_val_sender = self.receiver.lock().unwrap().recv().unwrap();
        let key_val_splited: Vec<&str> = key_val_sender.split(":").collect();
        self.structure.insert(String::from(key_val_splited[0]),String::from(key_val_splited[1]));
    }

    pub fn get_string(&self, key: String) -> String {
        let mut structure = self.clone();
        //let mut res: String = String::from("EMPTY");
        println!("structure_string::init::get_string: {}", key);
        let mut res2 = thread::spawn(move || {
            structure.sender.send(key).unwrap();
            let res = structure.load();
            println!("structure_string::thread::get_string: {}", res);
            return String::from(res)
        })
            .join()
            .unwrap();

        println!("structure_string::end::get_string: {}", res2);
        String::from(res2)
    }

    fn load(&self) -> String {
        let mut key_val = self.receiver.lock().unwrap().recv().unwrap();
        println!("structure::load::receiver: {}",key_val);
        let mut res = (*self.structure.get_mut(&key_val).unwrap().clone());
        //String::from(res)
        res.to_string()
    }
}