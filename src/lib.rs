use std::io;
use std::collections::HashMap;
use std::hash::Hash;
use std::net::{TcpListener, TcpStream};

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

struct NodeRpc {
    address: String,
}

impl NodeRpc {
    pub fn new(address: String) -> NodeRpc {
        NodeRpc { address }
    }

    fn handle_connection(&self, stream: TcpStream) {
        println!("{:?}", stream);
    }

    pub fn start(&self) -> io::Result<()> {
        let listener = TcpListener::bind(self.address.clone())?;

        for stream in listener.incoming() {
            self.handle_connection(stream?);
        }
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
}
