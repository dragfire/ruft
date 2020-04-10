use std::thread;
use std::time::Duration;

use ruft::*;

fn main() {
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

