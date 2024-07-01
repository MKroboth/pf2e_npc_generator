use self::formats::Formats;

use super::*;
use log::error;
use rand::distributions::Distribution;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::{rngs, Rng, SeedableRng};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use tokio;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct GeneratorData {
    pub ancestries: WeightMap<Ancestry>,
    pub normal_heritage_weight: f64,
    pub versitile_heritages: WeightMap<Heritage>,
    pub heritages: HashMap<Trait, WeightMap<String>>,
    pub backgrounds: WeightMap<Background>,
    pub names: HashMap<Trait, HashMap<String, WeightMap<String>>>,
    pub archetypes: Vec<Archetype>,
}

pub struct Generator<R: rand::Rng + Send + Sync> {
    random_number_generator: R,
    pub data: Arc<GeneratorData>,
    pub scripts: Arc<GeneratorScripts>,
}
pub struct GeneratorScripts {
    pub default_format_flavor_description_line_script: String,
}

impl<R: rand::Rng + Send + Sync> Generator<R> {
    pub fn new(
        rng: R,
        data: Arc<GeneratorData>,
        scripts: Arc<GeneratorScripts>,
    ) -> Result<Generator<R>, Box<dyn std::error::Error>> {
        Ok(Self {
            random_number_generator: rng,
            data,
            scripts,
        })
    }

