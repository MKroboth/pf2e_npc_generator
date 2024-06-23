use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct DiceFormula(pub i8, pub Die);
impl DiceFormula {
    pub fn roll(&self, mut rng: &mut impl Rng) -> i32 {
        let mut result: i32 = 0;
        for _ in 0..self.0 {
            result += self.1.roll(&mut rng) as i32;
        }
        result
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

impl Die {
    pub fn roll(&self, rng: &mut impl Rng) -> i8 {
        rng.gen_range(match self {
            Die::D4 => 1..=4,
            Die::D6 => 1..=6,
            Die::D8 => 1..=8,
            Die::D10 => 1..=10,
            Die::D12 => 1..=12,
            Die::D20 => 1..=20,
        })
    }
}
