use super::*;
use rand::distributions::Distribution;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::{rngs, Rng, SeedableRng};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

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
#[derive(Debug, Serialize, Deserialize)]
pub struct Generator<R: rand::Rng> {
    random_number_generator: R,
    pub data: Arc<GeneratorData>,
}
impl<R: rand::Rng> Generator<R> {
    pub fn new(
        rng: R,
        data: Arc<GeneratorData>,
    ) -> Result<Generator<R>, Box<dyn std::error::Error>> {
        Ok(Self {
            random_number_generator: rng,
            data,
        })
    }

    fn generate_age<'a>(&self, rng: &mut impl Rng, ancestry: &'a Ancestry) -> (&'a AgeRange, u64) {
        let age_range = {
            let (values, distribution) = &ancestry.age_range_distribution.split_weights().unwrap();
            values[distribution.sample(rng)]
        };

        let valid_ages = ancestry.age_ranges.get(age_range);
        (age_range, valid_ages.choose(rng).unwrap())
    }

    fn generate_ancestry(
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

    pub fn generate(&mut self, options: &NpcOptions) -> Statblock {
        let ancestry_weights = options.ancestry_weights.as_ref();

        let ancestry: Ancestry = {
            let mut ancestry_rng =
                rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();

            options
                .ancestry
                .clone()
                .unwrap_or_else(|| self.generate_ancestry(&mut ancestry_rng, ancestry_weights))
                .clone()
        };

        let heritage: Option<Heritage> = {
            let mut heritage_rng =
                rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();

            options
                .heritage
                .clone()
                .unwrap_or_else(|| self.generate_heritage(&mut heritage_rng))
        };

        let background = {
            let mut background_rng =
                rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();

            options
                .background
                .clone()
                .unwrap_or_else(|| self.generate_background(&mut background_rng))
        };

        let sex = {
            if ancestry.is_asexual {
                String::new()
            } else {
                let mut sex_rng =
                    rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();
                self.generate_sex(&mut sex_rng, &ancestry)
            }
        };
        let (age_range, age) = {
            let mut age_rng = rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();
            self.generate_age(&mut age_rng, &ancestry)
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
                self.generate_name(&traits, &mut names_rng, &ancestry, &self.data.names, &sex)
            },
            sex,
            ..Default::default()
        };

        let pre_statblock = if let Some(ref archetype) = options.archetype {
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
            }
        } else {
            Statblock { ..pre_statblock }
        };
        let mut flavor_rng = rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();

        Statblock {
            ancestry: Some(ancestry.clone()),
            heritage: heritage.clone(),
            flavor: self.generate_flavor(
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
            ),
            class: background.name,
            ..pre_statblock
        }
    }

    pub fn generate_heritage(&self, rng: &mut impl Rng) -> Option<Heritage> {
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

    pub fn generate_background(&self, rng: &mut impl Rng) -> Background {
        let background: HashMap<&str, Background> = HashMap::from_iter(
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

    pub fn generate_flavor(
        &self,
        rng: &mut impl Rng,
        _skills: &[(Skill, i16)],
        _attributes: &AttributeStats,
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
                name,
                age,
                age_range,
                sex,
                &ancestry.name,
                heritage.map(NamedElement::name),
                background.name,
                None,
            ),
            lineage_line: generate_lineage_line(heritage),
            hair_and_eyes_line: generate_flavor_hair_and_eyes_line(rng, &ancestry, heritage),
        }
    }

    fn generate_sex(&self, random_number_generator: &mut impl Rng, _ancestry: &Ancestry) -> String {
        let sexes = vec!["male", "female"]; // TODO add diversity
        String::from_str(sexes.choose(random_number_generator).unwrap()).unwrap()
    }

    fn generate_name(
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

        let names = names.get(name_trait).unwrap().get(sex).expect(&format!(
            "No names for given sex `{sex}` present on ancestry `{}`",
            ancestry.name
        ));
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

        format!("{first_name} {surname}")
    }
}

fn generate_lineage_line(heritage: Option<&Heritage>) -> Option<String> {
    if let Some(heritage) = heritage {
        heritage
            .lineage
            .as_ref()
            .map(|lineage| format!("They are of the {lineage} lineage"))
    } else {
        None
    }
}

fn generate_flavor_description_line(
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

    match age_range {
        AgeRange::Infant => {
            if age == 0 {
                format!("{name} is a{sex} {ancestry_name}{heritage_name} newborn.")
            } else {
                format!("{name} is a {age} year old{sex} {ancestry_name}{heritage_name} infant.")
            }
        }
        AgeRange::Child => {
            format!(
                "{name} is a {age} year old{sex} {ancestry_name}{heritage_name} child {job_name}."
            )
        }
        AgeRange::Youth => {
            format!("{name} is a {age} year old{sex} {ancestry_name}{heritage_name} {job_name} in their youths.")
        }
        AgeRange::Adult => {
            format!("{name} is an adult, {age} year old{sex} {ancestry_name}{heritage_name} {job_name}.")
        }
        AgeRange::MiddleAged => {
            format!("{name} is a middle-aged, {age} year old{sex} {ancestry_name}{heritage_name} {job_name}.")
        }
        AgeRange::Old => {
            format!(
                "{name} is an old, {age} year old{sex} {ancestry_name}{heritage_name} {job_name}."
            )
        }
        AgeRange::Venerable => {
            format!("{name} is a venerable, {age} year old{sex} {ancestry_name}{heritage_name} {job_name}.")
        }
    }
}

fn generate_flavor_hairs(
    rng: &mut impl Rng,
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
    ancestry: &Ancestry,
    heritage: Option<&Heritage>,
) -> String {
    format!(
        "They have {} and {}.",
        generate_flavor_hairs(&mut rng, &ancestry, heritage),
        generate_flavor_eyes(&mut rng, ancestry, heritage)
    )
    .into()
}
