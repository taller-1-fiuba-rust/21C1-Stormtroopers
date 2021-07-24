use crate::constants::LINE_BREAK;
use regex::Regex;
use std::collections::{BTreeSet, HashMap};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;

const PUBLISH_CONSTANT: &str = "Reading pubsub messages...";

pub struct Client {
    sender: Arc<Mutex<Sender<String>>>, //puedo agregarle después a cuántos canales se suscribió
    receiver: Arc<Mutex<Receiver<String>>>,
}

impl Client {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            sender: Arc::new(Mutex::new(tx)),
            receiver: Arc::new(Mutex::new(rx)),
        }
    }

    pub fn new_with_recv(
        sender: Arc<Mutex<Sender<String>>>,
        receiver: Arc<Mutex<Receiver<String>>>,
    ) -> Self {
        Self { sender, receiver }
    }

    pub fn get_recv(&self) -> Arc<Mutex<Receiver<String>>> {
        self.receiver.clone()
    }

    pub fn publish(&self, msg: String, name_channel: String) {
        let sender = self.sender.lock().unwrap();

        let response = format!(
            "\n{}\nFrom Channel: {}\n{}\n",
            PUBLISH_CONSTANT, name_channel, msg
        );
        sender.send(response).unwrap();
    }

    pub fn private_publish(&self, mut msg: String) {
        let sender = self.sender.lock().unwrap();

        msg.push(LINE_BREAK);
        sender.send(msg).unwrap();
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();
        Self { sender, receiver }
    }
}

pub struct Pubsub {
    suscribers: Arc<Mutex<HashMap<usize, Client>>>, //cada cliente tiene su id
    channels: Arc<Mutex<HashMap<String, BTreeSet<usize>>>>,
}

impl Default for Pubsub {
    fn default() -> Self {
        Self::new()
    }
}

impl Pubsub {
    pub fn new() -> Self {
        let suscribers = Arc::new(Mutex::new(HashMap::new()));
        let channels = Arc::new(Mutex::new(HashMap::new()));
        Self {
            suscribers,
            channels,
        }
    }

    pub fn add_client_with_recv(
        &mut self,
        id_client: usize,
        sender: Arc<Mutex<Sender<String>>>,
        receiver: Arc<Mutex<Receiver<String>>>,
    ) -> Arc<Mutex<Receiver<String>>> {
        let client = Client::new_with_recv(sender, receiver);

        let mut suscribers_lock = self.suscribers.lock().unwrap();
        suscribers_lock.insert(id_client, client.clone());
        client.get_recv()
    }

    pub fn add_client(&mut self, id_client: usize) -> Arc<Mutex<Receiver<String>>> {
        let client = Client::new();

        let mut suscribers_lock = self.suscribers.lock().unwrap();
        suscribers_lock.insert(id_client, client.clone());
        client.get_recv()
    }

    pub fn suscribe(&mut self, channel: String, client: usize) {
        let mut channels_lock = self.channels.lock().unwrap();

        let subbed_clients = channels_lock.entry(channel).or_insert_with(BTreeSet::new);
        subbed_clients.insert(client);
    }

    pub fn create_channel(&mut self, channel: String) {
        let mut channels_lock = self.channels.lock().unwrap();
        channels_lock.insert(channel, BTreeSet::new());
    }

    pub fn len_channels(&self) -> usize {
        let channels = self.channels.lock().unwrap();
        channels.len()
    }

    pub fn len_channel(&self, channel: String) -> usize {
        let channels = self.channels.lock().unwrap();
        channels.get(&channel).unwrap().len()
    }

    pub fn get_suscribers(&self, channel: String) -> Vec<usize> {
        let mut suscribers_vec = vec![];
        let channels = self.channels.lock().unwrap();
        let suscribers = channels.get(&channel).unwrap();

        for suscriber in suscribers.iter() {
            suscribers_vec.push(*suscriber);
        }

        suscribers_vec
    }

    pub fn publish(&self, name_channel: String, msg: String, private: bool) -> Option<()> {
        let channels = self.channels.lock().unwrap();
        let suscribers = self.suscribers.lock().unwrap();

        if let Some(channel) = channels.get(&name_channel) {
            for suscriber in channel.iter() {
                let client = suscribers.get(&suscriber).unwrap();
                if !private {
                    client.publish(msg.clone(), name_channel.clone());
                } else {
                    client.private_publish(msg.clone());
                }
            }
        } else {
            return None;
        }

        Some(())
    }

    pub fn unsuscribe(&self, name_channel: String, client: usize) {
        let mut channels = self.channels.lock().unwrap();
        //let channel: &BTreeSet<usize> = channels.get(&name_channel).unwrap();

        if let Some(subbed_clients) = channels.get_mut(&name_channel) {
            subbed_clients.remove(&client);
        }
    }

    pub fn available_channels(&self) -> Vec<String> {
        let mut channels_vec = Vec::<String>::new();
        let channels = self.channels.lock().unwrap();

        for key in channels.keys() {
            channels_vec.push((*(key.clone())).to_string());
        }

        channels_vec
    }

    pub fn available_channels_pattern(&self, pattern: &str) -> Vec<String> {
        let mut channels_vec = Vec::<String>::new();
        let channels = self.channels.lock().unwrap();
        let re = Regex::new(pattern).unwrap();

        for key in channels.keys() {
            if re.is_match(&key) {
                channels_vec.push((*(key.clone())).to_string());
            }
        }

        channels_vec
    }

    pub fn numsub(&self) -> Vec<(String, usize)> {
        let mut channels_vec = Vec::<(String, usize)>::new();
        let channels = self.channels.lock().unwrap();

        for (key, val) in channels.iter() {
            let name_channel = (*(key.clone())).to_string();
            let size_channel = val.len();
            channels_vec.push((name_channel, size_channel));
        }

        channels_vec
    }
}

impl Clone for Pubsub {
    fn clone(&self) -> Self {
        Self {
            suscribers: self.suscribers.clone(),
            channels: self.channels.clone(),
        }
    }
}
