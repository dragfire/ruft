use std::thread;
use std::sync::mpsc;
use std::time::Duration;

pub struct Ticker {
    duration: Duration,
    channel: mpsc::Receiver<bool>,
}

impl Ticker {
    pub fn new(duration: Duration) -> Self {
        let (tx, rx) = mpsc::channel();
        let ticker = Ticker {
            duration,
            channel: rx,
        };

        thread::spawn(move || {
            loop {
                tx.send(true).unwrap();
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
        let ticker = Ticker::new(Duration::from_millis(100));
        loop {
            match ticker.channel.recv() {
                Ok(_) => {
                    println!("OK");
                }
                Err(_) => {
                    println!("Err");
                }
            }
        }
    }
}
