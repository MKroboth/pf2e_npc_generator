use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};
use weighted_container::{WeightedHeapArray, WeightedVector};

pub mod generators;
mod newtypes;
mod weighted_container;

pub use newtypes::*;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum AbilityBoost {
    Boost(Ability),
    Flaw(Ability),
    Free,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Ability {
    Charisma,
    Constitution,
    Dexterity,
    Intelligence,
    Strength,
    Wisdom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityModifications(Vec<AbilityBoost>);

impl AbilityModifications {
    pub fn new(value: &[AbilityBoost]) -> Self {
        Self(value.into())
    }
}

impl Default for AbilityModifications {
    fn default() -> Self {
        Self(Vec::new())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[repr(transparent)]
pub struct Trait(String);

impl Trait {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(value.as_ref().into())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[repr(transparent)]
pub struct Sense(String);

impl Sense {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(value.as_ref().into())
    }
}

pub trait NamedElement {
    fn traits(&self) -> &[Trait];
    fn name(&self) -> &str;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Language {
    traits: Vec<Trait>,
    name: String,
}

impl Language {
    pub fn new(traits: Vec<Trait>, name: impl AsRef<str>) -> Self {
        Self {
            traits: traits,
            name: String::from(name.as_ref()),
        }
    }
}

impl NamedElement for Language {
    fn traits(&self) -> &[Trait] {
        &self.traits
    }
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Garganutan,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ancestry {
    pub traits: Vec<Trait>,
    pub name: String,
    pub ability_modifications: AbilityModifications,
    pub languages: Vec<Language>,
    pub senses: Vec<Sense>,
    pub size: Size,
    pub speed: u16,
    pub possible_eye_colors: Option<HashMap<String, i32>>,
    pub possible_hair_colors: Option<HashMap<String, i32>>,
    pub possible_hair_length: Option<HashMap<String, i32>>,
    pub possible_hair_type: Option<HashMap<String, i32>>,
    pub mutation_probabilities: HashMap<Mutation, f64>,
    pub specimen_names_per_sex: HashMap<String, HashMap<String, i32>>,
}
impl Eq for Heritage {}
impl Eq for Ancestry {}
impl PartialEq for Heritage {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
impl PartialEq for Ancestry {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
impl Hash for Heritage {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl Hash for Ancestry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Mutation {
    Heterochromia,
}

impl Ancestry {
    pub fn new(
        traits: Vec<Trait>,
        name: impl AsRef<str>,
        ability_modifications: AbilityModifications,
        languages: Vec<Language>,
        senses: Vec<Sense>,
        size: Size,
        speed: u16,
        possible_eye_colors: Option<HashMap<String, i32>>,
        possible_hair_colors: Option<HashMap<String, i32>>,
        possible_hair_length: Option<HashMap<String, i32>>,
        possible_hair_type: Option<HashMap<String, i32>>,
        mutation_probabilities: HashMap<Mutation, f64>,
        specimen_names_per_sex: HashMap<String, HashMap<String, i32>>,
    ) -> Self {
        Self {
            traits: traits,
            name: String::from(name.as_ref()),
            ability_modifications,
            languages: languages,
            senses: senses,
            size,
            speed,
            possible_eye_colors,
            possible_hair_colors,
            possible_hair_length,
            possible_hair_type,
            mutation_probabilities,
            specimen_names_per_sex,
        }
    }
}

impl NamedElement for Ancestry {
    fn traits(&self) -> &[Trait] {
        &self.traits
    }
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ValidAncestries {
    Any,
    AllOf(Vec<String>),
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Heritage {
    traits: Vec<Trait>,
    name: String,
    valid_ancestries: ValidAncestries,

    additional_eye_colors: HashMap<String, i32>,
    additional_hair_colors: HashMap<String, i32>,
    force_heterochromia: Option<String>,
}

impl Heritage {
    pub fn new(
        traits: Vec<Trait>,
        name: impl AsRef<str>,
        valid_ancestries: ValidAncestries,
        additional_eye_colors: HashMap<String, i32>,
        additional_hair_colors: HashMap<String, i32>,
        force_heterochromia: Option<&str>,
    ) -> Self {
        Self {
            traits,
            name: String::from(name.as_ref()),
            valid_ancestries,
            additional_eye_colors,
            additional_hair_colors,
            force_heterochromia: force_heterochromia.map(|x| String::from(x)),
        }
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

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Skill {
    Acrobatics,
    Arcana,
    Athletics,
    Crafting,
    Deception,
    Diplomacy,
    Intimidation,
    Lore(String),
    Medicine,
    Nature,
    Occultism,
    Performance,
    Religion,
    Society,
    Stealth,
    Survival,
    Thievery,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Proficienty {
    Trained,
    Expert,
    Master,
    Legendary,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Background {
    name: String,
    traits: Vec<Trait>,
    trainings: Vec<Skill>,
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

    pub fn trainings(&self) -> &[Skill] {
        &self.trainings
    }
}

#[derive(Debug)]
pub struct NpcOptions {
    pub ancestry: Option<Ancestry>,
    pub heritage: Option<Heritage>,
    pub background: Option<Background>,
    pub ancestry_weights: Option<AncestryWeights>,
}

#[macro_export]
macro_rules! traits {
    [] => (vec![]);
    [$($literal:literal),+ $(,)?] => ( vec![ $(Trait::new($literal)),+ ] );

}
#[macro_export]
macro_rules! language {
    ($literal:literal $($traits:literal),* $(,)?) => (
    Language::new(vec![$($traits),*], $literal) );

}
