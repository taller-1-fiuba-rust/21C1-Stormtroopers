use std::collections::{BTreeSet, HashMap};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;

pub struct Client {
    sender: Arc<Mutex<Sender<String>>>, //puedo agregarle después a cuántos canales se suscribió
    receiver: Arc<Mutex<Receiver<String>>>,
}

impl Client {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn publish(&self, msg: String) {
        let sender = self.sender.lock().unwrap();
        sender.send(msg).unwrap();
        sender.send("\n".to_string()).unwrap();
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
    suscribers: Arc<Mutex<HashMap<i32, Client>>>, //cada cliente tiene su id
    channels: Arc<Mutex<HashMap<String, BTreeSet<i32>>>>,
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
        id_client: i32,
        sender: Arc<Mutex<Sender<String>>>,
        receiver: Arc<Mutex<Receiver<String>>>,
    ) -> Arc<Mutex<Receiver<String>>> {
        let client = Client::new_with_recv(sender, receiver);

        let mut suscribers_lock = self.suscribers.lock().unwrap();
        suscribers_lock.insert(id_client, client.clone());
        client.get_recv()
    }

    #[allow(dead_code)]
    pub fn add_client(&mut self, id_client: i32) -> Arc<Mutex<Receiver<String>>> {
        let client = Client::new();

        let mut suscribers_lock = self.suscribers.lock().unwrap();
        suscribers_lock.insert(id_client, client.clone());
        client.get_recv()
    }

    #[allow(dead_code)]
    fn suscribe_channel(&mut self, channel: String, client: i32) {
        let mut channels_lock = self.channels.lock().unwrap();

        let subbed_clients = channels_lock
            .entry(channel.clone())
            .or_insert(BTreeSet::new());
        subbed_clients.insert(client);
    }

    #[allow(dead_code)]
    pub fn suscribe(&mut self, channel: String, client: i32) {
        //unificar el anterior y este (son lo mismo)
        self.suscribe_channel(channel, client);
    }

    #[allow(dead_code)]
    pub fn len_channels(&self) -> usize {
        let channels = self.channels.lock().unwrap();
        channels.len()
    }

    #[allow(dead_code)]
    pub fn len_channel(&self, channel: String) -> usize {
        let channels = self.channels.lock().unwrap();
        channels.get(&channel).unwrap().len()
    }

    #[allow(dead_code)]
    pub fn get_suscribers(&self, channel: String) -> Vec<i32> {
        let mut suscribers_vec = vec![];
        let channels = self.channels.lock().unwrap();
        let suscribers = channels.get(&channel).unwrap();

        for suscriber in suscribers.iter() {
            suscribers_vec.push(*suscriber);
        }

        suscribers_vec
    }

    #[allow(dead_code)]
    pub fn publish(&self, name_channel: String, msg: String) {
        let channels = self.channels.lock().unwrap();
        let suscribers = self.suscribers.lock().unwrap();

        let channel = channels.get(&name_channel).unwrap();

        for suscriber in channel.iter() {
            let client = suscribers.get(&suscriber).unwrap();
            client.publish(msg.clone());
        }
    }

    #[allow(dead_code)]
    pub fn unsuscribe(&self, name_channel: String, client: i32) {
        let mut channels = self.channels.lock().unwrap();
        //let channel: &BTreeSet<i32> = channels.get(&name_channel).unwrap();

        if let Some(subbed_clients) = channels.get_mut(&name_channel) {
            subbed_clients.remove(&client);
        }
    }

    #[allow(dead_code)]
    pub fn available_channels(&self) -> Vec<String> {
        let mut channels_vec = Vec::<String>::new();
        let channels = self.channels.lock().unwrap();

        for key in channels.keys() {
            channels_vec.push((*(key.clone())).to_string());
        }

        channels_vec
    }

    #[allow(dead_code)]
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
