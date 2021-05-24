use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::SeekFrom;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;
use crate::config_server::ConfigServer;

pub trait Loggable {
    fn get_id_client(&self) -> i32;

    fn get_id_thread(&self) -> i32;

    fn get_timestamp(&self) -> SystemTime;
}

pub struct Logger<String> {
    file: File,
    sender: Arc<SyncSender<String>>,
    receiver: Arc<Mutex<Receiver<String>>>,
}

impl Clone for Logger<String> {
    fn clone(&self) -> Self {
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();
        let file = self.file.try_clone().unwrap();
        Self {
            file,
            sender,
            receiver,
        }
    }
}

impl<String> Drop for Logger<String> {
    fn drop(&mut self) {
        drop(self.sender.clone());
    }
}

fn generate_path_file(name_file: String, mut path_file: String) -> String {
    path_file.push_str(&"/".to_string());
    path_file.push_str(&name_file);
    path_file
}

impl Logger<String> {
    pub fn new(name_file: String, path_file: String) -> std::result::Result<Logger<String>, Error> {
        let path = generate_path_file(name_file, path_file);

        let file = File::create(path)?;
        let (sender, receiver) = sync_channel(1);
        let sender = Arc::new(sender);
        let receiver = Arc::new(Mutex::new(receiver));
        Ok(Self {
            file,
            sender,
            receiver,
        })
    }

    pub fn set_new_file_name(&mut self, name_file: String) {
        self.file = File::create(name_file).unwrap();
    }

    fn load_info(&mut self) -> Result<(), Error> {
        let msg = self.receiver.lock().unwrap().recv().unwrap();
        let file_size = self.file.metadata().unwrap().len();
        self.file.seek(SeekFrom::Start(file_size))?;
        self.file.write_all(msg.as_bytes())?;

        Ok(())
    }

    pub fn info(&self, service: &dyn Loggable, message_info: &str) -> Result<(), Error> {
        let msg = generate_menssage(service, message_info);
        let mut log = self.clone();

        thread::spawn(move || {
            log.sender.send(msg).unwrap();
            log.load_info().unwrap();
        })
            .join()
            .unwrap();

        Ok(())
    }
}

fn generate_menssage(service: &dyn Loggable, message_info: &str) -> String {
    let id_client = service.get_id_client();
    let id_thread = service.get_id_thread();
    let timestamp = service.get_timestamp();

    format!(
        "{:?} -- [{:?} -- {:?}] -- {}\n",
        id_client, id_thread, timestamp, message_info
    )
}

//--------------------------------------------PRUEBAS--------------------------------------------\\

struct Client(i32, i32);

impl Loggable for Client {
    fn get_id_client(&self) -> i32 {
        self.0.clone()
    }

    fn get_id_thread(&self) -> i32 {
        self.1.clone()
    }

    fn get_timestamp(&self) -> SystemTime {
        SystemTime::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_request() {
        let log = Logger::new(
            "prueba.txt".to_string(),
            "/home/gonzalosabatino/Escritorio".to_string(), //no sé qué otro path ponerle
        )
            .unwrap();

        log.info(&Client(1, 1), "hola").unwrap();
        log.info(&Client(2, 1), "hola").unwrap();
        log.info(&Client(3, 1), "hola").unwrap();
        log.info(&Client(4, 1), "hola").unwrap();
        log.info(&Client(5, 1), "hola").unwrap();
        log.info(&Client(6, 1), "hola").unwrap();
        log.info(&Client(7, 1), "hola").unwrap();
        log.info(&Client(8, 1), "hola").unwrap();
    }
}