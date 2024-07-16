use std::{fmt::Display, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize, Hash)]
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

impl Skill {
    pub fn values_excluding_lore() -> &'static [Skill] {
        static SKILLS_EXCLUDING_LORE: [Skill; 16] = [
            Skill::Acrobatics,
            Skill::Arcana,
            Skill::Athletics,
            Skill::Crafting,
            Skill::Deception,
            Skill::Diplomacy,
            Skill::Intimidation,
            Skill::Medicine,
            Skill::Nature,
            Skill::Occultism,
            Skill::Performance,
            Skill::Religion,
            Skill::Society,
            Skill::Stealth,
            Skill::Survival,
            Skill::Thievery,
        ];
        &SKILLS_EXCLUDING_LORE
    }
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
