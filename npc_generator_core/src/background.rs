use std::borrow::Cow;
use std::collections::HashSet;
use std::sync::Arc;

use crate::NamedElement;
use crate::Skill;
use crate::Trait;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

type BackgroundString = Arc<str>;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Background {
    pub name: BackgroundString,
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
    fn traits(&self) -> &[Trait] {
        &self.traits
    }
    fn name(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }

    fn formatted_name(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }
}

impl Background {
    pub fn new(
        name: impl AsRef<str>,
        traits: impl Into<Vec<Trait>>,
        trainings: HashSet<Skill>,
    ) -> Self {
        Self {
            name: name.as_ref().into(),
            traits: traits.into(),
            trainings: trainings.into_iter().collect::<Vec<_>>(),
        }
    }
}