    async fn generate_age<'a>(
        &self,
        rng: &mut impl Rng,
        ancestry: &'a Ancestry,
        age_range: Option<&'a AgeRange>,
    ) -> (&'a AgeRange, u64) {
        let age_range = age_range.unwrap_or_else(|| {
            let (values, distribution) = &ancestry.age_range_distribution.split_weights().unwrap();
            values[distribution.sample(rng)]
        });

        let valid_ages = ancestry.age_ranges.get(age_range);
        (age_range, valid_ages.choose(rng).unwrap())
    }

    async fn generate_ancestry(
        &self,
        rng: &mut impl Rng,
        ancestry_weights: Option<&WeightMap<String>>,
    ) -> Ancestry {
        let ancestry = {
            let (values, distribution) = if let Some(ancestry_weights) = ancestry_weights {
                self.data
                    .ancestries
                    .split_weights_with_modifications(|x| ancestry_weights.get(&x.name).copied())
                    .unwrap()
            } else {
                self.data.ancestries.split_weights().unwrap()
            };
            values[distribution.sample(rng)].clone()
        };
        ancestry
    }

    #[tokio::main(flavor = "current_thread")]
    pub async fn generate(&mut self, options: &NpcOptions) -> Statblock {
        let ancestry_weights = options.ancestry_weights.as_ref();

        let ancestry = {
            let mut ancestry_rng =
                rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();
            match options.ancestry.clone() {
                Some(x) => x,
                None => {
                    self.generate_ancestry(&mut ancestry_rng, ancestry_weights)
                        .await
                }
            }
            .clone()
        };

        let heritage = {
            let mut heritage_rng =
                rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();

            match options.heritage.clone() {
                Some(x) => x,
                None => self.generate_heritage(&mut heritage_rng).await,
            }
        };

        let background = {
            let mut background_rng =
                rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();

            match options.background.clone() {
                Some(x) => x,
                None => self.generate_background(&mut background_rng).await,
            }
        };

        let sex = options
            .sex
            .as_ref()
            .map(|x| x.to_string())
            .unwrap_or_else(|| {
                if ancestry.is_asexual {
                    String::new()
                } else {
                    let mut sex_rng =
                        rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();
                    self.generate_sex(&mut sex_rng, &ancestry)
                }
            })
            .to_string();
        let (age_range, age) = {
            let mut age_rng = rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();
            self.generate_age(&mut age_rng, &ancestry, options.age_range.as_ref())
                .await
        };
        let mut traits: HashSet<Trait> = HashSet::new();

        traits.extend(ancestry.traits().into_iter().map(|x| x.clone()));
        traits.extend(heritage.iter().flat_map(|x| Vec::from(x.traits())));
        traits.insert(Trait(ancestry.size.to_string()));
        let traits: Vec<Trait> = traits.into_iter().collect::<Vec<_>>();

        let pre_statblock = Statblock {
            traits: traits.clone(),
            age,
            age_range: *age_range,
            name: {
                let mut names_rng =
                    rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();
                if options.enable_flavor_text {
                    self.generate_name(&traits, &mut names_rng, &ancestry, &self.data.names, &sex)
                        .await
                } else {
                    String::default()
                }
            },
            sex,
            ..Default::default()
        };

        let (background, pre_statblock) = if let Some(ref archetype) = options.archetype {
            let archetype_background = Background {
                name: archetype.name.clone(),
                trainings: Default::default(),
                traits: Default::default(),
            };
            (
                archetype_background,
                Statblock {
                    perception: archetype.perception,
                    land_speed: archetype.speed,
                    skills: archetype
                        .skills
                        .iter()
                        .map(|(x, y)| (x.clone(), y.clone()))
                        .collect::<Vec<_>>(),
                    attributes: archetype.attributes.clone(),
                    items: archetype.items.clone(),
                    armor_class: archetype.armor_class,
                    fortitude_save: archetype.fortitude_save,
                    reflex_save: archetype.reflex_save,
                    will_save: archetype.will_save,
                    hit_points: archetype.hp,
                    level: archetype.level,
                    ..pre_statblock
                },
            )
        } else {
            let mut stats_rng = rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();

            (
                background.clone(),
                generate_stats(
                    &mut stats_rng,
                    &ancestry,
                    heritage.as_ref(),
                    &background,
                    pre_statblock,
                ),
            )
        };
        let mut flavor_rng = rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();

        Statblock {
            ancestry: Some(ancestry.clone()),
            heritage: heritage.clone(),
            flavor: if options.enable_flavor_text {
                self.generate_flavor(
                    &ancestry.formats.clone(),
                    self.scripts.clone(),
                    &mut flavor_rng,
                    &pre_statblock.skills,
                    &pre_statblock.attributes,
                    &pre_statblock.name,
                    None, // class not yet supported
                    pre_statblock.level,
                    pre_statblock.age,
                    pre_statblock.age_range,
                    pre_statblock.perception,
                    pre_statblock.fortitude_save,
                    pre_statblock.reflex_save,
                    pre_statblock.will_save,
                    ancestry,
                    heritage.as_ref(),
                    background.clone(),
                    &pre_statblock.sex,
                )
                .await
            } else {
                Default::default()
            },
            class: background.name,
            ..pre_statblock
        }
    }

    pub async fn generate_heritage(&self, rng: &mut impl Rng) -> Option<Heritage> {
        if rng
            .sample(rand::distributions::Bernoulli::new(self.data.normal_heritage_weight).unwrap())
        {
            // TODO choose normal heritage
            None
        } else {
            let heritage = {
                let (values, distribution) =
                    &self.data.versitile_heritages.split_weights().unwrap();
                values[distribution.sample(rng)].clone()
            };

            Some(heritage)
        }
    }

    pub async fn generate_background(&self, rng: &mut impl Rng) -> Background {
        let background: HashMap<String, Background> = HashMap::from_iter(
            self.data
                .backgrounds
                .keys()
                .into_iter()
                .map(|elem| (elem.name(), elem.clone())),
        );

        // if let Some(ancestry_weights) = ancestry_weights {
        // unimplemented!("Weighting ancestries are not implemented yet")
        // } else {
        let background_vec: Vec<Background> = background
            .values()
            .into_iter()
            .map(Clone::clone)
            .collect::<Vec<_>>();
        let result = background_vec
            .choose(rng)
            .expect("Background list is empty")
            .clone();
        result
        /* } */
    }

    pub async fn generate_flavor(
        &self,
        formats: &Formats,
        generator_scripts: Arc<GeneratorScripts>,
        rng: &mut impl Rng,
        _skills: &[(Skill, i16)],
        _attributes: &AbilityStats,
        name: impl AsRef<str>,
        _class: Option<&str>,
        _level: i8,
        age: u64,
        age_range: AgeRange,
        _perception: i16,
        _fortitude_save: i16,
        _reflex_save: i16,
        _will_save: i16,
        ancestry: Ancestry,
        heritage: Option<&Heritage>,
        background: Background,
        sex: impl AsRef<str>,
    ) -> NpcFlavor {
        NpcFlavor {
            description_line: generate_flavor_description_line(
                generator_scripts,
                formats,
                name,
                age,
                age_range,
                sex,
                &ancestry.name,
                heritage.map(NamedElement::name).as_deref(),
                background.name,
                None,
            )
            .await,
            lineage_line: generate_lineage_line(heritage, formats).await,
            hair_and_eyes_line: generate_flavor_hair_and_eyes_line(
                rng, formats, &ancestry, heritage,
            ),
            skin_line: generate_flavor_skin_line(rng, formats, &ancestry, heritage),
            size_and_build_line: generate_size_and_build(
                rng, formats, &ancestry, age, age_range, heritage,
            ),
            face_line: generate_flavor_face_line(rng, formats, &ancestry),
            habit_line: generate_flavor_habit_line(rng, formats, &ancestry),
        }
    }

    fn generate_sex(&self, random_number_generator: &mut impl Rng, _ancestry: &Ancestry) -> String {
        let sexes = vec!["male", "female"]; // TODO add diversity
        String::from_str(sexes.choose(random_number_generator).unwrap()).unwrap()
    }

    async fn generate_name(
        &self,
        traits: &[Trait],
        name_rng: &mut impl Rng,
        ancestry: &Ancestry,
        names: &HashMap<Trait, HashMap<String, WeightMap<String>>>,
        sex: &str,
    ) -> String {
        let traits: Vec<Trait> = {
            let filtered_traits: HashSet<_> = HashSet::from_iter(traits);
            let available_name_traits: HashSet<_> = HashSet::from_iter(names.keys());
            filtered_traits
                .into_iter()
                .filter(|x| available_name_traits.contains(x))
                .map(|x| x.clone())
                .collect::<Vec<Trait>>()
        };

        let name_trait = traits
            .choose(name_rng)
            .expect("We have no traits to get our name from");

        let names = if let Some(name) = names.get(name_trait).unwrap().get(sex) {
            name
        } else {
            error!(
                "No names for given sex `{sex}` present on name trait `{}`",
                name_trait
            );
            return String::from("@@NAME_ERROR@@");
        };
        let first_name = {
            let (names, weights) = names.split_weights().unwrap();
            names[name_rng.sample(weights)].clone()
        };

        let surname = if let Some(ref surnames) = ancestry.specimen_surnames {
            let (surnames, weights) = surnames.split_weights().unwrap();
            surnames[name_rng.sample(weights)].clone()
        } else {
            return first_name;
        };

        ancestry
            .formats
            .format_full_name(&first_name, &surname, vec![])
            .await
    }
}

