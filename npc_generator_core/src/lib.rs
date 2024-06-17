use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use weighted_container::{WeightedHeapArray, WeightedVector};

mod generators;
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
pub struct AbilityModifications(Arc<[AbilityBoost]>);

impl AbilityModifications {
    pub fn new(value: Vec<AbilityBoost>) -> Self {
        Self(value.into_boxed_slice().into())
    }
}

impl Default for AbilityModifications {
    fn default() -> Self {
        Self(Vec::new().into_boxed_slice().into())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[repr(transparent)]
pub struct Trait(Arc<str>);

impl Trait {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(String::from(value.as_ref()).into_boxed_str().into())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[repr(transparent)]
pub struct Sense(Arc<str>);

impl Sense {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(String::from(value.as_ref()).into_boxed_str().into())
    }
}

pub trait NamedElement {
    fn traits(&self) -> &[Trait];
    fn name(&self) -> &str;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Language {
    traits: Arc<[Trait]>,
    name: Arc<str>,
}

impl Language {
    pub fn new(traits: Vec<Trait>, name: impl AsRef<str>) -> Self {
        Self {
            traits: traits.into_boxed_slice().into(),
            name: String::from(name.as_ref()).into_boxed_str().into(),
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Ancestry {
    traits: Arc<[Trait]>,
    name: Arc<str>,
    ability_modifications: AbilityModifications,
    languages: Arc<[Language]>,
    senses: Arc<[Sense]>,
    size: Size,
    speed: u16,
    possible_eye_colors: Option<WeightedHeapArray<Arc<str>>>,
    possible_hair_colors: Option<WeightedHeapArray<Arc<str>>>,
    mutation_probabilities: WeightedHeapArray<Mutation>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Mutation {
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
        possible_eye_colors: Option<WeightedHeapArray<Arc<str>>>,
        possible_hair_colors: Option<WeightedHeapArray<Arc<str>>>,
    ) -> Self {
        Self {
            traits: traits.into_boxed_slice().into(),
            name: String::from(name.as_ref()).into_boxed_str().into(),
            ability_modifications,
            languages: languages.into_boxed_slice().into(),
            senses: senses.into_boxed_slice().into(),
            size,
            speed,
            possible_eye_colors,
            possible_hair_colors,
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
    AllOf(Arc<[Arc<str>]>),
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Heritage {
    traits: Arc<[Trait]>,
    name: Arc<str>,
    valid_ancestries: ValidAncestries,

    additional_eye_colors: WeightedHeapArray<Arc<str>>,
    additional_hair_colors: WeightedHeapArray<Arc<str>>,
    force_heterochromia: Option<Arc<str>>,
}

impl Heritage {
    pub fn new(
        traits: Vec<Trait>,
        name: impl AsRef<str>,
        valid_ancestries: ValidAncestries,
        additional_eye_colors: WeightedVector<String>,
        additional_hair_colors: WeightedVector<String>,
        force_heterochromia: Option<&str>,
    ) -> Self {
        Self {
            traits: traits.into_boxed_slice().into(),
            name: String::from(name.as_ref()).into_boxed_str().into(),
            valid_ancestries,
            additional_eye_colors: WeightedHeapArray(
                additional_eye_colors
                    .map_elements(|element| element.into_boxed_str().into())
                    .0
                    .into_boxed_slice()
                    .into(),
            ),
            additional_hair_colors: WeightedHeapArray(
                additional_hair_colors
                    .map_elements(|element| element.into_boxed_str().into())
                    .0
                    .into_boxed_slice()
                    .into(),
            ),
            force_heterochromia: force_heterochromia
                .map(|x| String::from(x).into_boxed_str().into()),
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
    Lore(Arc<str>),
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Background {
    name: Arc<str>,
    traits: Arc<[Trait]>,
    trainings: Arc<[Skill]>,
}

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
            name: String::from(name.as_ref()).into_boxed_str().into(),
            traits: traits.into_boxed_slice().into(),
            trainings: trainings
                .into_iter()
                .collect::<Vec<_>>()
                .into_boxed_slice()
                .into(),
        }
    }

    pub fn trainings(&self) -> &[Skill] {
        &self.trainings
    }
}

#[derive(Debug)]
pub struct NpcOptions {
    ancestry: Option<Ancestry>,
    heritage: Option<Heritage>,
    background: Option<Background>,
    ancestry_weights: Option<AncestryWeights>,
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
