use std::sync::Arc;
use std::{borrow::Cow, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::WeightMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeightPreset {
    name: Arc<str>,
    ancestry_weights: WeightMap<Arc<str>>,
    heritage_weights: WeightMap<Arc<str>>,
}

impl WeightPreset {
    pub fn new(
        name: impl AsRef<str>,
        ancestry_weights: Option<impl Into<WeightMap<Arc<str>>>>,
        heritage_weights: Option<impl Into<WeightMap<Arc<str>>>>,
    ) -> Self {
        Self {
            name: name.as_ref().into(),
            ancestry_weights: match ancestry_weights {
                Some(x) => x.into(),
                None => WeightMap::new(),
            },
            heritage_weights: match heritage_weights {
                Some(x) => x.into(),
                None => WeightMap::new(),
            },
        }
    }

    pub fn name(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }

    pub fn ancestry_weights(&self) -> &WeightMap<Arc<str>> {
        &self.ancestry_weights
    }
    pub fn heritage_weights(&self) -> &WeightMap<Arc<str>> {
        &self.heritage_weights
    }
}

impl PartialEq for WeightPreset {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
