use std::collections::HashSet;

use crate::NamedElement;
use crate::Skill;
use crate::Trait;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Background {
    pub name: String,
    pub traits: Vec<Trait>,
    pub trainings: Vec<Skill>,
}
impl PartialEq for Background {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
impl Hash for Background {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl Eq for Background {}
impl NamedElement for Background {
    fn name(&self) -> &str {
        &self.name
    }
    fn traits(&self) -> &[Trait] {
        &self.traits
    }
}

impl Background {
    pub fn new(name: impl AsRef<str>, traits: Vec<Trait>, trainings: HashSet<Skill>) -> Self {
        Self {
            name: String::from(name.as_ref()),
            traits,
            trainings: trainings.into_iter().collect::<Vec<_>>().into(),
        }
    }
}
