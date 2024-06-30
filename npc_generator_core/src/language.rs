use serde::Deserialize;
use serde::Serialize;

use crate::NamedElement;
use crate::Trait;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Language {
    traits: Vec<Trait>,
    name: String,
}

impl Language {
    pub fn new(traits: Vec<Trait>, name: impl AsRef<str>) -> Self {
        Self {
            traits,
            name: String::from(name.as_ref()),
        }
    }
}

impl NamedElement for Language {
    fn traits(&self) -> &[Trait] {
        &self.traits
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}
