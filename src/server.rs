use std::thread;
use zmq::Socket;
use std::time;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::node::Node;
use crate::util;
use crate::message::Message;

type Handle = fn(&Message);

/// Leader Election:
///     Default State: Follower
///     After election timeout, becomes candidate. Request vote to all nodes.
///         Increase term count, votes for itself.
///     Elected as leader if received votes from majority of nodes.
///
///     TBD: take care of split vote (two candidates getting same number of votes)
///
pub struct Server {
    pub address: String,
    pub node: Arc<Mutex<Node>>,
    pub responder: Arc<Mutex<Socket>>,
    pub clients: HashMap<String, Socket>, // cache client connections
    pub handlers: HashMap<String, Handle>,
}

impl Server {
    pub fn new(address: String, other_node_adds: Vec<String>) -> Result<Server, zmq::Error> {
        let handlers = HashMap::new();
        let clients: HashMap<String, Socket> = HashMap::new();
        let tcp_addr = util::get_tcp_address(&address);

        let node = Arc::new(Mutex::new(Node::new(tcp_addr.to_owned(), other_node_adds)));
        let context = zmq::Context::new();
        let responder = Arc::new(Mutex::new(context.socket(zmq::REP).unwrap()));

        responder.lock()
            .unwrap()
            .bind(&tcp_addr)
            .or_else(|e: zmq::Error| -> Result<(), zmq::Error> {
                // just want to see the error
                println!("{:?}", e);
                Err(e)
            }).unwrap();

        Ok(Server { address, node, responder, clients, handlers })
    }

    pub fn start(&mut self) -> thread::JoinHandle<()> {
        self.register_handlers();
        println!("Starting server: {}", self.address);
        let responder = Arc::clone(&self.responder);

        thread::spawn(move || {
            loop {
                let mut msg = zmq::Message::new();
                responder.lock().unwrap().recv(&mut msg, 0).unwrap();
                let message: Message = serde_json::from_str(msg.as_str().unwrap()).unwrap();
                Server::process_msg(message);
                responder.lock().unwrap().send("OK", 0).unwrap();
            }
        })
    }

    /// if not candidate or leader, check if 
    pub fn start_leader_election(&mut self) -> thread::JoinHandle<()> {
        let election_timeout = util::random_timeout();
        thread::spawn(move || loop {
            thread::sleep(time::Duration::from_millis(election_timeout));
            println!("leader election");
        })
    }

    pub fn start_all(&mut self) -> Vec<thread::JoinHandle<()>> {
        let node_handle = self.start();
        let leader_handle = self.start_leader_election();

        vec![node_handle, leader_handle]
    }

    pub fn get_client(&mut self, address: &str) -> &Socket {
        self.clients.entry(address.to_string()).or_insert_with(|| {
            let context = zmq::Context::new();
            let client = context.socket(zmq::REQ).unwrap();
            Server::connect_socket(&client, address);
            client
        })
    }

    fn register_handlers(&mut self) {
        self.handlers.insert("/hello".to_string(), Server::check);
    }

    fn connect_socket(client: &Socket, address: &str) {
        let tcp_addr = String::from("tcp://") + &address;
        client.connect(&tcp_addr).unwrap();
    }

    fn check(msg: &Message) {
        println!("{:?}", msg);
    }

    fn leader_heartbeat(msg: Message) {
        unimplemented!();
    }

    fn election_request(msg: Message) {
        unimplemented!();
    }

    fn election_request_received(msg: Message) {
        unimplemented!();
    }

    fn respond_to_leader_heartbeat(&mut self) {
        unimplemented!();
    }

    fn process_msg(msg: Message) {
        println!("Processing Message: {:?}", msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message() {
        let mut msg = Message { content: HashMap::new() };
        msg.content.insert("key".to_string(), "value".to_string());
        msg.content.insert("key1".to_string(), "value".to_string());
        msg.content.insert("key2".to_string(), "value".to_string());
        msg.content.insert("key3".to_string(), "value".to_string());

        let astr = serde_json::to_string(&msg).unwrap();
        println!("{}", astr);
    }
}
