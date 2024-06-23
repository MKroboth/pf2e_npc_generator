use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ValidAncestries {
    Any,
    AllOf(Vec<String>),
    Only(Vec<String>),
}
