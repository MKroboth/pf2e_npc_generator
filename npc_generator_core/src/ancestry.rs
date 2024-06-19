use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use serde::{Deserialize, Serialize};

use crate::{
    AbilityModifications, AgeRange, AgeRanges, Language, Mutation, NamedElement, Sense, Size,
    Trait, WeightMap,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ancestry {
    pub traits: Vec<Trait>,
    pub name: String,
    pub ability_modifications: AbilityModifications,
    pub languages: Vec<Language>,
    pub senses: Vec<Sense>,
    pub size: Size,
    pub speed: u16,
    pub possible_eye_colors: Option<WeightMap<String>>,
    pub possible_hair_colors: Option<WeightMap<String>>,
    pub possible_hair_length: Option<WeightMap<String>>,
    pub possible_hair_type: Option<WeightMap<String>>,
    pub mutation_probabilities: HashMap<Mutation, f64>,
    pub specimen_surnames: Option<WeightMap<String>>,
    pub age_ranges: AgeRanges,
    pub age_range_distribution: WeightMap<AgeRange>,
    pub prd_reference: Option<String>,
    #[serde(default)]
    pub is_asexual: bool,
    #[serde(default = "default_hair_substance")]
    pub hair_substance: String,
}
fn default_hair_substance() -> String {
    String::from("hair")
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
        possible_eye_colors: Option<WeightMap<String>>,
        possible_hair_colors: Option<WeightMap<String>>,
        possible_hair_length: Option<WeightMap<String>>,
        possible_hair_type: Option<WeightMap<String>>,
        mutation_probabilities: HashMap<Mutation, f64>,
        specimen_surnames: Option<WeightMap<String>>,
        age_ranges: AgeRanges,
        age_range_distribution: WeightMap<AgeRange>,
        prd_reference: Option<String>,
        is_asexual: bool,
        hair_substance: String,
    ) -> Self {
        Self {
            traits,
            name: String::from(name.as_ref()),
            ability_modifications,
            languages,
            senses,
            size,
            speed,
            possible_eye_colors,
            possible_hair_colors,
            possible_hair_length,
            possible_hair_type,
            mutation_probabilities,
            specimen_surnames,
            age_ranges,
            age_range_distribution,
            prd_reference,
            is_asexual,
            hair_substance,
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

impl Eq for Ancestry {}

impl PartialEq for Ancestry {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Hash for Ancestry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