fn generate_size_and_build(
    _rng: &mut impl Rng,
    _formats: &Formats,
    _ancestry: &Ancestry,
    _age: u64,
    _age_range: AgeRange,
    _heritage: Option<&Heritage>,
) -> String {
    format!("")
}

fn generate_flavor_face_line(
    _rng: &mut impl Rng,
    _formats: &Formats,
    _ancestry: &Ancestry,
) -> String {
    format!("They have a face.")
}
fn generate_flavor_habit_line(
    _rng: &mut impl Rng,
    _formats: &Formats,
    _ancestry: &Ancestry,
) -> String {
    format!("")
}
fn generate_stats(
    stats_rng: &mut rand::prelude::StdRng,
    ancestry: &Ancestry,
    _heritage: Option<&Heritage>,
    background: &Background,
    pre_statblock: Statblock,
) -> Statblock {
    let level = pre_statblock.level;
    let mut attributes = AbilityStats::default();

    let mut choosen_this_round = HashSet::new();
    for amod in ancestry.ability_modifications.0.iter() {
        match amod {
            AbilityBoost::Boost(ability) => {
                *attributes.get_ability_mut(*ability) += 1;
                choosen_this_round.insert(ability);
            }
            AbilityBoost::Flaw(ability) => {
                *attributes.get_ability_mut(*ability) -= 1;
                choosen_this_round.insert(ability);
            }
            AbilityBoost::Free => {
                let mut ability = Ability::values().choose(stats_rng).unwrap();
                while choosen_this_round.contains(ability) {
                    ability = Ability::values().choose(stats_rng).unwrap();
                }

                *attributes.get_ability_mut(*ability) += 1;
            }
        }
    }
    choosen_this_round.clear();
    for _ in 0..4 {
        let mut ability = Ability::values().choose(stats_rng).unwrap();
        while choosen_this_round.contains(ability) {
            ability = Ability::values().choose(stats_rng).unwrap();
        }

        *attributes.get_ability_mut(*ability) += 1;
    }

    choosen_this_round.clear();
    for _ in 0..4 {
        let mut ability = Ability::values().choose(stats_rng).unwrap();
        while choosen_this_round.contains(ability) {
            ability = Ability::values().choose(stats_rng).unwrap();
        }

        *attributes.get_ability_mut(*ability) += 1;
    }

    // calculate initial proficiencies
    let proficiencies = {
        let mut proficiencies = Proficiencies::default();

        let excluded_skills = {
            let mut excluded_skills = HashSet::new();
            for training in background.trainings.iter() {
                proficiencies
                    .skills
                    .insert(training.clone(), Proficiency::Trained);
                excluded_skills.insert(training.clone());
            }
            excluded_skills
        };

        // calculate additional skills for intelligence
        let selected_skills = {
            let additional_skills = 2 + attributes.intelligence;
            let mut selected_skills: HashSet<Skill> = HashSet::new();

            let mut loop_guard = 10_000;
            while selected_skills.len() != additional_skills as usize {
                let choosen_skill = Skill::values_excluding_lore().choose(stats_rng).unwrap();
                if !excluded_skills.contains(&choosen_skill) {
                    selected_skills.insert(choosen_skill.clone());
                }
                loop_guard -= 1;
                if loop_guard == 0 {
                    break;
                }
            }
            selected_skills
        };

        proficiencies.skills.extend(
            selected_skills
                .into_iter()
                .map(|x| (x, Proficiency::Trained)),
        );

        proficiencies.perception = Proficiency::Trained;
        proficiencies.fortitude_save = Proficiency::Trained;
        proficiencies.will_save = Proficiency::Trained;
        proficiencies.reflex_save = Proficiency::Trained;
        proficiencies.unarmed = Proficiency::Trained;
        proficiencies.unarmored_defense = Proficiency::Trained;
        proficiencies
    };

    let skills = {
        let mut skills = Vec::new();

        for (skill, proficiency) in proficiencies.skills.iter() {
            let modifier: i16 = match skill {
                Skill::Acrobatics => attributes.dexterity,
                Skill::Arcana => attributes.intelligence,
                Skill::Athletics => attributes.strength,
                Skill::Crafting => attributes.intelligence,
                Skill::Deception => attributes.charisma,
                Skill::Diplomacy => attributes.charisma,
                Skill::Intimidation => attributes.charisma,
                Skill::Lore(_) => attributes.intelligence,
                Skill::Medicine => attributes.wisdom,
                Skill::Nature => attributes.wisdom,
                Skill::Occultism => attributes.intelligence,
                Skill::Performance => attributes.charisma,
                Skill::Religion => attributes.wisdom,
                Skill::Society => attributes.intelligence,
                Skill::Stealth => attributes.dexterity,
                Skill::Survival => attributes.wisdom,
                Skill::Thievery => attributes.dexterity,
            } as i16;
            let proficiency_bonus = proficiency.bonus_for_level(level) as i16;

            skills.push((skill.clone(), modifier + proficiency_bonus));
        }
        skills
    };

    let hit_points: i32 = (ancestry.base_hp as i32) + (attributes.constitution as i32);

    Statblock {
        perception: (attributes.wisdom + proficiencies.perception.bonus_for_level(level)) as i16,
        fortitude_save: (attributes.constitution
            + proficiencies.fortitude_save.bonus_for_level(level)) as i16,
        reflex_save: (attributes.dexterity + proficiencies.fortitude_save.bonus_for_level(level))
            as i16,
        will_save: (attributes.wisdom + proficiencies.fortitude_save.bonus_for_level(level)) as i16,
        armor_class: (attributes.dexterity + proficiencies.unarmored_defense.bonus_for_level(level))
            as i16,
        land_speed: ancestry.speed,
        skills,
        attributes,
        proficiencies,
        hit_points,
        ..pre_statblock
    }
}

