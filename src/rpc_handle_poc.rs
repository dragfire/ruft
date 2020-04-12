// TEST POC
/**
 * Handle: trait, process(msg: &message)
 * handlers: Map<String, Handle>
 * handle: Handle, handle.process(message)
 * struct HeartBeat;
 * impl Handle for HeartBeat {
 *  fn process(m: &message) { // do something }
 * }
 *
 */

#[derive(Debug)]
struct Message {
    key: i32,
    val: String,
}

impl Message {
    fn new() -> Self {
        Message {
            key: 1,
            val: "Raft".to_string(),
        }
    }
}

type handle = fn(&Message);

fn process(msg: &Message) {
    println!("{:?}", msg);
}

fn send_heart_beat(msg: &Message) {
    println!("{:?}", msg);
}

#[cfg(test)]
mod node_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_handle() {
        assert_eq!(2+3, 5);
        let mut msg = Message::new();
        let mut map: HashMap<String, handle> = HashMap::new();

        map.insert("/hello".to_string(), process);
        map.insert("/test".to_string(), send_heart_beat);

        if let Some(func) = map.get("/hello") {
            func(&msg);
        }

        if let Some(func) = map.get("/test") {
            msg.key = 4;
            msg.val = "Ruft!!!".to_string();
            func(&msg);
        }
    }
}
