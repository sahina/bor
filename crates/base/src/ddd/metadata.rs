use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents MetaData that is passed along with a payload in a Message.
/// Typically, the MetaData contains information about the message payload that isn't
/// "domain-specific". Examples are originating IP-address or executing User ID.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaData {
    store: HashMap<String, Value>,
}

impl MetaData {
    /// Creates a new MetaData from default values.
    pub fn new() -> Self {
        MetaData::default()
    }

    /// Adds key/value pair
    pub fn add(&mut self, key: impl Into<String>, value: impl Into<Value>) {
        self.store.insert(key.into(), value.into());
    }

    /// Inserts a new key/value pair and returns owned Self
    pub fn insert(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.store.insert(key.into(), value.into());
        self
    }

    /// Returns true if key exists in `MetaData` store.
    pub fn contains_key(&self, key: impl Into<String>) -> bool {
        self.store.contains_key(&key.into())
    }

    /// Returns an option of `Value` from given key.
    pub fn get(&self, key: impl Into<String>) -> Option<Value> {
        self.store.get(key.into().as_str()).cloned()
    }

    /// Extends MetaData with given one.
    pub fn extend(&mut self, metadata: MetaData) {
        self.store.extend(metadata.store)
    }

    /// Returns true if MetaData store is empty.
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    /// Returns number of items in the MetaData store.
    pub fn len(&self) -> usize {
        self.store.len()
    }
}

#[cfg(test)]
mod metadata_test {
    use crate::ddd::metadata::MetaData;

    #[test]
    fn no_key() {
        let metadata = MetaData::new();

        assert!(!metadata.contains_key("does not exist"));
    }

    #[test]
    fn get_value() {
        let metadata = MetaData::new().insert("key", "value");
        let value = metadata.get("key");

        assert!(metadata.contains_key("key"));
        assert_eq!(metadata.get("key"), value);
    }

    #[test]
    fn get_no_value() {
        let metadata = MetaData::new().insert("key", "value");
        let value = metadata.get("does not exist");

        assert_eq!(value, None);
    }
}
