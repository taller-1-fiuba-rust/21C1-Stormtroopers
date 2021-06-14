use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

pub struct Connection<String> {
    sender: Arc<Mutex<Sender<String>>>,
    receiver: Arc<Mutex<Receiver<String>>>,
}

impl<String> Connection<String> {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            sender: Arc::new(Mutex::new(tx)),
            receiver: Arc::new(Mutex::new(rx)),
        }
    }

    pub fn send(&self, response: String) {
        let sender = self.sender.lock().unwrap();
        sender.send(response).unwrap();
    }

    pub fn get_sender(&self) -> Arc<Mutex<Sender<String>>> {
        self.sender.clone()
    }

    pub fn get_receiver(&self) -> Arc<Mutex<Receiver<String>>> {
        self.receiver.clone()
    }
}

impl<String> Clone for Connection<String> {
    fn clone(&self) -> Self {
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();
        Self { sender, receiver }
    }
}
