use std::{borrow::Cow, fmt::Display, sync::Arc};

use npc_generator_core::{
    generators::{Generator, GeneratorData, GeneratorScripts},
    weight_presets::WeightPreset,
    NamedElement, NpcOptions, Statblock,
};
use rand::SeedableRng;
mod generator_format;
use generator_format::*;
mod ui_data;
use ui_data::UIData;

pub struct UserInterface {
    generator: Generator<rand::rngs::StdRng>,
    data: UIData,
    resulting_statblock: Option<Statblock>,
    weight_presets: Arc<[Arc<WeightPreset>]>,
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
    #[inline]
    fn show_top_panel(ctx: &egui::Context) {
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
    }

    #[inline]
    fn ui_generate_button(&mut self, ui: &mut egui::Ui) {
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
    }

    #[inline]
    fn ui_generator_mode_combobox(&mut self, ui: &mut egui::Ui) {
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
    }

    #[inline]
    fn ui_use_archetype_checkbox(&mut self, ui: &mut egui::Ui) {
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
    }

    #[inline]

    fn ui_weight_preset_combobox(&mut self, ui: &mut egui::Ui) {
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
    }

    #[inline]
    fn ui_ancestry_combobox(&mut self, ui: &mut egui::Ui) {
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
                    .selectable_value(&mut self.data.npc_options.ancestry, None, "Generate")
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
    }
    #[inline]
    fn ui_heritage_combobox(&mut self, ui: &mut egui::Ui) {
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
                ui.selectable_value(&mut self.data.npc_options.heritage, None, "Generate");

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
    }

    #[inline]
    fn ui_archetype_combobox(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_label("Archetype")
            .selected_text(format!(
                "{}",
                match self.data.npc_options.archetype {
                    Some(ref x) => x.name(),
                    None => Cow::Borrowed("No archetype"),
                }
            ))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.data.npc_options.archetype, None, "No archetype");

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
    }

    #[inline]
    fn ui_background_combobox(&mut self, ui: &mut egui::Ui) {
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
                ui.selectable_value(&mut self.data.npc_options.background, None, "Generate");
                for background in &self.generator.data.backgrounds {
                    ui.selectable_value(
                        &mut self.data.npc_options.background,
                        Some(background.0.clone()),
                        String::from(background.0.name()),
                    );
                }
            });
    }

    #[inline]
    fn ui_age_range_combobox(&mut self, ui: &mut egui::Ui) {
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
                ui.selectable_value(&mut self.data.npc_options.age_range, None, "Generate");
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
    }

    #[inline]
    fn ui_sex_combobox(&mut self, ui: &mut egui::Ui) {
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
                        match self.data.npc_options.sex.as_ref().map(|x| x.to_string()) {
                            None => String::from("Generate"),
                            Some(x) => x,
                        }
                        .to_string(),
                    )
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.data.npc_options.sex, None, "Generate");

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
    }

    #[inline]
    fn ui_generated_text_textedit(&mut self, ui: &mut egui::Ui) {
        if let Some(ref resulting_statblock) = self.resulting_statblock {
            ui.add_sized(
                egui::vec2(ui.available_width(), ui.available_height()),
                egui::TextEdit::multiline(&mut match self.data.generated_text_format {
                    GeneratorFormat::Flavor => resulting_statblock.flavor().to_string(),
                    GeneratorFormat::PF2EStats => resulting_statblock.as_pf2e_stats().to_string(),
                }),
            );
        }
    }

    #[inline]
    fn show_center_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Character Generator");

            ui.horizontal(|ui| {
                self.ui_generate_button(ui);
                self.ui_generator_mode_combobox(ui);
                self.ui_use_archetype_checkbox(ui);
                self.ui_weight_preset_combobox(ui);
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    self.ui_ancestry_combobox(ui);
                    self.ui_heritage_combobox(ui);
                    if self.data.use_archetype {
                        self.ui_archetype_combobox(ui);
                    } else {
                        self.ui_background_combobox(ui);
                    }
                });
                ui.separator();
                ui.vertical(|ui| {
                    self.ui_age_range_combobox(ui);
                    self.ui_sex_combobox(ui);
                });

                ui.separator();
            });

            self.ui_generated_text_textedit(ui);
            self.ui_generated_text_textedit(ui);
        });
    }
}
impl eframe::App for UserInterface {
    /// Called by the frame work to save state before shutdown.
    //fn save(&mut self, storage: &mut dyn eframe::Storage) {}

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        UserInterface::show_top_panel(ctx);
        UserInterface::show_center_panel(self, ctx);
    }
}
