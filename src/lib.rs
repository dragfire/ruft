use std::thread;
use std::collections::HashMap;

pub mod server;
pub mod node;
pub mod message;
pub mod storage;
pub mod util;
pub mod ticker;

use server::Server;
use message::Message;

pub fn start_all() {
    let address1 = "127.0.0.1:7000".to_string();
    let address2 = "127.0.0.1:7001".to_string();
    let addresses = vec![address1.to_owned(), address2.to_owned()];

    let mut server1 = Server::new(address1.to_owned(), addresses.to_owned()).unwrap();
    let mut server2 = Server::new(address2.to_owned(), addresses.to_owned()).unwrap();

    let mut handles: Vec<thread::JoinHandle<()>> = server1.start_all();
    handles.append(&mut server2.start_all());

    let mut msg = Message { content: HashMap::new() };
    msg.content.insert("key".to_string(), "value".to_string());
    msg.content.insert("key1".to_string(), "value".to_string());
    msg.content.insert("key2".to_string(), "value".to_string());
    msg.content.insert("key3".to_string(), "value".to_string());

    let astr = serde_json::to_string(&msg).unwrap();

    let client2 = server1.get_client(&address2);

    let mut msg = zmq::Message::new();

    client2.send(&astr, 0).unwrap();
    // check if get_client works
    server1.get_client(&address2).recv(&mut msg, 0).unwrap();
    assert_eq!(msg.as_str().unwrap(), "OK");

    let client1 = server2.get_client(&address1);
    client1.send(&astr, 0).unwrap();
    server2.get_client(&address1).recv(&mut msg, 0).unwrap();
    assert_eq!(msg.as_str().unwrap(), "OK");

    for handle in handles {
        handle.join().unwrap();
    }
}
