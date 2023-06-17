use std::collections::HashMap;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Creates a `KvStore`.
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}

#[cfg(test)]
mod tests {
    use crate::KvStore;

    // Should get previously stored value
    #[test]
    fn get_stored_value() {
        let mut store = KvStore::new();

        store.set("key1".to_owned(), "value1".to_owned());
        store.set("key2".to_owned(), "value2".to_owned());

        assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
        assert_eq!(store.get("key2".to_owned()), Some("value2".to_owned()));
    }

    // Should overwrite existent value
    #[test]
    fn overwrite_value() {
        let mut store = KvStore::new();

        store.set("key1".to_owned(), "value1".to_owned());
        assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));

        store.set("key1".to_owned(), "value2".to_owned());
        assert_eq!(store.get("key1".to_owned()), Some("value2".to_owned()));
    }

    // Should get `None` when getting a non-existent key
    #[test]
    fn get_non_existent_value() {
        let mut store = KvStore::new();

        store.set("key1".to_owned(), "value1".to_owned());
        assert_eq!(store.get("key2".to_owned()), None);
    }

    #[test]
    fn remove_key() {
        let mut store = KvStore::new();

        store.set("key1".to_owned(), "value1".to_owned());
        store.remove("key1".to_owned());
        assert_eq!(store.get("key1".to_owned()), None);
    }
}
