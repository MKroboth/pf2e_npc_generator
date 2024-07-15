use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
    sync::Arc,
};

use serde::{Deserialize, Serialize};

use crate::{formats::HeritageFormats, NamedElement, Trait, ValidAncestries, WeightMap};

type HeritageString = Arc<str>;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Heritage {
    traits: Vec<Trait>,
    name: HeritageString,
    lineage: Option<HeritageString>,
    prd_reference: Option<HeritageString>,
    valid_ancestries: ValidAncestries,

    additional_eye_colors: WeightMap<HeritageString>,
    additional_hair_colors: WeightMap<HeritageString>,
    force_heterochromia: Option<HeritageString>,
    #[serde(default)]
    formats: HeritageFormats,
}

impl Heritage {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        traits: impl Into<Vec<Trait>>,
        name: impl AsRef<str>,
        lineage: Option<impl AsRef<str>>,
        valid_ancestries: ValidAncestries,
        additional_eye_colors: impl Into<WeightMap<HeritageString>>,
        additional_hair_colors: impl Into<WeightMap<HeritageString>>,
        force_heterochromia: Option<impl AsRef<str>>,
        prd_reference: Option<impl AsRef<str>>,
        formats: HeritageFormats,
    ) -> Self {
        Self {
            traits: traits.into(),
            name: name.as_ref().into(),
            lineage: lineage.map(|x| x.as_ref().into()),
            valid_ancestries,
            additional_eye_colors: additional_eye_colors.into(),
            additional_hair_colors: additional_hair_colors.into(),
            prd_reference: prd_reference.map(|x| x.as_ref().into()),
            force_heterochromia: force_heterochromia.map(|x| x.as_ref().into()),
            formats,
        }
    }

    pub fn lineage(&self) -> Option<&str> {
        self.lineage.as_deref()
    }

    pub fn prd_reference(&self) -> Option<&str> {
        self.prd_reference.as_deref()
    }

    pub fn valid_ancestries(&self) -> &ValidAncestries {
        &self.valid_ancestries
    }

    pub fn additional_eye_colors(&self) -> &WeightMap<impl AsRef<str> + Eq + Hash> {
        &self.additional_eye_colors
    }

    pub fn additional_hair_colors(&self) -> &WeightMap<impl AsRef<str> + Eq + Hash> {
        &self.additional_hair_colors
    }

    pub fn force_heterochromia(&self) -> Option<&str> {
        self.force_heterochromia.as_deref()
    }

    pub fn formats(&self) -> &HeritageFormats {
        &self.formats
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
    fn name(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }
    fn formatted_name(&self) -> Cow<str> {
        if let Some(ref lineage) = self.lineage {
            Cow::Owned(format!("{} ({})", self.name, lineage))
        } else {
            Cow::Borrowed(&self.name)
        }
    }
}
