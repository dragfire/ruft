use ruft::Storage;
use std::thread;
use std::time::Duration;

fn spawn_server() {
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

fn client() {
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

fn main() {
    let mut storage: Storage<i32, &str> = Storage::new();
    storage.put(1, "ruft world");

    assert_eq!(storage.get(1), Some(&"ruft world"));

    spawn_server();
    client();
}

