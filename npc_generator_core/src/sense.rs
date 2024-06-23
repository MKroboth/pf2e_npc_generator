use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[repr(transparent)]
pub struct Sense(String);

impl Sense {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(value.as_ref().into())
    }
}
