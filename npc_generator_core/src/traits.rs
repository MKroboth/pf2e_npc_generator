use std::{fmt::Display, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Trait(Arc<str>);

impl Display for Trait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Trait {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(value.as_ref().into())
    }
}

impl Default for Trait {
    fn default() -> Self {
        Self(String::default().into())
    }
}

impl AsRef<str> for Trait {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Into<String> for Trait {
    fn into(self) -> String {
        self.0.to_string()
    }
}
