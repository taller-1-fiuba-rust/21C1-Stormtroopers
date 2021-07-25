use crate::server::pubsub::Pubsub;
use crate::Connection;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

///It manages the state of the connections with a structure that identifies the client_id and a Connection.
///
///It allows managing connections by adding new ones to the structure and recovering each connection.
///
///Allows you to disconnect a connection.
///
///Connects according to client_id a connection with the pub / sub endpoints.
///
///Supports activating a monitor and knowing if the connection has an associated monitor.
///
///Lets know the size of the structure.
pub struct ConnectionResolver {
    connections: Arc<Mutex<HashMap<usize, Connection<String>>>>,
}

impl Default for ConnectionResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectionResolver {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn connect_client(&self, id_client: usize, timeout: u64) {
        let connection_client = Connection::<String>::new(timeout);

        let mut connections = self.connections.lock().unwrap();
        connections.insert(id_client, connection_client);
    }

    pub fn disconnect_client(&self, id_client: usize) {
        let mut connections;
        loop {
            if let Ok(val) = self.connections.lock() {
                connections = val;
                break;
            }
        }
        //let mut connections = self.connections.lock().unwrap();
        connections.remove(&id_client).expect("Remove failed");
    }

    pub fn get_connection_client(&self, id_client: usize) -> Connection<String> {
        let connections = self.connections.lock().unwrap();
        connections.get(&id_client).unwrap().clone()
    }

    pub fn join_pubsub_receiver(
        &self,
        id_client: usize,
        mut pubsub: Pubsub,
    ) -> Arc<Mutex<Receiver<String>>> {
        let connection_client = self.get_connection_client(id_client);

        pubsub.add_client_with_recv(
            id_client,
            connection_client.get_sender(),
            connection_client.get_receiver(),
        )
    }

    pub fn connect_client_with_pubsub(
        &self,
        id_client: usize,
        timeout: u64,
        pubsub: Pubsub,
    ) -> Arc<Mutex<Receiver<String>>> {
        self.connect_client(id_client, timeout);
        self.join_pubsub_receiver(id_client, pubsub)
    }

    pub fn activate_monitor(&self, id_client: usize) {
        let mut connection = self.get_connection_client(id_client);
        connection.activate_monitor();
        let mut connections = self.connections.lock().unwrap();
        connections.insert(id_client, connection);
    }

    pub fn monitor(&self, id_client: usize) -> bool {
        let connection = self.get_connection_client(id_client);
        connection.monitor()
    }

    pub fn size(&self) -> usize {
        self.connections.lock().unwrap().clone().len()
    }
}

impl Clone for ConnectionResolver {
    fn clone(&self) -> Self {
        Self {
            connections: self.connections.clone(),
        }
    }
}
