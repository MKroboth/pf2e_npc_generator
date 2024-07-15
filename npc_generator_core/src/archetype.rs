use serde::Deserialize;
use serde::Serialize;

use crate::AbilityStats;
use crate::Language;
use crate::Skill;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Archetype {
    name: String,
    prd_reference: Option<String>,
    perception: i16,
    languages: Vec<Language>,
    skills: HashMap<Skill, i16>,
    attributes: AbilityStats,
    items: Vec<String>,
    armor_class: i16,
    fortitude_save: i16,
    reflex_save: i16,
    will_save: i16,
    hp: i32,
    speed: u16,
    actions: Vec<String>,
    level: i8,
}

impl Archetype {
    pub fn name(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }

    pub fn perception(&self) -> i16 {
        self.perception
    }

    pub fn prd_reference(&self) -> Option<&str> {
        self.prd_reference.as_ref().map(|x| x.as_ref())
    }

    pub fn languages(&self) -> &[Language] {
        &self.languages
    }

    pub fn skills(&self) -> &HashMap<Skill, i16> {
        &self.skills
    }

    pub fn attributes(&self) -> &AbilityStats {
        &self.attributes
    }

    pub fn items_iter(&self) -> impl std::iter::Iterator<Item = Cow<str>> {
        self.items.iter().map(|x| Cow::Borrowed(x.as_str()))
    }

    pub fn armor_class(&self) -> i16 {
        self.armor_class
    }

    pub fn fortitude_save(&self) -> i16 {
        self.fortitude_save
    }

    pub fn reflex_save(&self) -> i16 {
        self.reflex_save
    }

    pub fn will_save(&self) -> i16 {
        self.will_save
    }

    pub fn speed(&self) -> u16 {
        self.speed
    }

    pub fn hp(&self) -> i32 {
        self.hp
    }

    pub fn actions(&self) -> &[String] {
        &self.actions
    }

    pub fn level(&self) -> i8 {
        self.level
    }
}

impl PartialEq for Archetype {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Archetype {}
