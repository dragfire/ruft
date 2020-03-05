use ruft::Storage;

fn main() {
    let mut storage: Storage<i32, &str> = Storage::new();
    storage.put(1, "ruft world");
    assert_eq!(storage.get(1), Some(&"ruft world"));
}

