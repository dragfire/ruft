use crate::rpc::NodeRpc;

pub struct Log;

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
    rpc:  NodeRpc,
    term_count: u64,
    log: Log,
    heartbeat_interval: i32,
}

impl Node {
    fn new(address: String, other_node_adds: Vec<String>) -> Self {
        let tcp_addr = String::from("tcp://") + &address;
        let rpc = NodeRpc::new(tcp_addr).unwrap();
        let log = Log{};

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
    #[ignore]
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
            let mut node = Node::new(node_addr.to_string(), nodes_str.to_owned());
            node.rpc.start();
            let mut msg = zmq::Message::new();
            for req_nbr in 0..10 {
                println!("Req: {}, sending...", req_nbr);
                node.rpc.get_client(&node_addr).send("hello how are you!", 0).unwrap();
                node.rpc.get_client(&node_addr).recv(&mut msg, 0).unwrap();
                println!("Received: {}", msg.as_str().unwrap());
            }
        }
    }

    #[test]
    fn test_interact_two_nodes() {
        let address1 = "127.0.0.1:6000".to_string();
        let address2 = "127.0.0.1:6001".to_string();
        let addresses = vec![address1.to_owned(), address2.to_owned()];

        let mut node1 = Node::new(address1.to_owned(), addresses.to_owned());
        let mut node2 = Node::new(address2.to_owned(), addresses.to_owned());
        
        node1.rpc.start();
        node2.rpc.start();

        let client2 = node1.rpc.get_client(&address2);

        let mut msg = zmq::Message::new();

        client2.send("RUFT to Server 2", 0).unwrap();
        // check if get_client works
        node1.rpc.get_client(&address2).recv(&mut msg, 0).unwrap();
        println!("Received: {}", msg.as_str().unwrap());

        let client1 = node2.rpc.get_client(&address1);
        client1.send("RUFT to server 1", 0).unwrap();
        node2.rpc.get_client(&address1).recv(&mut msg, 0).unwrap();
        println!("Received: {}", msg.as_str().unwrap());
    }
}
