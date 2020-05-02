use crate::server::Server;
use crate::util;

pub struct Log;

pub enum NodeState {
    Follower,
    Candidate,
    Leader,
}

pub struct Node {
    pub state: NodeState,
    pub term: u64,
    pub address: String,
    pub other_node_adds: Vec<String>, 
    pub term_count: u64,
    pub log: Log,
    pub election_timeout: i32,
    pub heartbeat_timeout: i32,
}

impl Node {
    pub fn new(address: String, other_node_adds: Vec<String>) -> Self {
        let log = Log{};

        Node {
            log,
            address,
            other_node_adds,
            term: 0,
            term_count: 0,
            heartbeat_timeout: util::random_timeout(),
            election_timeout: util::random_timeout(),
            state: NodeState::Follower,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_nodes() {
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
            let mut server = Server::new(node_addr.to_string(), nodes_str.to_owned()).unwrap();
            server.start();
            let mut msg = zmq::Message::new();
            for _ in 0..10 {
                server.get_client(&node_addr).send("hello how are you!", 0).unwrap();
                server.get_client(&node_addr).recv(&mut msg, 0).unwrap();
                assert_eq!("OK", msg.as_str().unwrap());
            }
        }
    }

    #[test]
    fn test_interact_two_nodes() {
        let address1 = "127.0.0.1:7000".to_string();
        let address2 = "127.0.0.1:7001".to_string();
        let addresses = vec![address1.to_owned(), address2.to_owned()];

        let mut server1 = Server::new(address1.to_owned(), addresses.to_owned()).unwrap();
        let mut server2 = Server::new(address2.to_owned(), addresses.to_owned()).unwrap();

        let join_handle1 = server1.start();
        let join_handle2 = server2.start();

        // join_handle1.join().unwrap();
        // join_handle2.join().unwrap();

        let client2 = server1.get_client(&address2);

        let mut msg = zmq::Message::new();

        client2.send("RUFT to Server 2", 0).unwrap();
        // check if get_client works
        server1.get_client(&address2).recv(&mut msg, 0).unwrap();
        assert_eq!(msg.as_str().unwrap(), "OK");

        let client1 = server2.get_client(&address1);
        client1.send("RUFT to server 1", 0).unwrap();
        server2.get_client(&address1).recv(&mut msg, 0).unwrap();
        assert_eq!(msg.as_str().unwrap(), "OK");
    }
}
