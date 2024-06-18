use egui;
use npc_generator_core::{generators::Generator, *};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::io;
use std::io::Write;
use std::{
    collections::HashMap,
    error::Error,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use stderrlog;

fn generate_distribution_preview() -> Result<(), Box<dyn Error>> {
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
    let names: HashMap<Trait, HashMap<String, HashMap<String, i32>>> = {
        let mut path = data.clone();
        path.push("names.ron");
        ron::from_str(&fs::read_to_string(path)?)?
    };

    let generator_data = Arc::new(npc_generator_core::generators::GeneratorData {
        ancestries,
        special_heritages: heritages,
        normal_heritage_weight: 0.8,
        backgrounds,
        names,
    });

    fn index_for_ancestry(ancestry: &Ancestry) -> usize {
        match ancestry.name.as_str() {
            "Human" => 0,
            "Dwarf" => 1,
            "Elf" => 2,
            "Gnome" => 3,
            "Goblin" => 4,
            "Halfling" => 5,
            "Leshy" => 6,
            "Orc" => 7,
            "Kitsune" => 8,
            _ => panic!(),
        }
    }

    fn index_for_heritage(heritage: &Option<Heritage>) -> usize {
        if let Some(heritage) = heritage {
            match heritage.name.as_str() {
                "Aiuvarin" => 1,
                "Dromaar" => 2,
                "Changeling" => 3,
                "Nephilim" => 4,
                _ => panic!(),
            }
        } else {
            0
        }
    }

    let results: [u64; 9] = [0; 9];
    let heritages: [u64; 5] = [0; 5];
    println!("Generating npcs...");
    let results = Mutex::new(results);
    let heritages = Mutex::new(heritages);
    let sample_size = 2_000_000;
    (0..sample_size).into_par_iter().for_each(|i| {
        let mut generator = Generator::new(rand::thread_rng(), generator_data.clone()).unwrap();
        let percent = (100.0 / sample_size as f64) * i as f64;
        if percent as u8 as f64 == percent {
            print!("+");
            io::stdout().flush().unwrap();
        }

        let npc_options = NpcOptions {
            ancestry: None,
            heritage: None,
            background: None,
            ancestry_weights: None,
        };
        let result = generator.generate(npc_options);
        let ancestry = result.ancestry.unwrap();
        let heritage = result.heritage;
        let index = index_for_ancestry(&ancestry);
        results.lock().unwrap()[index] += 1;
        heritages.lock().unwrap()[index_for_heritage(&heritage)] += 1;
    });
    let results = results.lock().unwrap();
    let heritages = heritages.lock().unwrap();

    println!("printing results\n====");
    println!("From a sample size of {sample_size}, we have the following population count:");
    let mut sorted_population = vec![
        ("Human", results[0]),
        ("Dwarf", results[1]),
        ("Elf", results[2]),
        ("Gnome", results[3]),
        ("Goblin", results[4]),
        ("Halfling", results[5]),
        ("Leshy", results[6]),
        ("Orc", results[7]),
        ("Kistune", results[8]),
    ];
    let mut sorted_heritages = vec![
        ("Normal", heritages[0]),
        ("Aiuvarin", heritages[1]),
        ("Dromaar", heritages[2]),
        ("Changeling", heritages[3]),
        ("Nephilim", heritages[4]),
    ];

    sorted_population.sort_by_key(|x| x.1);
    sorted_heritages.sort_by_key(|x| x.1);

    for (ancestry, count) in sorted_population {
        let population_percent = (100.0 / sample_size as f64) * count as f64;
        println!(
            "{:<10}: {:>3.2}% ({:>20})",
            ancestry, population_percent, count
        );
    }

    println!("The heritages are split as follows:");
    for (heritage, count) in sorted_heritages {
        let population_percent = (100.0 / sample_size as f64) * count as f64;
        println!(
            "{:<10}: {:>3.2}% ({:>20})",
            heritage, population_percent, count
        );
    }
    Ok(())
}

fn generate_character() -> Result<(), Box<dyn Error>> {
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
    let names: HashMap<Trait, HashMap<String, HashMap<String, i32>>> = {
        let mut path = data.clone();
        path.push("names.ron");
        ron::from_str(&fs::read_to_string(path)?)?
    };

    let generator_data = Arc::new(npc_generator_core::generators::GeneratorData {
        ancestries,
        special_heritages: heritages,
        normal_heritage_weight: 0.8,
        backgrounds,
        names,
    });
    let mut ctx = egui::Context::default();


    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(|cc| Box::new(UserInterface::new(cc))),
    )
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    generate_distribution_preview()
}
