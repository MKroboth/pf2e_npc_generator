use super::*;
use std::path::Path;
use rand::distributions::WeightedIndex;
use rand::distributions::Distribution;
use rand::Rng;

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
    pub flavor: NpcFlavor
}

struct Generator {}
impl Generator {
    pub fn new(data_path: &Path) -> Result<Generator, Box<dyn std::error::Error>> { todo!() }


    fn generate_ancestry(&self, ancestry_weights: Option<AncestryWeights>) -> Ancestry { todo!() }

    pub fn generate(&self, options: NpcOptions) -> Statblock {
        let ancestry = options.ancestry.ok_or_else(|| self.generate_ancestry(options.ancestry_weights));
    todo!()
}


    pub fn generate_flavor(&self,
        skills: Vec<(Skill, i16)>,
        abilities: Vec<(Ability, i16)>, 
        name: impl AsRef<str>,
        class: impl AsRef<str>,
        level: i8,
        age: i8,
        perception: i16,
        fortitude_save: i16,
        reflex_save: i16,
        will_save: i16,
        ancestry: Ancestry,
        background: Background,
    ) -> NpcFlavor {

    todo!()
    }
}


   fn generate_flavor_description_line(name: impl AsRef<str>, age: u128, sex: impl AsRef<str>, ancestry_name: impl AsRef<str>, background_name: impl AsRef<str>, class_name: Option<&str>) -> String {
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

fn generate_flavor_hairs(rng: &mut impl Rng, ancestry: &Ancestry, heritage: &Heritage) -> String {
    todo!()
}

fn generate_flavor_eyes(rng: &mut impl Rng, ancestry: &Ancestry, heritage: &Heritage) -> String {
    let mut available_eye_colors = Vec::new(); 
    if let Some(x) = &ancestry.possible_eye_colors {
        available_eye_colors.extend_from_slice(&x.0);
    } else {
        return "no eyes".into();
    };

    available_eye_colors.extend_from_slice(&heritage.additional_eye_colors.0);
        
    let weights = available_eye_colors.iter()
        .map(|element| element.weight)
        .collect::<Vec<_>>();
    let distribution = WeightedIndex::new(&weights).unwrap();

    let eye_color: String =  (*available_eye_colors[distribution.sample(rng)].element).into();
    let heterochromia_color: String =  (*available_eye_colors[distribution.sample(rng)].element).into();
    let (has_heterochromia, heterochromia_color): (bool, String) = 
    if let Some(color) = &heritage.force_heterochromia { 
        (true, String::from((&**color)))
    } else {
        (false, (heterochromia_color))
    };

    if has_heterochromia {
        format!("hc").into()
    } else {
        format!("{eye_color} eyes").into()
    }
}

fn generate_flavor_hair_and_eyes_line(mut rng: &mut impl Rng, ancestry: &Ancestry, heritage: &Heritage) -> String {
    format!("They have {} and {}.", generate_flavor_hairs(&mut rng, &ancestry, &heritage), 
        generate_flavor_eyes(&mut rng, ancestry, heritage)).into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_if_generate_flavor_description_line_yields_expected_result() {
        assert_eq!("Labras Nagrabosch is a 81 year old male gnome merchant.", super::generate_flavor_description_line("Labras Nagrabosch", 81, "male", "gnome", "merchant", None));
    }

    #[test]
    fn check_if_generate_flavor_eyes_works() {
        use std::path::PathBuf;
        use crate::{Ancestry, Heritage, ValidAncestries, traits, Trait};
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/ancestries.ron");
        let ancestries: Vec<Ancestry> = ron::from_str(&std::fs::read_to_string(d).unwrap()).unwrap();

        let mut rng = rand::thread_rng();

        let got = super::generate_flavor_eyes(&mut rng, ancestries.first().unwrap(), &Heritage::new(traits! ["Gnome", "Humanoid"], "Gnome", ValidAncestries::Any, vec![], vec![], None));


        assert_eq!("", got);
    }
}
