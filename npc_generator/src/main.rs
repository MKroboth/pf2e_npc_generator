use egui;
use npc_generator_core::generators::GeneratorData;
use npc_generator_core::{generators::Generator, *};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;
use std::env::args;
use std::io::Write;
use std::{
    error::Error,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use std::{io, usize};
mod ui;

fn generate_distribution_preview() -> Result<(), Box<dyn Error>> {
    let generator_data = load_generator_data()?;

    let sample_size = 2_000_000;
    let (results, heritages, elapsed) = {
        let results: HashMap<String, usize> = HashMap::new();
        let heritages: HashMap<String, usize> = HashMap::new();

        println!("Generating npcs...");
        let results = Mutex::new(results);
        let heritages = Mutex::new(heritages);

        use std::time::Instant;
        let now = Instant::now();
        (0..sample_size).into_par_iter().for_each(|i| {
            let mut generator = Generator::new(rand::thread_rng(), generator_data.clone()).unwrap();
            let percent = (100.0 / sample_size as f64) * i as f64;
            if percent as u8 as f64 == percent {
                print!("+");
                io::stdout().flush().unwrap();
            }

            let npc_options = NpcOptions {
                ..Default::default()
            };
            let result = generator.generate(&npc_options);
            let ancestry = result.ancestry.unwrap();
            let heritage = result.heritage;
            let ancestry_name = ancestry.name();
            let heritage_name = heritage.as_ref().map(|x| x.name()).unwrap_or("Normal");
            let mut results = results.lock().unwrap();
            let mut heritages = heritages.lock().unwrap();

            if !results.contains_key(ancestry_name) {
                results.insert(ancestry_name.to_string(), 0);
            }
            *results.get_mut(ancestry_name).unwrap() += 1;

            if !heritages.contains_key(heritage_name) {
                heritages.insert(heritage_name.to_string(), 0);
            }
            *heritages.get_mut(heritage_name).unwrap() += 1;
        });
        (
            results.into_inner().unwrap(),
            heritages.into_inner().unwrap(),
            now.elapsed(),
        )
    };
    println!();
    println!(
        "generation of {sample_size} samples took {:.4} seconds, printing results\n====",
        elapsed.as_secs_f32()
    );
    println!("From a sample size of {sample_size}, we have the following population count:");
    let mut sorted_population = results.into_iter().collect::<Vec<(String, usize)>>();
    let mut sorted_heritages = heritages.into_iter().collect::<Vec<(String, usize)>>();

    sorted_population.sort_by_key(|x| x.1);
    sorted_heritages.sort_by_key(|x| x.1);

    for (ancestry, count) in sorted_population {
        let population_percent = (100.0 / sample_size as f64) * count as f64;
        println!(
            "{:<10}: {:>6.2}% ({:>8})",
            ancestry, population_percent, count
        );
    }

    println!("The heritages are split as follows:");
    for (heritage, count) in sorted_heritages {
        let population_percent = (100.0 / sample_size as f64) * count as f64;
        println!(
            "{:<10}: {:>6.2}% ({:>8})",
            heritage, population_percent, count
        );
    }
    Ok(())
}

fn generate_character() -> Result<(), Box<dyn Error>> {
    let generator_data = load_generator_data()?;
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Character Generator",
        native_options,
        Box::new(move |cc| Box::new(ui::UserInterface::new(cc, generator_data.clone()))),
    )?;
    Ok(())
}

fn load_generator_data() -> Result<Arc<GeneratorData>, Box<dyn Error>> {
    let mut data: PathBuf = std::env::current_dir()?;
    data.push("data");

    let ancestries: WeightMap<Ancestry> = {
        let mut path = data.clone();
        path.push("ancestries.ron");
        ron::from_str(&fs::read_to_string(path)?)?
    };
    let heritages: WeightMap<Heritage> = {
        let mut path = data.clone();
        path.push("heritages.ron");
        ron::from_str(&fs::read_to_string(path)?)?
    };
    let backgrounds: WeightMap<Background> = {
        let mut path = data.clone();
        path.push("backgrounds.ron");
        ron::from_str(&fs::read_to_string(path)?)?
    };
    let names: HashMap<Trait, HashMap<String, WeightMap<String>>> = {
        let mut path = data.clone();
        path.push("names.ron");
        ron::from_str(&fs::read_to_string(path)?)?
    };

    let mut archetypes: Vec<Archetype> = {
        let mut path = data.clone();
        path.push("archetypes.ron");
        ron::from_str(&fs::read_to_string(path)?)?
    };
    archetypes.sort_by_key(|x| x.level);

    Ok(Arc::new(npc_generator_core::generators::GeneratorData {
        ancestries,
        versitile_heritages: heritages,
        normal_heritage_weight: 0.8,
        backgrounds,
        heritages: Default::default(),
        names,
        archetypes,
    }))
}

fn main() -> Result<(), Box<dyn Error>> {
    if args().len() > 1 {
        generate_distribution_preview()
    } else {
        generate_character()
    }
}
