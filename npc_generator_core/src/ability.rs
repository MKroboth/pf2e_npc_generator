use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum AbilityBoost {
    Boost(Ability),
    Flaw(Ability),
    Free,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Ability {
    Charisma,
    Constitution,
    Dexterity,
    Intelligence,
    Strength,
    Wisdom,
}

impl Ability {
    pub fn values() -> &'static [Ability] {
        static VALUES: [Ability; 6] = [
            Ability::Charisma,
            Ability::Constitution,
            Ability::Dexterity,
            Ability::Intelligence,
            Ability::Strength,
            Ability::Wisdom,
        ];
        &VALUES
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AbilityModifications(Vec<AbilityBoost>);

impl From<Vec<AbilityBoost>> for AbilityModifications {
    fn from(value: Vec<AbilityBoost>) -> Self {
        Self(value)
    }
}

impl AsRef<[AbilityBoost]> for AbilityModifications {
    fn as_ref(&self) -> &[AbilityBoost] {
        &self.0
    }
}

impl IntoIterator for AbilityModifications {
    type Item = AbilityBoost;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl AbilityModifications {
    pub fn new(value: impl Into<Vec<AbilityBoost>>) -> Self {
        Self(value.into())
    }

    pub fn iter(&self) -> core::slice::Iter<AbilityBoost> {
        self.0.iter()
    }
}
