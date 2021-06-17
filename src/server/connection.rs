use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

pub struct Connection<String> {
    sender: Arc<Mutex<Sender<String>>>,
    receiver: Arc<Mutex<Receiver<String>>>,
}

impl<String> Drop for Connection<String> {
    fn drop(&mut self) {
        let sender = self.sender.lock().unwrap();
        drop(sender);
    }
}

/*
impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last = inner.senders == 0;
        drop(inner);
        if was_last {
            self.shared.available.notify_one();
        }
    }
}
*/

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
