use npc_generator_core::{
    generators::{Generator, GeneratorData},
    *,
};
use std::{
    alloc::System,
    collections::HashMap,
    error::Error,
    fs, io,
    path::{self, Path, PathBuf},
};
use stderrlog;

fn main() -> Result<(), Box<dyn Error>> {
    stderrlog::new()
        .module(module_path!())
        .module("npc_generator_core::generators")
        .init()
        .unwrap();

    let mut data: PathBuf = std::env::current_dir()?;
    data.push("data");

    let ancestries: HashMap<Ancestry, i32> = {
        let mut path = data.clone();
        path.push("ancestries.ron");
        ron::from_str(&fs::read_to_string(path)?)?
    };
    let heritages: HashMap<Heritage, i32> = {
        let mut path = data.clone();
        path.push("heritages.ron");
        ron::from_str(&fs::read_to_string(path)?)?
    };
    let backgrounds: HashMap<Background, i32> = {
        let mut path = data.clone();
        path.push("backgrounds.ron");
        ron::from_str(&fs::read_to_string(path)?)?
    };

    let generator_data = npc_generator_core::generators::GeneratorData {
        ancestries,
        special_heritages: heritages,
        normal_heritage_weight: 0.7,
        backgrounds,
    };

    let mut generator = Generator::new(rand::thread_rng(), generator_data)?;

    let npc_options = NpcOptions {
        ancestry: None,
        heritage: None,
        background: None,
        ancestry_weights: None,
    };
    let result = generator.generate(npc_options);
    println!("{}", result.flavor.description_line);
    println!("{}", result.flavor.hair_and_eyes_line);

    Ok(())
}
