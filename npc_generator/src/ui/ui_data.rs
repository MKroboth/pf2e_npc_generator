use std::sync::Arc;

use npc_generator_core::{weight_presets::WeightPreset, NpcOptions};
use serde::{Deserialize, Serialize};

use super::GeneratorFormat;

#[derive(Default, Serialize, Deserialize)]
pub struct UIData {
    pub generated_text_format: GeneratorFormat,
    pub generated_text: String,
    pub npc_options: NpcOptions,
    pub use_archetype: bool,
    pub current_weight_preset: Option<Arc<WeightPreset>>,
}
