use std::hash::Hash;
use std::collections::HashMap;

pub struct Storage<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> Storage<K, V>
where K: Hash + Eq,
      V: Clone,
{
    pub fn new() -> Self {
        Storage { data: HashMap::new() }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.data.get(&key)
    }

    pub fn put(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage() {
        let mut storage: Storage<i32, &str> = Storage::new();
        storage.put(1, "ruft world");
        storage.put(2, "hello world");
    }
}
