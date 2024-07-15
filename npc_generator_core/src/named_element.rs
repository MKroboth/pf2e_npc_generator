use std::borrow::Cow;

use crate::Trait;

pub trait NamedElement {
    fn traits(&self) -> &[Trait];
    fn name(&self) -> Cow<str>;
    fn formatted_name(&self) -> Cow<str>;
}
