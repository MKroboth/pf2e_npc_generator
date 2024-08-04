use std::{borrow::Cow, fmt::Display, sync::Arc};

use npc_generator_core::{
    generators::{Generator, GeneratorData, GeneratorScripts},
    weight_presets::WeightPreset,
    NamedElement, NpcOptions, Statblock,
};
use rand::SeedableRng;
use serde::{Deserialize, Serialize};

pub struct UserInterface {
    generator: Generator<rand::rngs::StdRng>,
    data: UIData,
    resulting_statblock: Option<Statblock>,
    weight_presets: Arc<[Arc<WeightPreset>]>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
enum GeneratorFormat {
    Flavor,
    PF2EStats,
}

impl Display for GeneratorFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GeneratorFormat::Flavor => "Flavor",
                GeneratorFormat::PF2EStats => "pf2e-stats",
            }
        )
    }
}

impl Default for GeneratorFormat {
    fn default() -> Self {
        Self::Flavor
    }
}

#[derive(Default, Serialize, Deserialize)]
struct UIData {
    generated_text_format: GeneratorFormat,
    generated_text: String,
    npc_options: NpcOptions,
    use_archetype: bool,
    current_weight_preset: Option<Arc<WeightPreset>>,
}

impl UserInterface {
    /// Called once before the first frame.
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        generator_data: Arc<GeneratorData>,
        generator_scripts: Arc<GeneratorScripts>,
        weight_presets: impl AsRef<[Arc<WeightPreset>]>,
    ) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let data: UIData = UIData {
            npc_options: NpcOptions {
                enable_flavor_text: true,
                ..Default::default()
            },
            ..Default::default()
        };

        UserInterface {
            data,
            generator: Generator::new(
                rand::rngs::StdRng::from_rng(rand::thread_rng()).unwrap(),
                generator_data,
                generator_scripts,
            )
            .unwrap(),
            resulting_statblock: Default::default(),
            weight_presets: weight_presets.as_ref().into(),
        }
    }
}

impl eframe::App for UserInterface {
    /// Called by the frame work to save state before shutdown.
    //fn save(&mut self, storage: &mut dyn eframe::Storage) {}

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Character Generator");

