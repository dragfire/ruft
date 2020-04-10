use std::io;
use std::thread;
use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;
use zmq::Socket;

pub struct Storage<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> Storage<K, V>
where K: Hash + Eq,
      V: Clone,
{
    pub fn new() -> Self {
        Storage { data: HashMap::new() }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.data.get(&key)
    }

    pub fn put(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }
}

// ======================== Node =============================

enum NodeState {
    Follower,
    Candidate,
    Leader,
}

struct Node<'a> {
    state: NodeState,
    term: u64,
    rpc: &'a NodeRpc,
}

// ======================== NodeRpc =============================

pub fn spawn_server() {
    thread::spawn(|| {
        let context = zmq::Context::new();
        let responder = context.socket(zmq::REP).unwrap();

        assert!(responder.bind("tcp://*:5555").is_ok());

        let mut msg = zmq::Message::new();
        loop {
            responder.recv(&mut msg, 0).unwrap();
            println!("Received {}", msg.as_str().unwrap());
            thread::sleep(Duration::from_millis(1000));
            responder.send("World", 0).unwrap();
        }
    });
}

pub fn client() {
    println!("Connecting to hello world server...\n");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let mut msg = zmq::Message::new();

    for request_nbr in 0..10 {
        println!("Sending Hello {}...", request_nbr);
        requester.send("Hello", 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("Received World {}: {}", msg.as_str().unwrap(), request_nbr);
    }
}

struct NodeRpc {
    address: String,
    server: Result<Socket, zmq::Error>,
    client: Result<Socket, zmq::Error>,
}

impl NodeRpc {
    pub fn new(address: String) -> NodeRpc {
        let context = zmq::Context::new();
        let client = context.socket(zmq::REQ);
        let server = context.socket(zmq::REP);
        NodeRpc { address, server, client }
    }

    pub fn start() -> Result<()> {

        Ok(())
    }
}

// ======================== Tests ========================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noderpc() {
        let rpc = NodeRpc::new("127.0.0.0:8080".to_string());
        rpc.start();
    }

    #[test]
    fn test_storage() {
        let mut storage: Storage<i32, &str> = Storage::new();
        storage.put(1, "ruft world");
        storage.put(2, "hello world");
    }
}
