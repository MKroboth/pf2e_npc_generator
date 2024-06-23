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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityModifications(pub Vec<AbilityBoost>);

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
