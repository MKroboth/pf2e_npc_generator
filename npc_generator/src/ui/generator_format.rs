use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum GeneratorFormat {
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
