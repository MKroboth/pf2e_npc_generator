use gluon::{
    base::types::ArcType,
    vm::thread::{ActiveThread, Trace},
    Thread, ThreadExt,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Range};

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
impl gluon::vm::api::VmType for AgeRange {
    type Type = Self;
    fn make_type(thread: &Thread) -> ArcType {
        thread
            .find_type_info("npc_generator.core.AgeRange")
            .unwrap()
            .into_type()
    }
}
impl<'vm, 'value> gluon::vm::api::Pushable<'vm> for AgeRange {
    fn vm_push(self, context: &mut ActiveThread<'vm>) -> gluon::vm::Result<()> {
        gluon::vm::api::ser::Ser(self).vm_push(context)
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
