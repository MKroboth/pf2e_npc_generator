use super::*;
use log::{error, info, warn};
use rand::distributions::uniform::SampleUniform;
use rand::distributions::Distribution;
use rand::seq::SliceRandom;
use rand::{rngs, Rng, SeedableRng};
use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;
use std::str::FromStr;

#[derive(Default, Debug)]
pub struct NpcFlavor {
    pub description_line: String,
    pub hair_and_eyes_line: String,
}

#[derive(Default, Debug)]
pub struct Statblock {
    pub name: String,
    pub class: String,
    pub level: i8,
    pub age: u128,
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
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct GeneratorData {
    pub ancestries: HashMap<Ancestry, i32>,
    pub normal_heritage_weight: f64,
    pub special_heritages: HashMap<Heritage, i32>,
    pub backgrounds: HashMap<Background, i32>,
}
pub struct Generator<R: rand::Rng> {
    random_number_generator: R,
    data: GeneratorData,
}
impl<R: rand::Rng> Generator<R> {
    pub fn new(rng: R, data: GeneratorData) -> Result<Generator<R>, Box<dyn std::error::Error>> {
        Ok(Self {
            random_number_generator: rng,
            data,
        })
    }

    fn generate_age(&self, rng: &mut impl Rng, ancestry: &Ancestry) -> u128 {
        0
    }

    fn generate_ancestry(
        &self,
        rng: &mut impl Rng,
        ancestry_weights: Option<AncestryWeights>,
    ) -> Ancestry {
        let ancestries: HashMap<&str, Ancestry> = HashMap::from_iter(
            self.data
                .ancestries
                .keys()
                .into_iter()
                .map(|elem| (elem.name(), elem.clone())),
        );

        if let Some(ancestry_weights) = ancestry_weights {
            unimplemented!("Weighting ancestries are not implemented yet")
        } else {
            let ancestry_vec: Vec<Ancestry> = ancestries
                .values()
                .into_iter()
                .map(Clone::clone)
                .collect::<Vec<_>>();
            let result = ancestry_vec
                .choose(rng)
                .expect("Ancestry list is empty")
                .clone();
            result
        }
    }

    pub fn generate(&mut self, options: NpcOptions) -> Statblock {
        let ancestry: Ancestry = {
            let mut ancestry_rng =
                rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();

            options.ancestry.unwrap_or_else(|| {
                self.generate_ancestry(&mut ancestry_rng, options.ancestry_weights)
            })
        };

        let heritage: Option<Heritage> = {
            let mut heritage_rng =
                rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();

            options
                .heritage
                .or_else(|| self.generate_heritage(&mut heritage_rng))
        };

        let background = {
            let mut background_rng =
                rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();

            options
                .background
                .unwrap_or_else(|| self.generate_background(&mut background_rng))
        };

        let pre_statblock = Statblock {
            age: {
                let mut age_rng =
                    rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();
                self.generate_age(&mut age_rng, &ancestry)
            },
            sex: {
                let mut sex_rng =
                    rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();
                self.generate_sex(&mut sex_rng, &ancestry)
            },
            ..Default::default()
        };
        let mut flavor_rng = rngs::StdRng::from_rng(&mut self.random_number_generator).unwrap();

        Statblock {
            flavor: self.generate_flavor(
                &mut flavor_rng,
                &pre_statblock.skills,
                &pre_statblock.abilities,
                &pre_statblock.name,
                None, // class not yet supported
                pre_statblock.level,
                pre_statblock.age,
                pre_statblock.perception,
                pre_statblock.fortitude_save,
                pre_statblock.reflex_save,
                pre_statblock.will_save,
                ancestry,
                heritage.as_ref(),
                background,
                &pre_statblock.sex,
            ),
            ..pre_statblock
        }
    }

