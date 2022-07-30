use std::ops::Deref;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payload {
    inner: Value,
}

impl Payload {
    pub fn new(payload: impl Into<Value>) -> Self {
        Payload {
            inner: payload.into(),
        }
    }
}

impl Deref for Payload {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
