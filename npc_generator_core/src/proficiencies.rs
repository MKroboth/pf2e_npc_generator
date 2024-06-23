use crate::Skill;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Proficiency {
    Untrained,
    Trained,
    Expert,
    Master,
    Legendary,
}

impl Proficiency {
    pub fn bonus_for_level(&self, level: i8) -> i8 {
        if let Self::Untrained = self {
            return 0;
        }

        level as i8
            + match self {
                Proficiency::Untrained => unreachable!(),
                Proficiency::Trained => 2,
                Proficiency::Expert => 4,
                Proficiency::Master => 6,
                Proficiency::Legendary => 8,
            }
    }
}

impl Default for Proficiency {
    fn default() -> Self {
        Proficiency::Untrained
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Proficiencies {
    pub perception: Proficiency,
    pub fortitude_save: Proficiency,
    pub reflex_save: Proficiency,
    pub will_save: Proficiency,
    pub unarmed: Proficiency,
    pub simple_weapons: Proficiency,
    pub martial_weapons: Proficiency,
    pub advanced_weapons: Proficiency,
    pub unarmored_defense: Proficiency,
    pub light_armor: Proficiency,
    pub medium_armor: Proficiency,
    pub heavy_armor: Proficiency,
    pub skills: HashMap<Skill, Proficiency>,
}