    pub fn generate_heritage(&self, rng: &mut impl Rng) -> Option<Heritage> {
        let heritage: HashMap<&str, Heritage> = HashMap::from_iter(
            self.data
                .special_heritages
                .keys()
                .into_iter()
                .map(|elem| (elem.name(), elem.clone())),
        );

        if rng
            .sample(rand::distributions::Bernoulli::new(self.data.normal_heritage_weight).unwrap())
        {
            None
        } else {
            let ancestry_vec: Vec<Heritage> = heritage
                .values()
                .into_iter()
                .map(Clone::clone)
                .collect::<Vec<_>>();
            let result = ancestry_vec
                .choose(rng)
                .expect("Ancestry list is empty")
                .clone();
            println!();
            Some(result)
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
        skills: &[(Skill, i16)],
        abilities: &[(Ability, i16)],
        name: impl AsRef<str>,
        class: Option<&str>,
        level: i8,
        age: u128,
        perception: i16,
        fortitude_save: i16,
        reflex_save: i16,
        will_save: i16,
        ancestry: Ancestry,
        heritage: Option<&Heritage>,
        background: Background,
        sex: impl AsRef<str>,
    ) -> NpcFlavor {
        NpcFlavor {
            description_line: generate_flavor_description_line(
                name,
                age,
                sex,
                &ancestry.name,
                background.name,
                None,
            ),
            hair_and_eyes_line: generate_flavor_hair_and_eyes_line(rng, &ancestry, heritage),
        }
    }

    fn generate_sex(&self, random_number_generator: &mut impl Rng, ancestry: &Ancestry) -> String {
        let sexes = vec!["male", "female"]; // TODO add diversity
        String::from_str(sexes.choose(random_number_generator).unwrap()).unwrap()
    }
}

fn generate_flavor_description_line(
    name: impl AsRef<str>,
    age: u128,
    sex: impl AsRef<str>,
    ancestry_name: impl AsRef<str>,
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

    format!("{name} is a {age} year old {sex} {ancestry_name} {job_name}.").into()
}

fn generate_flavor_hairs(
    rng: &mut impl Rng,
    ancestry: &Ancestry,
    heritage: Option<&Heritage>,
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
        let (values, distribution) = split_weights(&possible_hair_colors).unwrap();
        (*values[distribution.sample(rng)]).into()
    };
    let hair_type: String = {
        let (values, distribution) = split_weights(&possible_hair_type).unwrap();
        (*values[distribution.sample(rng)]).into()
    };
    let hair_length: String = {
        let (values, distribution) = split_weights(&possible_hair_length).unwrap();
        (*values[distribution.sample(rng)]).into()
    };

    format!("{hair_length}, {hair_type}, {hair_color} hair")
}

fn split_weights<Weight, Value>(
    container: &HashMap<Value, Weight>,
) -> Result<
    (Vec<Value>, rand::distributions::WeightedIndex<Weight>),
    rand::distributions::WeightedError,
>
where
    Weight: SampleUniform + PartialOrd + Default + Clone + for<'a> core::ops::AddAssign<&'a Weight>,
    Value: Hash + Clone,
{
    let mut weights = Vec::new();
    let mut values = Vec::new();

    for (value, weight) in container {
        values.push(value.clone());
        weights.push(weight);
    }

    Ok((values, rand::distributions::WeightedIndex::new(weights)?))
}

fn generate_flavor_eyes(
    rng: &mut impl Rng,
    ancestry: &Ancestry,
    heritage: Option<&Heritage>,
) -> String {
    let mut available_eye_colors: HashMap<String, i32> = HashMap::new();
    if let Some(x) = &ancestry.possible_eye_colors {
        available_eye_colors.extend(x.clone());
    } else {
        return "no eyes".into();
    };

    if let Some(heritage) = heritage {
        available_eye_colors.extend(heritage.additional_eye_colors.clone());
    }

    let (available_eye_colors, distribution) = split_weights(&available_eye_colors).unwrap();

    let eye_color: String = (*available_eye_colors[distribution.sample(rng)]).into();
    let heterochromia_color: String = (*available_eye_colors[distribution.sample(rng)]).into();
    let force_heterochromia = if let Some(heritage) = heritage {
        heritage.force_heterochromia.clone()
    } else {
        None
    };
    let (has_heterochromia, heterochromia_color): (bool, String) =
        if let Some(color) = &force_heterochromia {
            (true, color.to_string())
        } else {
            let dist = rand::distributions::Bernoulli::new(
                ancestry.mutation_probabilities[&Mutation::Heterochromia],
            )
            .unwrap();
            (dist.sample(rng), (heterochromia_color))
        };

    if has_heterochromia {
        let mut eye_color = eye_color;
        while heterochromia_color == eye_color {
            eye_color = (*available_eye_colors[distribution.sample(rng)]).into();
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

#[cfg(test)]
mod tests {
    #[test]
    fn check_if_generate_flavor_description_line_yields_expected_result() {
        assert_eq!(
            "Labras Nagrabosch is a 81 year old male gnome merchant.",
            super::generate_flavor_description_line(
                "Labras Nagrabosch",
                81,
                "male",
                "gnome",
                "merchant",
                None
            )
        );
    }
}
