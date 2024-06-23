use crate::{AgeRange, Ancestry, Archetype, Background, Heritage, WeightMap};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct NpcOptions {
    pub ancestry: Option<Ancestry>,
    pub heritage: Option<Option<Heritage>>,
    pub background: Option<Background>,
    pub ancestry_weights: Option<WeightMap<String>>,
    pub archetype: Option<Archetype>,
    pub age_range: Option<AgeRange>,
    pub sex: Option<String>,
}