async fn generate_lineage_line(heritage: Option<&Heritage>, formats: &Formats) -> Option<String> {
    if let Some(heritage) = heritage {
        match heritage
            .lineage
            .as_ref()
            .map(|lineage| heritage.formats.format_lineage_line(lineage))
        {
            Some(x) => Some(x.await),
            None => None,
        }
    } else {
        None
    }
}

async fn generate_flavor_description_line(
    generator_scripts: Arc<GeneratorScripts>,
    formats: &Formats,
    name: impl AsRef<str>,
    age: u64,
    age_range: AgeRange,
    sex: impl AsRef<str>,
    ancestry_name: impl AsRef<str>,
    heritage_name: Option<&str>,
    background_name: impl AsRef<str>,
    class_name: Option<&str>,
) -> String {
    let name = name.as_ref();
    let sex = sex.as_ref();
    let ancestry_name = ancestry_name.as_ref();
    let background_name = background_name.as_ref();
    let job_name = if class_name.is_some() {
        class_name.unwrap()
    } else {
        background_name
    };

    let heritage_name = heritage_name
        .map(|x| " ".to_owned() + x)
        .unwrap_or("".to_string());

    let sex = if sex.is_empty() {
        sex.to_string()
    } else {
        format!(" {sex}")
    };

    formats
        .format_flavor_description_line(
            &generator_scripts.default_format_flavor_description_line_script,
            name,
            age,
            age_range,
            &sex,
            ancestry_name,
            &heritage_name,
            job_name,
        )
        .await
    // match age_range {
    //     AgeRange::Infant => {
    //         if age == 0 {
    //             format!("{name} is a{sex} {ancestry_name}{heritage_name} newborn.")
    //         } else {
    //             format!("{name} is a {age} year old{sex} {ancestry_name}{heritage_name} infant.")
    //         }
    //     }
    //     AgeRange::Child => {
    //         format!(
    //             "{name} is a {age} year old{sex} {ancestry_name}{heritage_name} child {job_name}."
    //         )
    //     }
    //     AgeRange::Youth => {
    //         format!("{name} is a {age} year old{sex} {ancestry_name}{heritage_name} {job_name} in their youths.")
    //     }
    //     AgeRange::Adult => {
    //         format!("{name} is an adult, {age} year old{sex} {ancestry_name}{heritage_name} {job_name}.")
    //     }
    //     AgeRange::MiddleAged => {
    //         format!("{name} is a middle-aged, {age} year old{sex} {ancestry_name}{heritage_name} {job_name}.")
    //     }
    //     AgeRange::Old => {
    //         format!(
    //             "{name} is an old, {age} year old{sex} {ancestry_name}{heritage_name} {job_name}."
    //         )
    //     }
    //     AgeRange::Venerable => {
    //         format!("{name} is a venerable, {age} year old{sex} {ancestry_name}{heritage_name} {job_name}.")
    //     }
    // }
}

