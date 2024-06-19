use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fmt::Display,
    hash::{Hash, Hasher},
    ops::Range,
};

pub mod generators;
mod newtypes;

mod heritage;
pub use heritage::*;
mod ancestry;
pub use ancestry::*;
mod statblock;
pub use newtypes::*;
pub use statblock::*;

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Trait(String);

impl Display for Trait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
            traits,
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct DiceFormula(pub i8, pub Die);
impl DiceFormula {
    pub fn roll(&self, mut rng: &mut impl Rng) -> i32 {
        let mut result: i32 = 0;
        for _ in 0..self.0 {
            result += self.1.roll(&mut rng) as i32;
        }
        result
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

impl Die {
    pub fn roll(&self, rng: &mut impl Rng) -> i8 {
        rng.gen_range(match self {
            Die::D4 => 1..=4,
            Die::D6 => 1..=6,
            Die::D8 => 1..=8,
            Die::D10 => 1..=10,
            Die::D12 => 1..=12,
            Die::D20 => 1..=20,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgeRanges {
    pub child: u64,
    pub youth: u64,
    pub adulthood: u64,
    pub middle_age: u64,
    pub old: u64,
    pub venerable: u64,
    pub lifespan: u64,
}

impl AgeRanges {
    pub fn get(&self, range: &AgeRange) -> Range<u64> {
        match range {
            AgeRange::Infant => 0..self.child,
            AgeRange::Child => self.child..self.youth,
            AgeRange::Youth => self.youth..self.adulthood,
            AgeRange::Adult => self.adulthood..self.middle_age,
            AgeRange::MiddleAged => self.middle_age..self.old,
            AgeRange::Old => self.old..self.venerable,
            AgeRange::Venerable => self.venerable..self.lifespan + 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum AgeRange {
    Infant,
    Child,
    Youth,
    Adult,
    MiddleAged,
    Old,
    Venerable,
}

impl Default for AgeRange {
    fn default() -> Self {
        AgeRange::Adult
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Mutation {
    Heterochromia,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ValidAncestries {
    Any,
    AllOf(Vec<String>),
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

impl Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Skill::Acrobatics => "Acrobatics".to_string(),
                Skill::Arcana => "Arcana".to_string(),
                Skill::Athletics => "Athletics".to_string(),
                Skill::Crafting => "Crafting".to_string(),
                Skill::Deception => "Deception".to_string(),
                Skill::Diplomacy => "Diplomacy".to_string(),
                Skill::Intimidation => "Intimidation".to_string(),
                Skill::Lore(x) => format!("{} Lore", x).to_string(),
                Skill::Medicine => "Medicine".to_string(),
                Skill::Nature => "Nature".to_string(),
                Skill::Occultism => "Occultism".to_string(),
                Skill::Performance => "Performance".to_string(),
                Skill::Religion => "Religion".to_string(),
                Skill::Society => "Society".to_string(),
                Skill::Stealth => "Stealth".to_string(),
                Skill::Survival => "Survival".to_string(),
                Skill::Thievery => "Thievery".to_string(),
            }
        )
    }
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

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct NpcOptions {
    pub ancestry: Option<Ancestry>,
    pub heritage: Option<Option<Heritage>>,
    pub background: Option<Background>,
    pub ancestry_weights: Option<WeightMap<String>>,
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
