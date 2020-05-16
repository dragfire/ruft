use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use crate::util;

pub type Channel<T> = (mpsc::Sender<T>, mpsc::Receiver<T>);

pub struct Ticker {
    pub duration: Duration,
    pub channel: Channel<bool>,
}

impl Ticker {
    pub fn new(duration: Duration) -> Self {
        let channel: Channel<bool> = mpsc::channel();
        let tx1 = channel.0.clone();
        let ticker = Ticker {
            duration,
            channel,
        };

        thread::spawn(move || {
            loop {
                tx1.send(true).unwrap();
                thread::sleep(duration);
            }
        });

        ticker
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker() {
        let ticker = Ticker::new(Duration::from_millis(util::random_timeout()));
        let tx1 = ticker.channel.0.clone();

        thread::spawn( move || {
            thread::sleep(Duration::from_secs(2));
            tx1.send(false).unwrap();
        });

        loop {
            match ticker.channel.1.recv() {
                Ok(res) => {
                    if res {
                        println!("OK");
                    } else {
                        println!("DONE");
                        break;
                    }
                }
                Err(_) => {
                    println!("Err");
                    break;
                }
            }
        }
    }
}
