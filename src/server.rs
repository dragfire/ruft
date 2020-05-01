use std::thread;
use std::sync::{Arc, Mutex};
use zmq::Socket;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::node::Node;
use crate::util;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    content: HashMap<String, String>,
}

type Handle = fn(&Message);

pub struct Server {
    pub node: Node,
    pub responder: Arc<Mutex<Socket>>,
    pub clients: HashMap<String, Socket>, // cache client connections
    pub handlers: HashMap<String, Handle>,
}

impl Server {
    pub fn new(address: String, other_node_adds: Vec<String>) -> Result<Server, zmq::Error> {
        let handlers = HashMap::new();
        let clients: HashMap<String, Socket> = HashMap::new();

        let node = Node::new(address.to_owned(), other_node_adds);
        let context = zmq::Context::new();
        let responder = Arc::new(Mutex::new(context.socket(zmq::REP).unwrap()));

        responder.lock()
            .unwrap()
            .bind(&util::get_tcp_address(&address))
            .or_else(|e: zmq::Error| -> Result<(), zmq::Error> {
                // just want to see the error
                println!("{:?}", e);
                Err(e)
            }).unwrap();


        Ok(Server { node, responder, clients, handlers })
    }

    pub fn start(&mut self) -> thread::JoinHandle<()> {
        self.register_handlers();
        println!("Starting server: {}", self.node.address);
        let address = util::get_tcp_address(&self.node.address);
        let responder = Arc::clone(&self.responder);

        thread::spawn(move || {
            let mut msg = zmq::Message::new();

            loop {
                responder.lock().unwrap().recv(&mut msg, 0).unwrap();
                println!("Server({}) received: {}", address, msg.as_str().unwrap());
                responder.lock().unwrap().send("OK", 0).unwrap();
                // TBD: pass msg to the right handler
            }
        })
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
}

#[cfg(test)]
mod rpc_tests {
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
