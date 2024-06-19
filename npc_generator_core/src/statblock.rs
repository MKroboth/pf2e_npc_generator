use std::fmt::Display;

use crate::{Ability, AgeRange, Ancestry, Heritage, Skill, Trait};

#[derive(Default, Debug)]
pub struct NpcFlavor {
    pub description_line: String,
    pub hair_and_eyes_line: String,
}

impl Display for NpcFlavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.description_line)?;
        writeln!(f, "")?;
        writeln!(f, "{}", self.hair_and_eyes_line)
    }
}

#[derive(Default, Debug)]
pub struct Statblock {
    pub name: String,
    pub class: String,
    pub level: i8,
    pub age: u64,
    pub age_range: AgeRange,
    pub sex: String,
    pub traits: Vec<Trait>,
    pub perception: i16,
    pub skills: Vec<(Skill, i16)>,
    pub abilities: Vec<(Ability, i16)>,
    pub items: Vec<String>,
    //--
    pub armor_class: i16,
    pub fortitude_save: i16,
    pub reflex_save: i16,
    pub will_save: i16,
    pub hit_points: i32,
    //--
    pub land_speed: u16,
    // TODO add other speeds
    pub flavor: NpcFlavor,
    pub ancestry: Option<Ancestry>,
    pub heritage: Option<Heritage>,
}
pub struct PF2eStats(Statblock);

impl Statblock {
    pub fn into_pf2e_stats(self) -> PF2eStats {
        PF2eStats(self)
    }
}

impl PF2eStats {
    fn start_codeblock() -> &'static str {
        "```pf2e-stats"
    }
    fn end_codeblock() -> &'static str {
        "```"
    }
}

impl Display for PF2eStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", Self::start_codeblock())?;
        writeln!(f, "{}", Self::end_codeblock())
    }
}
