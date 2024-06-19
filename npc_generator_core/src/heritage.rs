use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::{NamedElement, Trait, ValidAncestries, WeightMap};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Heritage {
    pub traits: Vec<Trait>,
    pub name: String,
    pub lineage: Option<String>,
    pub prd_reference: Option<String>,
    pub valid_ancestries: ValidAncestries,

    pub additional_eye_colors: WeightMap<String>,
    pub additional_hair_colors: WeightMap<String>,
    pub force_heterochromia: Option<String>,
}

impl Heritage {
    pub fn new(
        traits: Vec<Trait>,
        name: impl AsRef<str>,
        lineage: Option<&str>,
        valid_ancestries: ValidAncestries,
        additional_eye_colors: WeightMap<String>,
        additional_hair_colors: WeightMap<String>,
        force_heterochromia: Option<&str>,
        prd_reference: Option<&str>,
    ) -> Self {
        Self {
            traits,
            name: String::from(name.as_ref()),
            lineage: lineage.map(String::from),
            valid_ancestries,
            additional_eye_colors,
            additional_hair_colors,
            prd_reference: prd_reference.map(String::from),
            force_heterochromia: force_heterochromia.map(|x| String::from(x)),
        }
    }
}

impl Eq for Heritage {}
impl PartialEq for Heritage {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
impl Hash for Heritage {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl NamedElement for Heritage {
    fn traits(&self) -> &[Trait] {
        &self.traits
    }
    fn name(&self) -> &str {
        &self.name
    }
}
