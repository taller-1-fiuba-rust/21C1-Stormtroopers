use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

pub struct Connection<String> {
    sender: Arc<Mutex<Sender<String>>>,
    receiver: Arc<Mutex<Receiver<String>>>,
    timeout: Duration,
    system_time: SystemTime,
    monitor: bool,
}

impl<String> Drop for Connection<String> {
    fn drop(&mut self) {
        let sender = self.sender.lock().unwrap();
        drop(sender);
    }
}

impl<String> Connection<String> {
    pub fn new(timeout: u64) -> Self {
        let (tx, rx) = channel();
        Self {
            sender: Arc::new(Mutex::new(tx)),
            receiver: Arc::new(Mutex::new(rx)),
            timeout: Duration::new(timeout, 0),
            system_time: SystemTime::now(),
            monitor: false,
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

    pub fn activate_monitor(&mut self) {
        self.monitor = true;
    }

    pub fn deactivate_monitor(&mut self) {
        self.monitor = false;
    }

    pub fn monitor(&self) -> bool {
        self.monitor
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
            monitor: self.monitor,
        }
    }
}
