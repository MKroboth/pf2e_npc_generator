use clap::{Parser, ValueEnum};
#[cfg(feature = "rayon")]
use indicatif::ParallelProgressIterator;
#[cfg(not(feature = "rayon"))]
use indicatif::ProgressIterator;
use indicatif::ProgressStyle;
use log::{error, info};
use native_dialog::FileDialog;
use npc_generator_core::generators::{GeneratorData, GeneratorScripts};
use npc_generator_core::{generators::Generator, *};
use rand::SeedableRng;
#[cfg(feature = "rayon")]
use rayon::iter::IntoParallelIterator;
#[cfg(feature = "rayon")]
use rayon::iter::ParallelIterator;
use std::collections::{HashMap, LinkedList};
use std::fs::File;
use std::path::Path;
use std::{
    error::Error,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use std::{io, usize};
mod config;
mod ui;
use anyhow::{anyhow, Context, Result};

#[cfg(not(feature = "rayon"))]
fn generate_iterator(range: std::ops::Range<usize>) -> impl Iterator<Item = usize> {
    range.into_iter()
}

#[cfg(feature = "rayon")]
fn generate_iterator(range: std::ops::Range<usize>) -> rayon::range::Iter<usize> {
    range.into_par_iter()
}

fn generate_distribution_preview(sample_size: u64) -> Result<()> {
    let (generator_data, generator_scripts) = load_generator_data()?;

    let (results, heritages, errors, elapsed) = {
        let results: HashMap<String, usize> = HashMap::new();
        let heritages: HashMap<String, usize> = HashMap::new();

        println!("Generating npcs...");
        let results = Mutex::new(results);
        let heritages = Mutex::new(heritages);
        let errors: Mutex<LinkedList<anyhow::Error>> = Mutex::new(LinkedList::new());

        use std::time::Instant;
        let now = Instant::now();

        let pb = indicatif::ProgressBar::new(sample_size);
        pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        ).unwrap(),);
        let sample_size = sample_size as usize;

        generate_iterator(0..sample_size)
            .progress_with(pb)
            .map(|_| {
                let thread_rng = rand::thread_rng();

                Generator::new(
                    rand::rngs::StdRng::from_rng(thread_rng).unwrap(),
                    generator_data.clone(),
                    generator_scripts.clone(),
                )
                .unwrap()
            })
            .for_each(|mut generator| {
                let npc_options = NpcOptions {
                    enable_flavor_text: false,
                    ..Default::default()
                };
                match generator.generate(&npc_options) {
                    Ok(result) => {
                        let ancestry = result.ancestry().unwrap();
                        let heritage = result.heritage();
                        let ancestry_name = ancestry.name();
                        let heritage_name = heritage
                            .as_ref()
                            .map(|x| x.name())
                            .unwrap_or(std::borrow::Cow::Borrowed("Normal"));
                        let mut results = results.lock().unwrap();
                        let mut heritages = heritages.lock().unwrap();

                        if !results.contains_key(ancestry_name.as_ref()) {
                            results.insert(ancestry_name.to_string(), 0);
                        }
                        *results.get_mut(ancestry_name.as_ref()).unwrap() += 1;

                        if !heritages.contains_key(heritage_name.as_ref()) {
                            heritages.insert(heritage_name.to_string(), 0);
                        }
                        *heritages.get_mut(heritage_name.as_ref()).unwrap() += 1;
                    }
                    Err(err) => {
                        let mut errors = errors.lock().unwrap();
                        errors.push_front(err.into())
                    }
                };
            });

        (
            results.into_inner().unwrap(),
            heritages.into_inner().unwrap(),
            errors.into_inner().unwrap(),
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

    let error_len = errors.len();
    if error_len == 0 {
        Ok(())
    } else {
        println!(
            "Got {} errors during generation ({}% failure rate):",
            error_len,
            (100.0 / sample_size as f64) * error_len as f64
        );
        errors.iter().for_each(|err| println!("{err}"));
        Err(anyhow!("Something went wrong :-)"))
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn generate_character() -> Result<(), Box<dyn Error>> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    let (generator_data, generator_scripts) = load_generator_data()?;
    eframe::run_native(
        "Character Generator",
        native_options,
        Box::new(move |cc| {
            Box::new(ui::UserInterface::new(
                cc,
                generator_data.clone(),
                generator_scripts,
            ))
        }),
    )?;
    Ok(())
}

fn show_open_zip_dialog() -> Result<Option<PathBuf>> {
    Ok(FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("Generator Source Data Package", &["zip"])
        .show_open_single_file()?)
}

fn load_generator_data_from_zip(
    zip_path: impl AsRef<Path>,
) -> Result<(Arc<GeneratorData>, Arc<GeneratorScripts>)> {
    let file = File::open(zip_path)?;

    let mut zip = zip::ZipArchive::new(file)?;

    fn read_from_zip<T, R>(zip: &mut zip::ZipArchive<R>, name: &str) -> Result<T>
    where
        T: for<'a> serde::Deserialize<'a>,
        R: io::Seek + io::Read,
    {
        let file = zip
            .by_name(name)
            .with_context(|| format!("Can't read {name} from zip file"))?;
        let data = io::read_to_string(file)?;
        ron::from_str(&data).with_context(|| format!("Can't deserialize {name}"))
    }

    let generator_data = {
        let ancestries: WeightMap<Ancestry> = read_from_zip(&mut zip, "ancestries.ron")?;
        let heritages: WeightMap<Heritage> = read_from_zip(&mut zip, "heritages.ron")?;
        let backgrounds: WeightMap<Background> = read_from_zip(&mut zip, "backgrounds.ron")?;
        let names: HashMap<Trait, HashMap<String, WeightMap<String>>> =
            read_from_zip(&mut zip, "names.ron")?;

        let archetypes: Vec<Archetype> = {
            let mut archetypes: Vec<Archetype> = read_from_zip(&mut zip, "archetypes.ron")?;
            archetypes.sort_by_key(|x| x.level());
            archetypes
        };

        Arc::new(npc_generator_core::generators::GeneratorData {
            ancestries,
            versitile_heritages: heritages,
            normal_heritage_weight: 0.8,
            backgrounds,
            heritages: Default::default(),
            names,
            archetypes,
        })
    };

    let generator_scripts = {
        Arc::new(GeneratorScripts {
            default_format_flavor_description_line_script: io::read_to_string(
                zip.by_name("scripts/default_format_flavor_description_line.glu")?,
            )?,
        })
    };

    Ok((generator_data, generator_scripts))
}

fn load_generator_data_from_directory(
    base_path: impl AsRef<Path>,
) -> Result<(Arc<GeneratorData>, Arc<GeneratorScripts>)> {
    let base_path = base_path.as_ref();
    info!("Loading generator data from {base_path:?}");
    let generator_data = {
        let mut data: PathBuf = base_path.into();
        data.push("data");

        fn read_file<T>(data: &Path, name: &str) -> Result<T>
        where
            T: for<'a> serde::Deserialize<'a>,
        {
            let mut path: PathBuf = data.into();
            path.push(name);
            info!("Trying to read {data:?}");
            let data = fs::read_to_string(&path)
                .with_context(|| format!("Can't read {name} from {path:?}"))?;
            ron::from_str(&data).with_context(|| format!("Can't deserialize {name}"))
        }

        let ancestries: WeightMap<Ancestry> = read_file(&data, "ancestries.ron")?;
        let heritages: WeightMap<Heritage> = read_file(&data, "heritages.ron")?;
        let backgrounds: WeightMap<Background> = read_file(&data, "backgrounds.ron")?;
        let names: HashMap<Trait, HashMap<String, WeightMap<String>>> =
            read_file(&data, "names.ron")?;

        let archetypes = {
            let mut archetypes: Vec<Archetype> = read_file(&data, "archetypes.ron")?;
            archetypes.sort_by_key(|x| x.level());
            archetypes
        };

        Arc::new(npc_generator_core::generators::GeneratorData {
            ancestries,
            versitile_heritages: heritages,
            normal_heritage_weight: 0.8,
            backgrounds,
            heritages: Default::default(),
            names,
            archetypes,
        })
    };

    info!("Reading scripts...");

    let generator_scripts = {
        let mut scripts: PathBuf = base_path.into();
        scripts.push("data");
        scripts.push("scripts");
        let scripts = scripts;

        Arc::new(GeneratorScripts {
            default_format_flavor_description_line_script: io::read_to_string({
                let mut path = scripts.clone();
                path.push("default_format_flavor_description_line.glu");
                info!("Reading script file {path:?}");
                File::open(path)?
            })?,
        })
    };

    Ok((generator_data, generator_scripts))
}

fn load_generator_data() -> Result<(Arc<GeneratorData>, Arc<GeneratorScripts>)> {
    let generator_data = std::env::current_dir().map(load_generator_data_from_directory);
    fn handle_err() -> Result<(Arc<GeneratorData>, Arc<GeneratorScripts>)> {
        let persistent_generator_config_path = {
            if let Some(path) = dirs::data_dir() {
                let mut path = path;
                path.push("pf2e_npc_generator");
                path.push("generator_data");
                if path.exists() && path.is_dir() {
                    Some(path)
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(path) = persistent_generator_config_path {
            match load_generator_data_from_directory(path) {
                Ok(data) => return Ok(data),
                Err(err) => error!("{}", err),
            }
        }

        if let Some(file_path) = show_open_zip_dialog()? {
            let data = load_generator_data_from_zip(&file_path)?;
            if let Some(path) = dirs::data_dir() {
                let mut path = path;
                path.push("pf2e_npc_generator");
                path.push("generator_data");
                path.push("data");
                match fs::create_dir_all(&path)
                    .and_then(|()| File::open(file_path))
                    .and_then(|file| Ok(zip::ZipArchive::new(file)?))
                    .and_then(|mut file| Ok(file.extract(path)?))
                {
                    Ok(()) => {}
                    Err(err) => {
                        error!("Couldn't persist generator data: {}", err);
                    }
                }
            }
            Ok(data)
        } else {
            Err(anyhow!("No zip file selected in file dialog"))
        }
    }
    match generator_data {
        Ok(Ok(data)) => Ok(data),

        Ok(Err(err)) => {
            error!("{}", err);
            handle_err()
        }
        Err(err) => {
            error!("{}", err);
            handle_err()
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    #[default]
    Interactive,
    Statistics,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "interactive")]
    mode: Mode,

    #[arg(short, long, default_value_t = 2_000_000)]
    sample_size: u64,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    let args = Args::parse();
    match args.mode {
        Mode::Statistics => {
            generate_distribution_preview(args.sample_size)?;
            Ok(())
        }
        Mode::Interactive => generate_character(),
    }
}
