use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

pub struct Connection<String> {
    sender: Arc<Mutex<Sender<String>>>,
    receiver: Arc<Mutex<Receiver<String>>>,
    timeout: Duration,
    system_time: SystemTime,
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
    pub fn new(timeout: u64) -> Self {
        let (tx, rx) = channel();
        Self {
            sender: Arc::new(Mutex::new(tx)),
            receiver: Arc::new(Mutex::new(rx)),
            timeout: Duration::new(timeout, 0),
            system_time: SystemTime::now(),
        }
    }

    pub fn renew_connection(&mut self) {
        self.system_time = SystemTime::now();
    }

    pub fn over(&self) -> bool {
        if self.timeout == Duration::new(0, 0) {
            return false;
        }
        let sys_now = SystemTime::now();
        sys_now.duration_since(self.system_time).unwrap() >= self.timeout
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
        let timeout = self.timeout;
        let system_time = self.system_time;
        Self {
            sender,
            receiver,
            timeout,
            system_time,
        }
    }
}
