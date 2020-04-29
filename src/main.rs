fn main() {
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

    println!("Ruft Raft...");
}

