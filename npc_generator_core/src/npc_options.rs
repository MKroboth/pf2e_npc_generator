use crate::{AgeRange, Ancestry, Archetype, Background, Heritage};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct NpcOptions {
    pub ancestry: Option<Ancestry>,
    pub heritage: Option<Option<Heritage>>,
    pub background: Option<Background>,
    pub archetype: Option<Archetype>,
    pub age_range: Option<AgeRange>,
    pub sex: Option<String>,
    pub enable_flavor_text: bool,
}
