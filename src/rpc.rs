use std::thread;
use zmq::Socket;
use std::collections::HashMap;

pub struct message;

type handle = fn() -> i32;

pub struct NodeRpc {
    pub address: String,
    pub client: Socket,
    pub handlers: HashMap<String, handle>,
}

fn check() -> i32 {
    println!("Check...");
    1
}

fn register_handlers(rpc: &mut NodeRpc) {
    let h: handle = check;
    rpc.handlers.insert("/hello".to_string(), h);
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
mod tests {
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
                if let Some(her) = rpc.handlers.get("/hello") {
                    assert_eq!(her(), 4);
                }
            },
            Err(_) => println!("Something went wrong")
        }
    }
}
