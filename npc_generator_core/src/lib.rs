use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub mod generators;
mod newtypes;

mod heritage;
pub use heritage::*;
mod ancestry;
pub use ancestry::*;
mod statblock;
pub use newtypes::*;
pub use statblock::*;
mod ability;
pub use ability::*;
mod traits;
pub use traits::*;
mod sense;
pub use sense::*;
mod named_element;
pub use named_element::*;
mod language;
pub use language::*;
mod size;
pub use size::*;
mod dice;
pub use dice::*;
mod age_range;
pub use age_range::*;
mod mutation;
pub use mutation::*;
mod ancestries;
pub use ancestries::*;
mod skills;
pub use skills::*;
mod proficiencies;
pub use proficiencies::*;
mod background;
pub use background::*;
mod archetype;
pub use archetype::*;
mod npc_options;
pub use npc_options::*;
pub mod formats;

#[macro_export]
macro_rules! traits {
    [] => (vec![]);
    [$($literal:literal),+ $(,)?] => ( vec![ $(Trait::new($literal)),+ ] );

}
#[macro_export]
macro_rules! language {
    ($literal:literal $($traits:literal),* $(,)?) => (
    Language::new(vec![$($traits),*], $literal) );

}
