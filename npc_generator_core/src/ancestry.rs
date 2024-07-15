use lazy_static::lazy_static;
use std::{
    borrow::Cow,
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};

use serde::{Deserialize, Serialize};

use crate::formats::Formats;
use crate::{
    AbilityModifications, AgeRange, AgeRanges, Language, Mutation, NamedElement, Sense, Size,
    Trait, WeightMap,
};

type AncestryString = Arc<str>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ancestry {
    traits: Vec<Trait>,
    name: AncestryString,
    ability_modifications: AbilityModifications,
    languages: Vec<Language>,
    senses: Vec<Sense>,
    size: Size,
    speed: u16,
    possible_eye_colors: Option<WeightMap<AncestryString>>,
    possible_hair_colors: Option<WeightMap<AncestryString>>,
    possible_hair_length: Option<WeightMap<AncestryString>>,
    possible_hair_type: Option<WeightMap<AncestryString>>,
    mutation_probabilities: HashMap<Mutation, f64>,
    possible_skin_tone: WeightMap<AncestryString>,
    possible_skin_texture: WeightMap<AncestryString>,
    specimen_surnames: Option<WeightMap<AncestryString>>,
    age_ranges: AgeRanges,
    age_range_distribution: WeightMap<AgeRange>,
    prd_reference: Option<AncestryString>,
    #[serde(default)]
    is_asexual: bool,
    #[serde(default = "default_hair_substance")]
    hair_substance: AncestryString,
    #[serde(default = "default_skin_substance")]
    skin_substance: AncestryString,
    base_hp: u8,
    #[serde(default)]
    formats: Formats,
}
lazy_static! {
    static ref DEFAULT_HAIR_SUBSTANCE: Arc<str> = "hair".into();
    static ref DEFAULT_SKIN_SUBSTANCE: Arc<str> = "skin".into();
}
fn default_hair_substance() -> Arc<str> {
    DEFAULT_HAIR_SUBSTANCE.clone()
}
fn default_skin_substance() -> Arc<str> {
    DEFAULT_SKIN_SUBSTANCE.clone()
}
impl Ancestry {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        traits: impl Into<Vec<Trait>>,
        name: impl AsRef<str>,
        ability_modifications: AbilityModifications,
        languages: impl Into<Vec<Language>>,
        senses: impl Into<Vec<Sense>>,
        size: Size,
        speed: u16,
        possible_eye_colors: Option<impl Into<WeightMap<AncestryString>>>,
        possible_hair_colors: Option<impl Into<WeightMap<AncestryString>>>,
        possible_hair_length: Option<impl Into<WeightMap<AncestryString>>>,
        possible_hair_type: Option<impl Into<WeightMap<AncestryString>>>,
        possible_skin_tone: impl Into<WeightMap<AncestryString>>,
        possible_skin_texture: impl Into<WeightMap<AncestryString>>,
        mutation_probabilities: impl Into<HashMap<Mutation, f64>>,
        specimen_surnames: Option<impl Into<WeightMap<AncestryString>>>,
        age_ranges: AgeRanges,
        age_range_distribution: WeightMap<AgeRange>,
        prd_reference: Option<impl AsRef<str>>,
        is_asexual: bool,
        hair_substance: impl AsRef<str>,
        skin_substance: impl AsRef<str>,
        base_hp: u8,
        formats: Formats,
    ) -> Self {
        Self {
            traits: traits.into(),
            name: name.as_ref().into(),
            ability_modifications,
            languages: languages.into(),
            senses: senses.into(),
            size,
            speed,
            possible_eye_colors: possible_eye_colors.map(Into::into),
            possible_hair_colors: possible_hair_colors.map(Into::into),
            possible_hair_length: possible_hair_length.map(Into::into),
            possible_hair_type: possible_hair_type.map(Into::into),
            mutation_probabilities: mutation_probabilities.into(),
            specimen_surnames: specimen_surnames.map(Into::into),
            age_ranges,
            age_range_distribution,
            prd_reference: prd_reference.map(|x| x.as_ref().into()),
            is_asexual,
            hair_substance: hair_substance.as_ref().into(),
            skin_substance: skin_substance.as_ref().into(),
            possible_skin_tone: possible_skin_tone.into(),
            possible_skin_texture: possible_skin_texture.into(),
            base_hp,
            formats,
        }
    }

    pub fn ability_modifications(&self) -> &AbilityModifications {
        &self.ability_modifications
    }
    pub fn languages(&self) -> &[Language] {
        &self.languages
    }
    pub fn senses(&self) -> &[Sense] {
        &self.senses
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn speed(&self) -> u16 {
        self.speed
    }

    pub fn possible_eye_colors(&self) -> Option<&WeightMap<impl AsRef<str> + Eq + Hash>> {
        self.possible_eye_colors.as_ref()
    }

    pub fn possible_hair_colors(&self) -> Option<&WeightMap<impl AsRef<str> + Eq + Hash>> {
        self.possible_hair_colors.as_ref()
    }

    pub fn possible_hair_length(&self) -> Option<&WeightMap<impl AsRef<str> + Eq + Hash>> {
        self.possible_hair_length.as_ref()
    }

    pub fn possible_hair_type(&self) -> Option<&WeightMap<impl AsRef<str> + Eq + Hash>> {
        self.possible_hair_type.as_ref()
    }

    pub fn mutation_probabilities(&self) -> &HashMap<Mutation, f64> {
        &self.mutation_probabilities
    }

    pub fn possible_skin_tone(&self) -> &WeightMap<impl AsRef<str> + Eq + Hash> {
        &self.possible_skin_tone
    }

    pub fn possible_skin_texture(&self) -> &WeightMap<impl AsRef<str> + Eq + Hash> {
        &self.possible_skin_texture
    }

    pub fn specimen_surnames(&self) -> Option<&WeightMap<impl AsRef<str> + Eq + Hash>> {
        self.specimen_surnames.as_ref()
    }

    pub fn age_ranges(&self) -> &AgeRanges {
        &self.age_ranges
    }

    pub fn age_range_distribution(&self) -> &WeightMap<AgeRange> {
        &self.age_range_distribution
    }

    pub fn prd_reference(&self) -> Option<&str> {
        self.prd_reference.as_deref()
    }

    pub fn is_asexual(&self) -> bool {
        self.is_asexual
    }

    pub fn hair_substance(&self) -> &str {
        &self.hair_substance
    }

    pub fn skin_substance(&self) -> &str {
        &self.skin_substance
    }

    pub fn base_hp(&self) -> u8 {
        self.base_hp
    }

    pub fn formats(&self) -> &Formats {
        &self.formats
    }
}

impl NamedElement for Ancestry {
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

impl Eq for Ancestry {}

impl PartialEq for Ancestry {
    fn eq(&self, other: &Self) -> bool {
        self.formatted_name() == other.formatted_name()
    }
}

impl Hash for Ancestry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.formatted_name().hash(state);
    }
}
