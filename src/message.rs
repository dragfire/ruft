use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point() {
        let point = Point { x: 1, y: 2 };
        // Convert the Point to a JSON string.
        let serialized = serde_json::to_string(&point).unwrap();

        // Prints serialized = {"x":1,"y":2}
        println!("serialized = {}", serialized);

        // Convert the JSON string back to a Point.
        let deserialized: Point = serde_json::from_str(&serialized).unwrap();

        // Prints deserialized = Point { x: 1, y: 2 }
        println!("deserialized = {:?}", deserialized);
        
        let mut message = HashMap::new();
        message.insert("reply_to", "127.0.0.1:6000");
        message.insert("action", "heartbeat");

        let m_str = serde_json::to_string(&message).unwrap();
        println!("{}", m_str);

        let mut msg: HashMap<String, String> = serde_json::from_str(&m_str).unwrap();
        println!("{:?}", msg);
    }
}