            ui.horizontal(|ui| {
                if ui
                    .add_enabled(
                        !self.data.use_archetype || self.data.npc_options.archetype.is_some(),
                        egui::Button::new("Generate"),
                    )
                    .clicked()
                {
                    self.resulting_statblock = match self.generator.generate(
                        &self.data.npc_options,
                        self.data.current_weight_preset.clone(),
                    ) {
                        Ok(x) => Some(x),
                        Err(_err) => {
                            // todo pop-up error dialog
                            None
                        }
                    }
                }
                egui::ComboBox::from_label("Generator Mode")
                    .selected_text(format!("{}", &self.data.generated_text_format))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.data.generated_text_format,
                            GeneratorFormat::Flavor,
                            GeneratorFormat::Flavor.to_string(),
                        );
                        ui.selectable_value(
                            &mut self.data.generated_text_format,
                            GeneratorFormat::PF2EStats,
                            GeneratorFormat::PF2EStats.to_string(),
                        );
                    });
                if ui
                    .checkbox(&mut self.data.use_archetype, "Use archetype")
                    .changed
                {
                    if self.data.use_archetype {
                        self.data.npc_options.background = None;
                    } else {
                        self.data.npc_options.archetype = None;
                    }
                }

                egui::ComboBox::from_label("Weight Preset")
                    .selected_text(format!(
                        "{}",
                        &self
                            .data
                            .current_weight_preset
                            .as_ref()
                            .map(|x| x.name())
                            .unwrap_or("Default Weights".into())
                    ))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.data.current_weight_preset,
                            None,
                            "Default Weights",
                        );
                        for (value, name) in self.weight_presets.iter().map(|x| (x, x.name())) {
                            ui.selectable_value(
                                &mut self.data.current_weight_preset,
                                Some(value.clone()),
                                name,
                            );
                        }
                    });
                ui.label("?").on_hover_ui(|ui| {
                    ui.label(format!(
                        "Your presets are stored in {:?}",
                        dirs::config_dir().map(|path| {
                            let mut path = path.clone();
                            path.push("pf2e_npc_generator");
                            path.push("weight_presets");
                            path
                        })
                    ));
                });
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    egui::ComboBox::from_label("Ancestry")
                        .selected_text(
                            match self
                                .data
                                .npc_options
                                .ancestry
                                .as_ref()
                                .map(|x| String::from(x.name().as_ref()))
                            {
                                None => String::from("Generate"),
                                Some(x) => x,
                            }
                            .to_string(),
                        )
                        .show_ui(ui, |ui| {
                            if ui
                                .selectable_value(
                                    &mut self.data.npc_options.ancestry,
                                    None,
                                    "Generate",
                                )
                                .clicked
                            {
                                self.data.npc_options.sex = None;
                            }
                            for ancestry in &self.generator.data.ancestries {
                                if ui
                                    .selectable_value(
                                        &mut self.data.npc_options.ancestry,
                                        Some(ancestry.0.clone()),
                                        ancestry.0.name().clone(),
                                    )
                                    .clicked
                                {
                                    self.data.npc_options.sex = None;
                                }
                            }
                        });
                    egui::ComboBox::from_label("Heritage")
                        .selected_text(
                            match self.data.npc_options.heritage.as_ref() {
                                None => String::from("Generate"),
                                Some(None) => String::from("Normal Person"),
                                Some(Some(x)) => x.name().to_string(),
                            }
                            .to_string(),
                        )
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.data.npc_options.heritage,
                                None,
                                "Generate",
                            );

                            ui.selectable_value(
                                &mut self.data.npc_options.heritage,
                                Some(None),
                                "Normal Person",
                            );
                            for heritage in &self.generator.data.versitile_heritages {
                                ui.selectable_value(
                                    &mut self.data.npc_options.heritage,
                                    Some(Some(heritage.0.clone())),
                                    heritage.0.name(),
                                );
                            }
                        });
                    if self.data.use_archetype {
                        egui::ComboBox::from_label("Archetype")
                            .selected_text(format!(
                                "{}",
                                match self.data.npc_options.archetype {
                                    Some(ref x) => x.name(),
                                    None => Cow::Borrowed("No archetype"),
                                }
                            ))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.data.npc_options.archetype,
                                    None,
                                    "No archetype",
                                );

                                for archetype in &self.generator.data.archetypes {
                                    let name = archetype.name();
                                    let level = archetype.level();
                                    ui.selectable_value(
                                        &mut self.data.npc_options.archetype,
                                        Some(archetype.clone()),
                                        format!("{name} ({level})"),
                                    );
                                }
                            });
                    } else {
                        egui::ComboBox::from_label("Background")
                            .selected_text(
                                match self
                                    .data
                                    .npc_options
                                    .background
                                    .as_ref()
                                    .map(|x| String::from(x.name()))
                                {
                                    None => String::from("Generate"),
                                    Some(x) => x,
                                }
                                .to_string(),
                            )
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.data.npc_options.background,
                                    None,
                                    "Generate",
                                );
                                for background in &self.generator.data.backgrounds {
                                    ui.selectable_value(
                                        &mut self.data.npc_options.background,
                                        Some(background.0.clone()),
                                        String::from(background.0.name()),
                                    );
                                }
                            });
                    }
                });
                ui.separator();
                ui.vertical(|ui| {
                    egui::ComboBox::from_label("Age Range")
                        .selected_text(
                            match self
                                .data
                                .npc_options
                                .age_range
                                .as_ref()
                                .map(|x| x.to_string())
                            {
                                None => String::from("Generate"),
                                Some(x) => x,
                            }
                            .to_string(),
                        )
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.data.npc_options.age_range,
                                None,
                                "Generate",
                            );
                            ui.selectable_value(
                                &mut self.data.npc_options.age_range,
                                Some(npc_generator_core::AgeRange::Infant),
                                "Infant",
                            );

                            ui.selectable_value(
                                &mut self.data.npc_options.age_range,
                                Some(npc_generator_core::AgeRange::Child),
                                "Child",
                            );

                            ui.selectable_value(
                                &mut self.data.npc_options.age_range,
                                Some(npc_generator_core::AgeRange::Youth),
                                "Youth",
                            );

                            ui.selectable_value(
                                &mut self.data.npc_options.age_range,
                                Some(npc_generator_core::AgeRange::Adult),
                                "Adult",
                            );

                            ui.selectable_value(
                                &mut self.data.npc_options.age_range,
                                Some(npc_generator_core::AgeRange::Old),
                                "Old",
                            );

                            ui.selectable_value(
                                &mut self.data.npc_options.age_range,
                                Some(npc_generator_core::AgeRange::Venerable),
                                "Venerable",
                            );
                        });
                    ui.add_enabled_ui(
                        self.data.npc_options.ancestry.is_some()
                            && !self
                                .data
                                .npc_options
                                .ancestry
                                .as_ref()
                                .unwrap()
                                .is_asexual(),
                        |ui| {
                            egui::ComboBox::from_label("Sex")
                                .selected_text(
                                    match self.data.npc_options.sex.as_ref().map(|x| x.to_string())
                                    {
                                        None => String::from("Generate"),
                                        Some(x) => x,
                                    }
                                    .to_string(),
                                )
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.data.npc_options.sex,
                                        None,
                                        "Generate",
                                    );

                                    ui.selectable_value(
                                        &mut self.data.npc_options.sex,
                                        Some("male".to_string()),
                                        "Male",
                                    );

                                    ui.selectable_value(
                                        &mut self.data.npc_options.sex,
                                        Some("female".to_string()),
                                        "Female",
                                    );
                                });
                        },
                    );
                });

                ui.separator();
            });
            if let Some(ref resulting_statblock) = self.resulting_statblock {
                ui.add_sized(
                    egui::vec2(ui.available_width(), ui.available_height()),
                    egui::TextEdit::multiline(&mut match self.data.generated_text_format {
                        GeneratorFormat::Flavor => resulting_statblock.flavor().to_string(),
                        GeneratorFormat::PF2EStats => {
                            resulting_statblock.as_pf2e_stats().to_string()
                        }
                    }),
                );
            }
        });
    }
}
