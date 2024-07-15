use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::{borrow::Cow, sync::Arc};

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
        if self.0 < 4 || self.1 {
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

type StatblockString = Arc<str>;
#[derive(Debug, Clone)]
pub struct Statblock {
    name: StatblockString,
    class: StatblockString,
    level: i8,
    age: u64,
    age_range: AgeRange,
    sex: StatblockString,
    traits: Arc<[Trait]>,
    perception: i16,
    skills: Arc<[(Skill, i16)]>,
    attributes: AbilityStats,
    items: Arc<[StatblockString]>,
    //--
    armor_class: i16,
    fortitude_save: i16,
    reflex_save: i16,
    will_save: i16,
    hit_points: i32,
    //--
    land_speed: u16,
    // TODO add other speeds
    flavor: NpcFlavor,
    ancestry: Option<Ancestry>,
    heritage: Option<Heritage>,
    proficiencies: Proficiencies,
}
pub struct PF2eStats(Statblock);

impl Statblock {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: impl AsRef<str>,
        class: impl AsRef<str>,
        level: i8,
        age: u64,
        age_range: AgeRange,
        sex: impl AsRef<str>,
        traits: impl Into<Vec<Trait>>,
        perception: i16,
        skills: impl Into<Vec<(Skill, i16)>>,
        attributes: AbilityStats,
        items: impl Into<Vec<String>>,
        armor_class: i16,
        fortitude_save: i16,
        reflex_save: i16,
        will_save: i16,
        hit_points: i32,
        land_speed: u16,
        flavor: NpcFlavor,
        ancestry: Option<Ancestry>,
        heritage: Option<Heritage>,
        proficiencies: Proficiencies,
    ) -> Self {
        Self {
            name: name.as_ref().into(),
            class: class.as_ref().into(),
            level,
            age,
            age_range,
            sex: sex.as_ref().into(),
            traits: traits.into().into(),
            perception,
            skills: skills.into().into(),
            attributes,
            items: Vec::from_iter(items.into().into_iter().map(|x| x.into())).into(),
            armor_class,
            fortitude_save,
            reflex_save,
            will_save,
            hit_points,
            land_speed,
            flavor,
            ancestry,
            heritage,
            proficiencies,
        }
    }

    pub fn into_pf2e_stats(self) -> PF2eStats {
        PF2eStats(self)
    }
    pub fn as_pf2e_stats(&self) -> PF2eStats {
        PF2eStats(self.clone())
    }

    pub fn name(&self) -> std::borrow::Cow<str> {
        Cow::Borrowed(&self.name)
    }

    pub fn class(&self) -> &str {
        &self.class
    }

    pub fn skills(&self) -> &[(Skill, i16)] {
        &self.skills
    }

    pub fn attributes(&self) -> &AbilityStats {
        &self.attributes
    }

    pub fn traits(&self) -> &[Trait] {
        &self.traits
    }
    pub fn level(&self) -> i8 {
        self.level
    }

    pub fn age(&self) -> u64 {
        self.age
    }

    pub fn age_range(&self) -> AgeRange {
        self.age_range
    }

    pub fn perception(&self) -> i16 {
        self.perception
    }

    pub fn fortitude_save(&self) -> i16 {
        self.fortitude_save
    }

    pub fn reflex_save(&self) -> i16 {
        self.reflex_save
    }
    pub fn will_save(&self) -> i16 {
        self.will_save
    }

    pub fn sex(&self) -> &str {
        &self.sex
    }

    pub fn set_name(&mut self, name: impl AsRef<str>) {
        self.name = name.as_ref().into();
    }

    pub fn set_class(&mut self, class: impl AsRef<str>) {
        self.class = class.as_ref().into();
    }

    pub fn set_level(&mut self, level: i8) {
        self.level = level;
    }

    pub fn set_age(&mut self, age: u64) {
        self.age = age;
    }

    pub fn set_age_range(&mut self, age_range: AgeRange) {
        self.age_range = age_range;
    }

    pub fn set_sex(&mut self, sex: impl AsRef<str>) {
        self.sex = sex.as_ref().into();
    }

    pub fn set_traits(&mut self, traits: Vec<Trait>) {
        self.traits = traits.into();
    }

    pub fn set_perception(&mut self, perception: i16) {
        self.perception = perception;
    }

    pub fn set_skills(&mut self, skills: Vec<(Skill, i16)>) {
        self.skills = skills.into();
    }

    pub fn set_attributes(&mut self, attributes: impl Into<AbilityStats>) {
        self.attributes = attributes.into();
    }

    pub fn set_items(&mut self, items: impl Iterator<Item = impl AsRef<str>>) {
        let mut new_items: Vec<Arc<str>> = Vec::new();
        for item in items {
            new_items.push(item.as_ref().into())
        }
        self.items.clone_from(&new_items.into());
    }

    pub fn set_armor_class(&mut self, armor_class: i16) {
        self.armor_class = armor_class;
    }

    pub fn set_fortitude_save(&mut self, fortitude_save: i16) {
        self.fortitude_save = fortitude_save;
    }

    pub fn set_reflex_save(&mut self, reflex_save: i16) {
        self.reflex_save = reflex_save;
    }

    pub fn set_will_save(&mut self, will_save: i16) {
        self.will_save = will_save;
    }

    pub fn set_hit_points(&mut self, hit_points: i32) {
        self.hit_points = hit_points;
    }

    pub fn set_land_speed(&mut self, land_speed: u16) {
        self.land_speed = land_speed;
    }

    pub fn set_flavor(&mut self, flavor: NpcFlavor) {
        self.flavor = flavor;
    }

    pub fn set_ancestry(&mut self, ancestry: Option<impl Into<Ancestry>>) {
        self.ancestry = ancestry.map(Into::into);
    }

    pub fn set_heritage(&mut self, heritage: Option<impl Into<Heritage>>) {
        self.heritage = heritage.map(Into::into);
    }

    pub fn set_proficiencies(&mut self, proficiencies: Proficiencies) {
        self.proficiencies = proficiencies;
    }

    pub fn flavor(&self) -> &NpcFlavor {
        &self.flavor
    }

    pub fn ancestry(&self) -> Option<&Ancestry> {
        self.ancestry.as_ref()
    }

    pub fn heritage(&self) -> Option<&Heritage> {
        self.heritage.as_ref()
    }
}

