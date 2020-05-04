use std::thread;
use std::sync::mpsc;
use std::time;

mod util;
use util::random_timeout;

fn main() {
    println!("Ruft Raft...");
    let nodes = vec![
        "127.0.0.1:6000",
        "127.0.0.1:6001",
        "127.0.0.1:6002",
        "127.0.0.1:6003",
        "127.0.0.1:6004",
        "127.0.0.1:6005",
    ];

    for node in nodes {
        println!("{}", node);
    }

    let (tx, rx) = mpsc::channel::<i32>();

    let heartbeat_handle = thread::spawn(|| loop {
        thread::sleep(time::Duration::from_millis(random_timeout() as u64)); 
        println!("heartbeat");
    });

    let election_handle = thread::spawn(|| loop {
        thread::sleep(time::Duration::from_millis(random_timeout() as u64)); 
        println!("election");
    });

    heartbeat_handle.join().unwrap();
    election_handle.join().unwrap();
}
