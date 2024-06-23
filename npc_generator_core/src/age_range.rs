use std::{fmt::Display, ops::Range};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgeRanges {
    pub child: u64,
    pub youth: u64,
    pub adulthood: u64,
    pub middle_age: u64,
    pub old: u64,
    pub venerable: u64,
    pub lifespan: u64,
}

impl AgeRanges {
    pub fn get(&self, range: &AgeRange) -> Range<u64> {
        match range {
            AgeRange::Infant => 0..self.child,
            AgeRange::Child => self.child..self.youth,
            AgeRange::Youth => self.youth..self.adulthood,
            AgeRange::Adult => self.adulthood..self.middle_age,
            AgeRange::MiddleAged => self.middle_age..self.old,
            AgeRange::Old => self.old..self.venerable,
            AgeRange::Venerable => self.venerable..self.lifespan + 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum AgeRange {
    Infant,
    Child,
    Youth,
    Adult,
    MiddleAged,
    Old,
    Venerable,
}

impl Display for AgeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AgeRange::Infant => "Infant",
                AgeRange::Child => "Child",
                AgeRange::Youth => "Youth",
                AgeRange::Adult => "Adult",
                AgeRange::MiddleAged => "MiddleAged",
                AgeRange::Old => "Old",
                AgeRange::Venerable => "Venerable",
            }
        )
    }
}

impl Default for AgeRange {
    fn default() -> Self {
        AgeRange::Adult
    }
}
