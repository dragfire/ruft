use std::thread;
use std::time::Duration;

use ruft::*;

fn main() {

    spawn_server();
    client();
}

