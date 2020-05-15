use std::thread;
use std::sync::{Arc, atomic::{Ordering, AtomicBool}, Mutex};
use std::time::Duration;
use std::io::{self, BufRead};
use std::sync::mpsc::{self, TryRecvError, channel};
use ruft::util;
use ruft::message::Message;

/// Leader Election:
///     Spawn a thread that sleeps for every util::random_timeout() 
///     Two things that can happen while the countdown starts:
///         - Check if any other candidate requested for votes
///         - Check if it timeouts, vote for itself(change state to NodeState::Candidate),
///         increment termCount
///
struct StateInner {
    election_requested: bool,
    election_req_received: bool,
}

struct State {
    inner: Arc<Mutex<StateInner>>,
}

fn mpsc() -> thread::JoinHandle<()> {
    println!("Press enter to wake up the child thread");
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || loop {
        println!("Suspending...");
        match rx.recv() {
            Ok(_) => {
                println!("Working...");
                thread::sleep(Duration::from_millis(500));
            }
            Err(_) => {
                println!("Terminating...");
                break;
            }
        }
    });

    let mut line = String::new();
    let stdin = io::stdin();
    for _ in 0..4 {
        let _ = stdin.lock().read_line(&mut line);
        let _ = tx.send(());
    }

    handle
}

fn park_thread() {
    let flag = Arc::new(AtomicBool::new(false));
    let flag2 = Arc::clone(&flag);

    let parked_thread = thread::spawn(move || {
        // We want to wait until the flag is set. We *could* just spin, but using
        // park/unpark is more efficient.
        while !flag2.load(Ordering::Acquire) {
            println!("Parking thread");
            thread::park();
            // We *could* get here spuriously, i.e., way before the 10ms below are over!
            // But that is no problem, we are in a loop until the flag is set anyway.
            println!("Thread unparked");
        }
        println!("Flag received");
    });

    // Let some time pass for the thread to be spawned.
    thread::sleep(Duration::from_millis(10));

    // Set the flag, and let the thread wake up.
    // There is no race condition here, if `unpark`
    // happens first, `park` will return immediately.
    // Hence there is no risk of a deadlock.
    flag.store(true, Ordering::Release);
    println!("Unpark the thread");
    parked_thread.thread().unpark();

    parked_thread.join().unwrap();
}

fn election_thread(rx: Arc<Mutex<mpsc::Receiver<Message>>>) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        let rec = rx.lock().unwrap();
        match rec.recv_timeout(Duration::from_millis(util::random_timeout())) {
            Ok(msg) => {
                println!("received something {:?}", msg);
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                println!("Timeout...");
            }
            Err(_) => {
                println!("Error");
            }
        }
    })
}

fn main() {
    // mpsc().join().unwrap();

    let (tx, rx) = channel();
    let handle = election_thread(Arc::new(Mutex::new(rx)));
    let tx1 = tx.clone();

    thread::spawn(move || loop {
        let mut msg = Message::new();
        msg.put("key".to_string(), "value".to_string());

        thread::sleep(Duration::from_millis(100));
        tx1.send(msg).unwrap();
    }).join().unwrap();

    handle.join().unwrap();
}
