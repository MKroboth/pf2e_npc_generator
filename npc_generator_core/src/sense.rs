use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[repr(transparent)]
pub struct Sense(Arc<str>);

impl Sense {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(value.as_ref().into())
    }
}

impl AsRef<str> for Sense {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
