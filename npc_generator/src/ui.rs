use std::sync::Arc;

use npc_generator_core::{
    generators::{Generator, GeneratorData},
    NamedElement, NpcOptions,
};
use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};

pub struct UserInterface {
    generator: Generator<ThreadRng>,
    data: UIData,
}

#[derive(Default, Serialize, Deserialize)]
struct UIData {
    label: String,
    generated_text: String,
    npc_options: NpcOptions,
}

impl UserInterface {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, generator_data: Arc<GeneratorData>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let data: UIData = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        UserInterface {
            data,
            generator: Generator::new(ThreadRng::default(), generator_data).unwrap(),
        }
    }
}

impl eframe::App for UserInterface {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.data);
    }

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

            if ui.button("Generate").clicked() {
                let result = self.generator.generate(&self.data.npc_options);
                self.data.generated_text = result.flavor.to_string();
            }
            ui.separator();
            ui.vertical(|ui| {
                egui::ComboBox::from_label("Ancestry")
                    .selected_text(format!(
                        "{}",
                        match self
                            .data
                            .npc_options
                            .ancestry
                            .as_ref()
                            .map(|x| String::from(&x.name))
                        {
                            None => String::from("Generate"),
                            Some(x) => x,
                        }
                    ))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.data.npc_options.ancestry, None, "Generate");
                        for ancestry in &self.generator.data.ancestries {
                            ui.selectable_value(
                                &mut self.data.npc_options.ancestry,
                                Some(ancestry.0.clone()),
                                ancestry.0.name.clone(),
                            );
                        }
                    });
                egui::ComboBox::from_label("Heritage")
                    .selected_text(format!(
                        "{}",
                        match self.data.npc_options.heritage.as_ref() {
                            None => String::from("Generate"),
                            Some(None) => String::from("Normal Person"),
                            Some(Some(x)) => String::from(&x.name),
                        }
                    ))
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
                                heritage.0.name.clone(),
                            );
                        }
                    });
                egui::ComboBox::from_label("Background")
                    .selected_text(format!(
                        "{}",
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
                    ))
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
            });

            ui.text_edit_multiline(&mut self.data.generated_text);
        });
    }
}
