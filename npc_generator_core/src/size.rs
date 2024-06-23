use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Garganutan,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Tiny => "Tiny",
                Self::Small => "Small",
                Self::Medium => "Medium",
                Self::Large => "Large",
                Self::Huge => "Huge",
                Self::Garganutan => "Gargantuan",
            }
        )
    }
}
