use npc_generator_core::*;
use std::error::Error;

const ANCESTRY_DATA: &'static str = /* ron */ r#"
#![enable(unwrap_newtypes)]
[
    Ancestry(
        traits: [
            "Human",
            "Humanoid",
        ],
        name: "Human",
        ability_modifications: [
            Free,
            Free,
        ],
        languages: [
            Language(
                traits: [],
                name: "Common",
            ),
        ],
        senses: [],
        size: Medium,
        speed: 25,
        possible_eye_colors: Some([
            WeightedElement(
                weight: 8,
                element: "Brown",
            ),
            WeightedElement(
                weight: 4,
                element: "Blue",
            ),
            WeightedElement(
                weight: 4,
                element: "Green",
            ),
            WeightedElement(
                weight: 2,
                element: "Golden",
            ),
            WeightedElement(
                weight: 1,
                element: "Cyan",
            ),
            WeightedElement(
                weight: 2,
                element: "Gray",
                ),
            WeightedElement(
                weight: 2,
                element: "Black",
            ),
            WeightedElement(
                weight: 1,
                element: "Red",
            ),
        ]),
        possible_hair_colors: Some([
            WeightedElement(
                weight: 8,
                element: "Brown",
            ),
        ]),
    ),
]
"#;

fn main() -> Result<(), Box<dyn Error>>{
  let common = language!("Common");
  
  let ancestries: Vec<Ancestry> = ron::from_str(ANCESTRY_DATA)?;
  let languages = vec![common];
  // let heritages = vec![aiuvarinn];
//
  println!("{}", ron::ser::to_string_pretty(&ancestries, ron::ser::PrettyConfig::default()
    .struct_names(true)
    .extensions(ron::extensions::Extensions::UNWRAP_NEWTYPES)
  ).unwrap());

  Ok(())
}
