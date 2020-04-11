use std::thread;
use zmq::Socket;

pub struct NodeRpc {
    pub address: String,
    pub client: Socket,
}

impl NodeRpc {
    pub fn new(address: String) -> Result<NodeRpc, zmq::Error> {
        let context = zmq::Context::new();
        let client = context.socket(zmq::REQ)?;
        client.connect(&address)?;

        Ok(NodeRpc { address, client })
    }

    pub fn start(&self) {
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
mod tests {
    use super::*;

    #[test]
    fn test_noderpc() {
        let rpc = NodeRpc::new(String::from("tcp://localhost:5555"));

        match rpc {
            Ok(rpc) => {
                rpc.start();

                let mut msg = zmq::Message::new();

                for request_nbr in 0..10 {
                    println!("Sending Ruft {}...", request_nbr);
                    rpc.client.send("Ruft", 0).unwrap();

                    rpc.client.recv(&mut msg, 0).unwrap();
                    println!("Received Raft {}: {}", msg.as_str().unwrap(), request_nbr);
                }
            },
            Err(_) => println!("Something went wrong")
        }
    }
}
