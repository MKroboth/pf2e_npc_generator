use crate::Trait;

pub trait NamedElement {
    fn traits(&self) -> &[Trait];
    fn name(&self) -> String;
}