impl Default for Statblock {
    fn default() -> Self {
        Self {
            name: "".into(),
            class: "".into(),
            level: Default::default(),
            age: Default::default(),
            age_range: Default::default(),
            sex: "".into(),
            traits: Vec::default().into(),
            perception: Default::default(),
            skills: Vec::default().into(),
            attributes: Default::default(),
            items: Vec::default().into(),
            //--
            armor_class: Default::default(),
            fortitude_save: Default::default(),
            reflex_save: Default::default(),
            will_save: Default::default(),
            hit_points: Default::default(),
            //--
            land_speed: Default::default(),
            // TODO add other speeds
            flavor: Default::default(),
            ancestry: Default::default(),
            heritage: Default::default(),
            proficiencies: Default::default(),
        }
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
        format!("# {}", self.0.name())
    }
    fn creature_type_level(&self) -> String {
        format!("## {} {}", self.0.class(), self.0.level())
    }
    fn traits(&self) -> String {
        let mut trait_string = String::new();
        trait_string.push_str("==Unique== ");
        let mut traits = Vec::from_iter(self.0.traits().iter());
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

mod tests {
    /**
      name: StatblockString,
        class: StatblockString,
        level: i8,
        age: u64,
        age_range: AgeRange,
        sex: StatblockString,
        traits: Arc<[Trait]>,
        perception: i16,
        skills: Arc<[(Skill, i16)]>,
        attributes: AbilityStats,
        items: Arc<[StatblockString]>,
        //--
        armor_class: i16,
        fortitude_save: i16,
        reflex_save: i16,
        will_save: i16,
        hit_points: i32,
        //--
        land_speed: u16,
        // TODO add other speeds
        flavor: NpcFlavor,
        ancestry: Option<Ancestry>,
        heritage: Option<Heritage>,
        proficiencies: Proficiencies,
    */
    #[test]
    fn test_name_accessor() {
        let mut statblock = super::Statblock::default();
        assert_eq!("", statblock.name());
        statblock.set_name("new value");
        assert_eq!("new value", statblock.name());
    }
    #[test]
    fn test_traits_accessor() {
        let mut statblock = super::Statblock::default();
        assert!(statblock.traits().is_empty());
        statblock.set_traits(vec![
            crate::Trait::new("1"),
            crate::Trait::new("2"),
            crate::Trait::new("3"),
        ]);
        let traits = statblock.traits();
        assert!(traits.len() == 3);
        assert_eq!(crate::Trait::new("1"), traits[0]);
        assert_eq!(crate::Trait::new("2"), traits[1]);
        assert_eq!(crate::Trait::new("3"), traits[2]);
    }
}
