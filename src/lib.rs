use std::io;
use std::hash::Hash;
use std::collections::HashMap;

mod rpc;

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

// ======================== Log ==============================

pub struct Log;

// ======================== Node =============================

enum NodeState {
    Follower,
    Candidate,
    Leader,
}

pub struct Node {
    state: NodeState,
    term: u64,
    address: String,
    other_node_adds: Vec<String>, 
    rpc:  rpc::NodeRpc,
    term_count: u64,
    log: Log,
    heartbeat_interval: i32,
}

impl Node {
    fn new(address: String, other_node_adds: Vec<String>, rpc: rpc::NodeRpc, log: Log) -> Self {
        Node {
            state: NodeState::Follower,
            term: 0,
            term_count: 0,
            heartbeat_interval: 500, // ms?
            rpc, 
            log,
            address,
            other_node_adds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node() {
        let nodes = vec![
            "127.0.0.1:6000",
            "127.0.0.1:6001",
            "127.0.0.1:6002",
            "127.0.0.1:6003",
            "127.0.0.1:6004",
            "127.0.0.1:6005",
        ];

        let nodes_str: Vec<String> = nodes.iter().map(|node| node.to_owned().to_string()).collect();

        for node_addr in &nodes {
            let tcp_addr = String::from("tcp://") + &node_addr;
            let rpc = rpc::NodeRpc::new(tcp_addr);
            let log = Log{};
            match rpc {
                Ok(node_rpc) => {
                    let mut node = Node::new(node_addr.to_string(), nodes_str.to_owned(), node_rpc, log);
                    node.rpc.start();
                    let mut msg = zmq::Message::new();
                    for req_nbr in 0..10 {
                        println!("Req: {}, sending...", req_nbr);
                        node.rpc.client.send("hello how are you!", 0).unwrap();
                        node.rpc.client.recv(&mut msg, 0).unwrap();
                        println!("Received: {}", msg.as_str().unwrap());
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
    }

    #[test]
    fn test_storage() {
        let mut storage: Storage<i32, &str> = Storage::new();
        storage.put(1, "ruft world");
        storage.put(2, "hello world");
    }
}
