use std::thread;
use zmq::Socket;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    content: HashMap<String, String>,
}

type Handle = fn(&Message);

pub struct NodeRpc {
    pub address: String,
    pub client: Socket,
    pub handlers: HashMap<String, Handle>,
}

fn check(msg: &Message) {
    println!("{:?}", msg);
}

fn register_handlers(rpc: &mut NodeRpc) {
    rpc.handlers.insert("/hello".to_string(), check);
}


impl NodeRpc {
    pub fn new(address: String) -> Result<NodeRpc, zmq::Error> {
        let context = zmq::Context::new();
        let client = context.socket(zmq::REQ)?;
        client.connect(&address)?;
        let handlers = HashMap::new();

        Ok(NodeRpc { address, client, handlers })
    }

    pub fn start(&mut self) {
        register_handlers(self);

        thread::spawn(|| {
            let context = zmq::Context::new();
            let server = context.socket(zmq::REP).unwrap();
            assert!(server.bind("tcp://*:5555").is_ok());

            let mut msg = zmq::Message::new();

            loop {
                server.recv(&mut msg, 0).unwrap();
                println!("Received {}", msg.as_str().unwrap());
                server.send("OK", 0).unwrap();
            }
        });
    }
}

// ======================== Tests ========================

#[cfg(test)]
mod rpc_tests {
    use super::*;

    #[test]
    fn test_noderpc() {
        let mut rpc = NodeRpc::new(String::from("tcp://localhost:5555"));

        match rpc {
            Ok(ref mut rpc) => {
                rpc.start();

                let mut msg = zmq::Message::new();

                for request_nbr in 0..10 {
                    println!("Sending Ruft {}...", request_nbr);
                    rpc.client.send("Ruft", 0).unwrap();

                    rpc.client.recv(&mut msg, 0).unwrap();
                    println!("Received Raft {}: {}", msg.as_str().unwrap(), request_nbr);
                }
                if let Some(func) = rpc.handlers.get("/hello") {
                    func(&Message{ content: HashMap::new() });
                }
            },
            Err(_) => println!("Something went wrong")
        }
    }

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
