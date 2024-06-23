use serde::Deserialize;
use serde::Serialize;

use crate::AbilityStats;
use crate::Language;
use crate::Skill;
use std::collections::HashMap;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Archetype {
    pub name: String,
    pub prd_reference: Option<String>,
    pub perception: i16,
    pub languages: Vec<Language>,
    pub skills: HashMap<Skill, i16>,
    pub attributes: AbilityStats,
    pub items: Vec<String>,
    pub armor_class: i16,
    pub fortitude_save: i16,
    pub reflex_save: i16,
    pub will_save: i16,
    pub hp: i32,
    pub speed: u16,
    pub actions: Vec<String>,
    pub level: i8,
}

impl PartialEq for Archetype {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Archetype {}
