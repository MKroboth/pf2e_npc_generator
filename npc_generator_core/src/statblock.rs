use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{Ability, AgeRange, Ancestry, Heritage, Proficiencies, Skill, Trait};

#[derive(Default, Debug, Clone)]
pub struct NpcFlavor {
    pub description_line: String,
    pub hair_and_eyes_line: String,
    pub skin_line: String,
    pub lineage_line: Option<String>,
    pub size_and_build_line: String,
    pub face_line: String,
    pub habit_line: String,
}

impl Display for NpcFlavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\n", self.description_line)?;
        writeln!(f, "{}\n", self.hair_and_eyes_line)?;
        writeln!(f, "{}\n", self.skin_line)?;
        if let Some(ref lineage_line) = self.lineage_line {
            writeln!(f, "{}\n", lineage_line)?;
        }

        writeln!(f, "{}\n", self.size_and_build_line)?;
        writeln!(f, "{}\n", self.face_line)?;
        writeln!(f, "{}\n", self.habit_line)?;
        Ok(())
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Copy, Clone)]
pub struct AbilityValue(i8, bool);

impl AbilityValue {
    pub fn new(value: i8) -> Self {
        Self(value, false)
    }

    pub fn boost(self) -> Self {
        if self.0 < 4 {
            Self(self.0 + 1, false)
        } else if self.1 {
            Self(self.0 + 1, false)
        } else {
            Self(self.0, true)
        }
    }

    pub fn flaw(self) -> Self {
        Self(self.0 - 1, false)
    }
}

impl Display for AbilityValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:+}{}", self.0, if self.1 { "*" } else { "" })
    }
}

impl From<AbilityValue> for i8 {
    fn from(value: AbilityValue) -> Self {
        value.0
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct AbilityStats {
    pub strength: i8,
    pub dexterity: i8,
    pub constitution: i8,
    pub intelligence: i8,
    pub wisdom: i8,
    pub charisma: i8,
}

impl AbilityStats {
    pub fn get_ability_mut(&mut self, ability: Ability) -> &mut i8 {
        match ability {
            Ability::Charisma => &mut self.charisma,
            Ability::Constitution => &mut self.constitution,
            Ability::Dexterity => &mut self.dexterity,
            Ability::Intelligence => &mut self.intelligence,
            Ability::Strength => &mut self.strength,
            Ability::Wisdom => &mut self.wisdom,
        }
    }
}

#[derive(Default, Debug, Clone)]
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
    pub attributes: AbilityStats,
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
    pub proficiencies: Proficiencies,
}
pub struct PF2eStats(Statblock);

impl Statblock {
    pub fn into_pf2e_stats(self) -> PF2eStats {
        PF2eStats(self)
    }
    pub fn as_pf2e_stats(&self) -> PF2eStats {
        PF2eStats(self.clone())
    }
}

impl PF2eStats {
    fn start_codeblock() -> &'static str {
        "```pf2e-stats"
    }
    fn end_codeblock() -> &'static str {
        "```"
    }
    fn creature_name(&self) -> String {
        format!("# {}", self.0.name)
    }
    fn creature_type_level(&self) -> String {
        format!("## {} {}", self.0.class, self.0.level)
    }
    fn traits(&self) -> String {
        let mut trait_string = String::new();
        trait_string.push_str("==Unique== ");
        let mut traits = self.0.traits.clone();
        traits.sort_by_key(|x| x.to_string());
        for trait_value in traits.iter().map(|x| format!("=={}==", x)) {
            trait_string.push_str(&trait_value);
            trait_string.push(' ');
        }
        let _ = trait_string.pop();
        trait_string
    }
    fn languages(&self) -> String {
        let mut languages = String::new();
        languages.push_str("Common"); // TODO add languages
        languages
    }

    fn skills(&self) -> String {
        let mut skills = self
            .0
            .skills
            .iter()
            .filter(|x| x.1 != 0)
            .collect::<Vec<_>>();
        skills.sort_by_key(|x| x.0.to_string());
        let mut skills_string = String::new();
        for skill in skills.iter().map(|x| format!("{} {:+},", x.0, x.1)) {
            skills_string.push(' ');
            skills_string.push_str(&skill);
        }
        let _ = skills_string.pop();
        skills_string
    }

    fn attributes(&self) -> String {
        let AbilityStats {
            strength,
            dexterity,
            constitution,
            intelligence,
            wisdom,
            charisma,
        } = self.0.attributes;
        format!("**Str** {strength:+}, **Dex** {dexterity:+}, **Con** {constitution:+}, **Int** {intelligence:+}, **Wis** {wisdom:+}, **Cha** {charisma:+}")
    }

    fn ac_and_saves(&self) -> String {
        format!(
            "**AC** {}; **Fort** {:+}, **Ref** {:+}, **Will** {:+}",
            self.0.armor_class, self.0.fortitude_save, self.0.reflex_save, self.0.will_save
        )
    }

    fn hp(&self) -> String {
        format!("**HP** {}", self.0.hit_points)
    }

    fn speed(&self) -> String {
        format!("**Speed** {}", self.0.land_speed)
    }
}

impl Display for PF2eStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", Self::start_codeblock())?;
        writeln!(f, "{}", self.creature_name())?;
        writeln!(f, "{}", self.creature_type_level())?;
        writeln!(f, "{}\n", self.traits())?;
        writeln!(f, "**Perception** {:+}", self.0.perception)?;
        writeln!(f, "**Languages** {}", self.languages())?;
        writeln!(f, "**Skills**{}", self.skills())?;
        writeln!(f, "{}", self.attributes())?;
        writeln!(f, "\n---\n")?;
        writeln!(f, "{}", self.ac_and_saves())?;
        writeln!(f, "{}", self.hp())?;
        writeln!(f, "\n---\n")?;
        writeln!(f, "{}", self.speed())?;
        writeln!(f, "\n---\n")?;
        writeln!(f, "{}", self.0.flavor)?;
        writeln!(f, "{}", Self::end_codeblock())
    }
}