fn generate_flavor_hairs(
    rng: &mut impl Rng,
    formats: &Formats,
    ancestry: &Ancestry,
    _heritage: Option<&Heritage>,
) -> String {
    let (possible_hair_colors, possible_hair_type, possible_hair_length) =
        if ancestry.possible_hair_type.is_some()
            && ancestry.possible_hair_colors.is_some()
            && ancestry.possible_hair_length.is_some()
        {
            (
                ancestry.possible_hair_colors.clone().unwrap(),
                ancestry.possible_hair_type.clone().unwrap(),
                ancestry.possible_hair_length.clone().unwrap(),
            )
        } else {
            return "no hair".to_string();
        };

    let hair_color: String = {
        let (values, distribution) = possible_hair_colors.split_weights().unwrap();
        (values[distribution.sample(rng)]).into()
    };
    let hair_type: String = {
        let (values, distribution) = possible_hair_type.split_weights().unwrap();
        (values[distribution.sample(rng)]).into()
    };
    let hair_length: String = {
        let (values, distribution) = possible_hair_length.split_weights().unwrap();
        (values[distribution.sample(rng)]).into()
    };

    let hair = &ancestry.hair_substance;
    format!("{hair_length}, {hair_type}, {hair_color} {hair}")
}

fn generate_flavor_eyes(
    rng: &mut impl Rng,
    formats: &Formats,
    ancestry: &Ancestry,
    heritage: Option<&Heritage>,
) -> String {
    let mut available_eye_colors: WeightMap<String> = WeightMap::new();
    if let Some(x) = &ancestry.possible_eye_colors {
        available_eye_colors.extend(x.clone());
    } else {
        return "no eyes".into();
    };

    if let Some(heritage) = heritage {
        let eye_colors = heritage.additional_eye_colors.clone();
        available_eye_colors.extend(eye_colors);
    }

    let (available_eye_colors, distribution) = available_eye_colors.split_weights().unwrap();

    let eye_color: &str = available_eye_colors[distribution.sample(rng)];
    let heterochromia_color: &str = available_eye_colors[distribution.sample(rng)];
    let force_heterochromia = if let Some(heritage) = heritage {
        heritage.force_heterochromia.clone()
    } else {
        None
    };
    let (has_heterochromia, heterochromia_color): (bool, &str) =
        if let Some(color) = &force_heterochromia {
            (true, color)
        } else {
            let dist = rand::distributions::Bernoulli::new(
                ancestry.mutation_probabilities[&Mutation::Heterochromia],
            )
            .unwrap();
            (dist.sample(rng), heterochromia_color)
        };

    if has_heterochromia {
        let mut eye_color: &str = eye_color;
        while heterochromia_color == eye_color {
            eye_color = available_eye_colors[distribution.sample(rng)];
        }
        let is_left_hetero = rng.sample(rand::distributions::Bernoulli::new(0.5).unwrap());
        let (left_eye, right_eye) = if is_left_hetero {
            (heterochromia_color, eye_color)
        } else {
            (eye_color, heterochromia_color)
        };
        format!(
            "heterochromatic eyes.\nTheir left eye is {left_eye} and their right eye is {right_eye}"
        )
        .into()
    } else {
        format!("{eye_color} eyes").into()
    }
}

fn generate_flavor_hair_and_eyes_line(
    mut rng: &mut impl Rng,
    formats: &Formats,
    ancestry: &Ancestry,
    heritage: Option<&Heritage>,
) -> String {
    format!(
        "They have {} and {}.",
        generate_flavor_hairs(&mut rng, formats, &ancestry, heritage),
        generate_flavor_eyes(&mut rng, formats, ancestry, heritage)
    )
    .into()
}
fn generate_flavor_skin_line(
    mut rng: &mut impl Rng,
    formats: &Formats,
    ancestry: &Ancestry,
    _heritage: Option<&Heritage>,
) -> String {
    let skin_texture = {
        let (skin_textures, distribution) = ancestry.possible_skin_texture.split_weights().unwrap();
        skin_textures[rng.sample(distribution)]
    };

    let skin_tone = {
        let (skin_tones, distribution) = ancestry.possible_skin_tone.split_weights().unwrap();
        skin_tones[rng.sample(distribution)]
    };
    let skin = &ancestry.skin_substance;
    format!("They have {skin_texture} {skin_tone} {skin}.")
}
