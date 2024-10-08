use self::formats::Formats;
use self::weight_presets::WeightPreset;

use super::*;
use log::{debug, error, info};
use rand::distributions::Distribution;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::{rngs, Rng, SeedableRng};
use std::borrow::Cow;
use std::collections::{HashMap, LinkedList};
use std::sync::Arc;
use thiserror::Error;
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

#[derive(Error, Debug)]
pub enum RngError {
    #[error("Weighted Error")]
    WeightedError(#[from] rand::distributions::WeightedError),
}

#[derive(Error, Debug)]
pub enum GenerationError {
    #[error(transparent)]
    AgeGenerationError(#[from] AgeGenerationError),
    #[error(transparent)]
    AncestryGenerationError(#[from] AncestryGenerationError),
    #[error(transparent)]
    AbilityGenerationError(#[from] AbilityGenerationError),
    #[error(transparent)]
    SkillGenerationError(#[from] SkillGenerationError),
    #[error(transparent)]
    FlavorGenerationError(#[from] FlavorGenerationError),
    #[error(transparent)]
    HeritageGenerationError(#[from] HeritageGenerationError),
    #[error(transparent)]
    BackgroundGenerationError(#[from] BackgroundGenerationError),
    #[error(transparent)]
    SexGenerationError(#[from] SexGenerationError),
}

#[derive(Error, Debug)]
#[error("unable to generate random age")]
pub struct AgeGenerationError;

#[derive(Error, Debug)]
#[error("unable to generate random ancestry")]
pub struct AncestryGenerationError;

#[derive(Error, Debug)]
#[error("unable to generate random abilities")]
pub struct AbilityGenerationError;

#[derive(Error, Debug)]
#[error("unable to generate skills")]
pub struct SkillGenerationError;

#[derive(Error, Debug)]
#[error("unable to generate hair {message}")]
pub struct HairGenerationError {
    pub message: String,
}

impl<T: AsRef<str>> From<T> for HairGenerationError {
    fn from(value: T) -> Self {
        Self {
            message: value.as_ref().into(),
        }
    }
}

#[derive(Error, Debug)]
#[error("unable to generate heritage")]
pub struct HeritageGenerationError;

#[derive(Error, Debug)]
#[error("unable to generate background")]
pub struct BackgroundGenerationError;

#[derive(Error, Debug)]
#[error("unable to generate sex")]
pub struct SexGenerationError;

#[derive(Error, Debug)]
pub enum FlavorGenerationError {
    #[error("Flavor generation error: ancestry in unflavored statblock is None")]
    AncestryIsNone,
    #[error("Flavor generation error: heritage in unflavored statblock is None")]
    HeritageIsNone,
    #[error(transparent)]
    FlavorLineGenerationError(#[from] FlavorLineGenerationError),
}

#[derive(Error, Debug)]
pub enum FlavorLineGenerationError {
    #[error(transparent)]
    HairGenerationError(#[from] HairGenerationError),
}

impl<R: rand::Rng + Send + Sync> Generator<R> {
    pub fn new(
        rng: R,
        data: Arc<GeneratorData>,
        scripts: Arc<GeneratorScripts>,
    ) -> Option<Generator<R>> {
        Some(Self {
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
    ) -> Result<(&'a AgeRange, u64), AgeGenerationError> {
        let age_range = match age_range {
            Some(x) => x,
            None => ancestry
                .age_range_distribution()
                .split_weights()
                .map_err(|_| AgeGenerationError)
                .map(|(values, distribution)| values[distribution.sample(rng)])?,
        };

        let valid_ages = ancestry.age_ranges().get(age_range);
        Ok((age_range, valid_ages.choose(rng).ok_or(AgeGenerationError)?))
    }

    fn generate_ancestry(
        &self,
        rng: &mut impl Rng,
        ancestry_weights: Option<&WeightMap<Arc<str>>>,
    ) -> Result<Ancestry, AncestryGenerationError>
where {
        let ancestry = {
            let (values, distribution) = if let Some(ancestry_weights) = ancestry_weights {
                self.data
                    .ancestries
                    .split_weights_with_modifications(|x| {
                        ancestry_weights.get(x.name().as_ref()).copied()
                    })
                    .map_err(|_| AncestryGenerationError)?
            } else {
                self.data
                    .ancestries
                    .split_weights()
                    .map_err(|_| AncestryGenerationError)?
            };
            values[distribution.sample(rng)].clone()
        };
        Ok(ancestry)
    }

    #[tokio::main(flavor = "current_thread")]
    pub async fn generate(
        &mut self,
        options: &NpcOptions,
        weight_preset: Option<Arc<WeightPreset>>,
    ) -> Result<Statblock, GenerationError> {
        let mut rng = rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();
        let ancestry_weights = weight_preset.as_ref().map(|x| x.ancestry_weights());

        let ancestry = {
            let mut ancestry_rng = rngs::StdRng::from_rng(&mut rng).unwrap();
            match options.ancestry.clone() {
                Some(x) => x,
                None => self.generate_ancestry(&mut ancestry_rng, ancestry_weights)?,
            }
            .clone()
        };

        let heritage = {
            let mut heritage_rng = rngs::StdRng::from_rng(&mut rng).unwrap();

            match options.heritage.clone() {
                Some(x) => x,
                None => self.generate_heritage(&mut heritage_rng).await?,
            }
        };

        let background = {
            let mut background_rng = rngs::StdRng::from_rng(&mut rng).unwrap();

            match options.background {
                Some(ref x) => Cow::Borrowed(x),
                None => self.generate_background(&mut background_rng).await?,
            }
        };

        let sex = match options.sex.as_ref().map(|x| x.to_string()) {
            Some(x) => x,
            None => {
                if ancestry.is_asexual() {
                    String::new()
                } else {
                    let mut sex_rng = rngs::StdRng::from_rng(&mut rng).unwrap();
                    self.generate_sex(&mut sex_rng, &ancestry)?
                }
            }
        }
        .to_string();
        let (age_range, age) = {
            let mut age_rng = rngs::StdRng::from_rng(&mut rng).unwrap();
            self.generate_age(&mut age_rng, &ancestry, options.age_range.as_ref())
                .await?
        };

        let traits: Vec<Trait> = {
            let mut traits: HashSet<Trait> = HashSet::new();

            traits.extend(ancestry.traits().iter().cloned());
            traits.extend(heritage.iter().flat_map(|x| Vec::from(x.traits())));
            traits.insert(Trait::new(ancestry.size().to_string()));
            traits.into_iter().collect::<Vec<_>>()
        };

        let (background, statblock) = {
            let statblock = {
                let mut statblock = Statblock::default();
                statblock.set_name({
                    let mut names_rng = rngs::StdRng::from_rng(&mut rng).unwrap();
                    if options.enable_flavor_text {
                        self.generate_name(
                            &traits,
                            &mut names_rng,
                            &ancestry,
                            &self.data.names,
                            &sex,
                        )
                        .await
                    } else {
                        String::default()
                    }
                });
                statblock.set_age(age);
                statblock.set_age_range(*age_range);
                statblock.set_sex(sex);
                statblock.set_traits(traits.clone());
                statblock
            };

            if let Some(ref archetype) = options.archetype {
                let mut statblock = statblock.clone();
                let archetype_background =
                    Background::new(archetype.name(), vec![], Default::default());
                statblock.set_perception(archetype.perception());
                statblock.set_land_speed(archetype.speed());
                statblock.set_skills(
                    archetype
                        .skills()
                        .iter()
                        .map(|(x, y)| (x.clone(), *y))
                        .collect::<Vec<_>>(),
                );
                statblock.set_attributes(archetype.attributes().clone());
                statblock.set_items(archetype.items_iter());
                statblock.set_armor_class(archetype.armor_class());
                statblock.set_fortitude_save(archetype.fortitude_save());
                statblock.set_reflex_save(archetype.reflex_save());
                statblock.set_will_save(archetype.will_save());
                statblock.set_hit_points(archetype.hp());
                statblock.set_level(archetype.level());
                (Cow::Owned(archetype_background), statblock)
            } else {
                let mut stats_rng = rngs::StdRng::from_rng(&mut rng).unwrap();

                (
                    background.clone(),
                    generate_stats(
                        &mut stats_rng,
                        &ancestry,
                        heritage.as_ref(),
                        &background,
                        &statblock,
                    )?,
                )
            }
        };

        Ok({
            let mut statblock = statblock.clone();
            statblock.set_ancestry(Some(ancestry.clone()));
            statblock.set_heritage(heritage.clone());
            statblock.set_flavor(if options.enable_flavor_text {
                let mut flavor_rng = rngs::StdRng::from_rng(&mut rng).unwrap();
                self.generate_flavor(
                    ancestry.formats(),
                    self.scripts.clone(),
                    &mut flavor_rng,
                    &statblock,
                    &background,
                )
                .await?
            } else {
                Default::default()
            });
            statblock.set_class(background.name());

            debug!("Generated statblock {statblock:?}");
            statblock
        })
    }

    pub async fn generate_heritage(
        &self,
        rng: &mut impl Rng,
    ) -> Result<Option<Heritage>, GenerationError> {
        if rng.sample(
            rand::distributions::Bernoulli::new(self.data.normal_heritage_weight)
                .map_err(|_| HeritageGenerationError)?,
        ) {
            // TODO choose normal heritage
            Ok(None)
        } else {
            let heritage = {
                let (values, distribution) = &self
                    .data
                    .versitile_heritages
                    .split_weights()
                    .map_err(|_| HeritageGenerationError)?;
                values[distribution.sample(rng)].clone()
            };

            Ok(Some(heritage))
        }
    }

    pub async fn generate_background<'a>(
        &'a self,
        rng: &mut impl Rng,
    ) -> Result<Cow<'a, Background>, GenerationError> {
        let background: HashMap<Cow<'a, str>, &'a Background> =
            HashMap::from_iter(self.data.backgrounds.keys().map(|elem| (elem.name(), elem)));

        // if let Some(ancestry_weights) = ancestry_weights {
        // unimplemented!("Weighting ancestries are not implemented yet")
        // } else {
        let background_vec: Box<[&'a Background]> =
            background.values().cloned().collect::<Box<_>>();
        let result = background_vec
            .choose(rng)
            .ok_or(BackgroundGenerationError)?;
        Ok(Cow::Borrowed(result))
        /* } */
    }

    pub async fn generate_flavor(
        &self,
        formats: &Formats,
        generator_scripts: Arc<GeneratorScripts>,
        rng: &mut impl Rng,
        unflavored_statblock: &Statblock,
        background: &Background,
    ) -> Result<NpcFlavor, FlavorGenerationError> {
        let ancestry = unflavored_statblock
            .ancestry()
            .ok_or(FlavorGenerationError::AncestryIsNone)?;
        let heritage = unflavored_statblock.heritage();
        Ok(NpcFlavor {
            description_line: generate_flavor_description_line(
                generator_scripts,
                formats,
                unflavored_statblock.name(),
                unflavored_statblock.age(),
                unflavored_statblock.age_range(),
                unflavored_statblock.sex(),
                ancestry.name(),
                heritage.map(Heritage::name).as_deref(),
                background.name(),
                None,
            )
            .await,
            lineage_line: generate_lineage_line(heritage, formats).await,
            hair_and_eyes_line: generate_flavor_hair_and_eyes_line(
                rng, formats, ancestry, heritage,
            )?,
            skin_line: generate_flavor_skin_line(rng, formats, ancestry, heritage),

            size_and_build_line: generate_size_and_build(
                rng,
                formats,
                ancestry,
                unflavored_statblock.age(),
                unflavored_statblock.age_range(),
                heritage,
            ),
            face_line: generate_flavor_face_line(rng, formats, ancestry, unflavored_statblock),
            habit_line: generate_flavor_habit_line(rng, formats, ancestry),
        })
    }

    fn generate_sex(
        &self,
        random_number_generator: &mut impl Rng,
        _ancestry: &Ancestry,
    ) -> Result<String, GenerationError> {
        let sexes = ["male", "female"]; // TODO add diversity
        Ok(sexes
            .choose(random_number_generator)
            .ok_or(SexGenerationError)?
            .to_string())
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
                .cloned()
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

        let surname = if let Some(surnames) = ancestry.specimen_surnames() {
            let (surnames, weights) = surnames.split_weights().unwrap();
            surnames[name_rng.sample(weights)]
        } else {
            return first_name;
        };

        ancestry
            .formats()
            .format_full_name(&first_name, surname.as_ref(), vec![])
            .await
    }
}

fn generate_size_and_build(
    _rng: &mut impl Rng,
    _formats: &Formats,
    ancestry: &Ancestry,
    _age: u64,
    _age_range: AgeRange,
    _heritage: Option<&Heritage>,
) -> String {
    let bulk = match ancestry.size() {
        Size::Tiny => 1,
        Size::Small => 3,
        Size::Medium => 6,
        Size::Large => 12,
        Size::Huge => 24,
        Size::Garganutan => 48,
    };

    format!("They have a bulk of {bulk}.")
}

fn generate_flavor_face_line(
    _rng: &mut impl Rng,
    _formats: &Formats,
    _ancestry: &Ancestry,
    _statblock: &Statblock,
) -> String {
    // TODO

    "They have a face.".to_string()
}
fn generate_flavor_habit_line(
    _rng: &mut impl Rng,
    _formats: &Formats,
    _ancestry: &Ancestry,
) -> String {
    // TODO
    "They have habits.".to_string()
}

fn generate_stats(
    stats_rng: &mut rand::prelude::StdRng,
    ancestry: &Ancestry,
    _heritage: Option<&Heritage>,
    background: &Background,
    pre_statblock: &Statblock,
) -> Result<Statblock, GenerationError> {
    let level = pre_statblock.level();
    let mut attributes = AbilityStats::default();

    let mut choices: LinkedList<LinkedList<AbilityBoost>> = LinkedList::new();
    let mut choosen_this_round = HashSet::new();
    let mut current_choices: LinkedList<AbilityBoost> = LinkedList::new();
    for amod in ancestry.ability_modifications().iter() {
        match amod {
            AbilityBoost::Boost(ability) if !choosen_this_round.contains(ability) => {
                *attributes.get_ability_mut(*ability) += 1;
                choosen_this_round.insert(ability);
                current_choices.push_back(*amod);
            }
            AbilityBoost::Flaw(ability) if !choosen_this_round.contains(ability) => {
                *attributes.get_ability_mut(*ability) -= 1;
                choosen_this_round.insert(ability);
                current_choices.push_back(*amod);
            }
            AbilityBoost::Free => {
                let mut ability = Ability::values()
                    .choose(stats_rng)
                    .ok_or(AbilityGenerationError)?;
                while choosen_this_round.contains(ability) {
                    ability = Ability::values()
                        .choose(stats_rng)
                        .ok_or(AbilityGenerationError)?;
                }

                *attributes.get_ability_mut(*ability) += 1;
                choosen_this_round.insert(ability);
                current_choices.push_back(AbilityBoost::Boost(*ability));
            }
            _ => continue,
        }
    }
    choosen_this_round.clear();
    choices.push_back(current_choices.clone());
    current_choices.clear();
    for _ in 0..2 {
        let mut ability = Ability::values()
            .choose(stats_rng)
            .ok_or(AbilityGenerationError)?;
        while choosen_this_round.contains(ability) {
            ability = Ability::values()
                .choose(stats_rng)
                .ok_or(AbilityGenerationError)?;
        }

        *attributes.get_ability_mut(*ability) += 1;
        choosen_this_round.insert(ability);
        current_choices.push_back(AbilityBoost::Boost(*ability));
    }

    choices.push_back(current_choices.clone());
    choosen_this_round.clear();
    current_choices.clear();
    for _ in 0..2 {
        let mut ability = Ability::values()
            .choose(stats_rng)
            .ok_or(AbilityGenerationError)?;
        while choosen_this_round.contains(ability) {
            ability = Ability::values()
                .choose(stats_rng)
                .ok_or(AbilityGenerationError)?;
        }

        *attributes.get_ability_mut(*ability) += 1;

        current_choices.push_back(AbilityBoost::Boost(*ability));
    }

    choices.push_back(current_choices.clone());
    current_choices.clear();
    // verify attributes
    {
        let mut is_valid = true;
        for (attribute, value) in attributes.clone() {
            if value > 4 {
                error!("Given attribute is invalid: {:?}", attribute);
                is_valid = false;
            }
        }
        if !is_valid {
            info!("Attributes: {:?}", choices);
        }
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
                let choosen_skill = Skill::values_excluding_lore()
                    .choose(stats_rng)
                    .ok_or(SkillGenerationError)?;
                if !excluded_skills.contains(choosen_skill) {
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

    let hit_points: i32 = (ancestry.base_hp() as i32) + (attributes.constitution as i32);

    Ok({
        let mut statblock = pre_statblock.clone();

        statblock.set_perception(
            (attributes.wisdom + proficiencies.perception.bonus_for_level(level)) as i16,
        );
        statblock.set_fortitude_save(
            (attributes.constitution + proficiencies.fortitude_save.bonus_for_level(level)) as i16,
        );
        statblock.set_reflex_save(
            (attributes.dexterity + proficiencies.reflex_save.bonus_for_level(level)) as i16,
        );
        statblock.set_will_save(
            (attributes.wisdom + proficiencies.fortitude_save.bonus_for_level(level)) as i16,
        );
        statblock.set_armor_class(
            (attributes.dexterity + proficiencies.unarmored_defense.bonus_for_level(level)) as i16,
        );
        statblock.set_land_speed(ancestry.speed());

        statblock.set_skills(skills);
        statblock.set_attributes(attributes);
        statblock.set_proficiencies(proficiencies);
        statblock.set_hit_points(hit_points);
        statblock
    })
}

async fn generate_lineage_line(heritage: Option<&Heritage>, _formats: &Formats) -> Option<String> {
    if let Some(heritage) = heritage {
        match heritage
            .lineage()
            .as_ref()
            .map(|lineage| heritage.formats().format_lineage_line(lineage))
        {
            Some(x) => Some(x.await),
            None => None,
        }
    } else {
        None
    }
}

#[allow(clippy::too_many_arguments)]
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
    let job_name = match class_name {
        Some(x) => x,
        None => background_name,
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
}

fn generate_flavor_hairs(
    rng: &mut impl Rng,
    _formats: &Formats,
    ancestry: &Ancestry,
    _heritage: Option<&Heritage>,
) -> Result<String, HairGenerationError> {
    let ancestry_hair_type = ancestry.possible_hair_type();
    let ancestry_hair_colors = ancestry.possible_hair_colors();
    let ancestry_hair_length = ancestry.possible_hair_length();

    let (possible_hair_colors, possible_hair_type, possible_hair_length) = match (
        ancestry_hair_colors,
        ancestry_hair_type,
        ancestry_hair_length,
    ) {
        (Some(hair_type), Some(hair_color), Some(hair_length)) => {
            (hair_color, hair_type, hair_length)
        }
        _ => return Ok("no hair".to_string()),
    };

    let hair_color: String = {
        let (values, distribution) = possible_hair_colors.split_weights().map_err(|_| "color")?;
        (values[distribution.sample(rng)]).as_ref().into()
    };
    let hair_type: String = {
        let (values, distribution) = possible_hair_type.split_weights().map_err(|_| "type")?;
        (values[distribution.sample(rng)]).as_ref().into()
    };
    let hair_length: String = {
        let (values, distribution) = possible_hair_length.split_weights().map_err(|_| "length")?;
        (values[distribution.sample(rng)]).as_ref().into()
    };

    let hair = ancestry.hair_substance();
    Ok(format!("{hair_length}, {hair_type}, {hair_color} {hair}"))
}

fn generate_flavor_eyes(
    rng: &mut impl Rng,
    _formats: &Formats,
    ancestry: &Ancestry,
    heritage: Option<&Heritage>,
) -> String {
    let mut available_eye_colors: WeightMap<Cow<str>> = WeightMap::new();
    if let Some(x) = ancestry.possible_eye_colors() {
        available_eye_colors.extend(x.into_iter().map(|(k, v)| (Cow::Borrowed(k.as_ref()), *v)));
    } else {
        return "no eyes".into();
    };

    if let Some(heritage) = heritage {
        let eye_colors = heritage
            .additional_eye_colors()
            .iter()
            .map(|(k, v)| (Cow::Borrowed(k.as_ref()), *v));
        available_eye_colors.extend(eye_colors);
    }

    let (available_eye_colors, distribution) = available_eye_colors.split_weights().unwrap();

    let eye_color: &str = available_eye_colors[distribution.sample(rng)];
    let heterochromia_color: &str = available_eye_colors[distribution.sample(rng)];
    let force_heterochromia = if let Some(heritage) = heritage {
        heritage.force_heterochromia()
    } else {
        None
    };
    let (has_heterochromia, heterochromia_color): (bool, &str) =
        if let Some(color) = &force_heterochromia {
            (true, color)
        } else {
            let dist = rand::distributions::Bernoulli::new(
                ancestry.mutation_probabilities()[&Mutation::Heterochromia],
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
    } else {
        format!("{eye_color} eyes")
    }
}

fn generate_flavor_hair_and_eyes_line(
    mut rng: &mut impl Rng,
    formats: &Formats,
    ancestry: &Ancestry,
    heritage: Option<&Heritage>,
) -> Result<String, FlavorLineGenerationError> {
    Ok(format!(
        "They have {} and {}.",
        generate_flavor_hairs(&mut rng, formats, ancestry, heritage)?,
        generate_flavor_eyes(&mut rng, formats, ancestry, heritage)
    ))
}
fn generate_flavor_skin_line(
    rng: &mut impl Rng,
    _formats: &Formats,
    ancestry: &Ancestry,
    _heritage: Option<&Heritage>,
) -> String {
    let skin_texture: &str = {
        let (skin_textures, distribution) =
            ancestry.possible_skin_texture().split_weights().unwrap();
        skin_textures[rng.sample(distribution)]
    }
    .as_ref();

    let skin_tone: &str = {
        let (skin_tones, distribution) = ancestry.possible_skin_tone().split_weights().unwrap();
        skin_tones[rng.sample(distribution)]
    }
    .as_ref();
    let skin: &str = ancestry.skin_substance();
    format!("They have {skin_texture} {skin_tone} {skin}.")
}
