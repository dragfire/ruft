use std::collections::HashMap;
use std::hash::Hash;

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
