use std::thread;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};

use std::collections::HashMap;
use std::io::Error;


//#[derive(Clone, Copy)]
pub struct StructureString<String> {
    structure: Arc<Mutex<HashMap<String,String>>>,
    sender: Arc<SyncSender<String>>,
    receiver: Arc<Mutex<Receiver<String>>>,
}

impl Clone for StructureString<String> {
    fn clone(&self) -> Self {
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();
        let mut structure = self.structure.clone();
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
        let mut structure = Arc::new(Mutex::new(HashMap::new()));
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
        let mut data = self.structure.clone();
        thread::spawn(move || {
            structure.sender.send(key_val_sender).unwrap();
            structure.save(&mut data);
        })
        .join()
        .unwrap();
        do_it(&mut self.structure.lock().unwrap());
    }

    pub fn get_string(&self, key: String) -> String {
        let mut structure = self.clone();
        do_it(&mut self.structure.lock().unwrap());
        let mut data = self.structure.clone();
        println!("structure_string::init::get_string: {}", key);
        let mut res2 = thread::spawn(move || {
            structure.sender.send(key).unwrap();
            let res = structure.load(&mut data);
            println!("structure_string::thread::get_string: {}", res);
            return String::from(res)
        })
        .join()
        .unwrap();
        do_it(&mut self.structure.lock().unwrap());
        println!("structure_string::end::get_string: {}", res2);
        String::from(res2)
    }

    fn save(&mut self, data: &mut Arc<Mutex<HashMap<String, String>>>) {
        let key_val_sender = self.receiver.lock().unwrap().recv().unwrap();
        let key_val_splited: Vec<&str> = key_val_sender.split(":").collect();
        println!("save: {} => {}",key_val_splited[0],key_val_splited[1]);

        //let data = Arc::get_mut(&mut self.structure).unwrap();
        //data.insert(String::from(key_val_splited[0].trim()),String::from(key_val_splited[1].trim()));
        let mut d = data.lock().unwrap();
        d.insert(String::from(key_val_splited[0].trim()),String::from(key_val_splited[1].trim()));
        do_it( &mut d);

        //data.insert(String::from(key_val_splited[0].trim()),String::from(key_val_splited[1].trim()));
    }

    //TODO: ver esto que onda
    fn load(&mut self, data: &mut Arc<Mutex<HashMap<String, String>>>) -> String {
        let key_val = self.receiver.lock().unwrap().recv().unwrap();
        println!("structure::load::receiver: {}",key_val);

        //let data = Arc::get_mut(&mut self.structure).unwrap();
        //let mut data = self.structure.clone();
        let mut d = data.lock().unwrap();
        do_it(&mut d );
        match d.get(&key_val) {
            Some(value) => String::from(value.clone()),
            None        => String::from("EMPTY_STRING"),
        }
        /*
                match  self.structure.get(&key_val) {
                    Some(value) => String::from(value.clone()),
                    None        => String::from("EMPTY_STRING"),
                }

                match  self.structure.lock().unwrap().get(&key_val) {
                    Some(value) => String::from(value.clone()),
                    None        => String::from("EMPTY_STRING"),
                }
                 */
    }
}

fn do_it(map: &mut HashMap<String, String>) {
    println!("look map");
    for (key, value) in &*map {
        println!("{} / {}", key, value);
    }
    //map.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structure_string_test(){
        let structure = StructureString::new();
        structure.set_string(String::from("test"),String::from("1"));
        let res = structure.get_string(String::from("test"));

        assert_eq!(res, String::from("1"));
    }
}