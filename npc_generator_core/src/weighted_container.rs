use rand::distributions::uniform::SampleUniform;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightedElement<T: Clone> {
    pub weight: i64,
    pub element: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightedHeapArray<T: Clone>(pub Arc<[WeightedElement<T>]>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightedVector<T: Clone>(pub Vec<WeightedElement<T>>);

pub trait WeightedContainer {
    type Element;
    fn weights(&self) -> Vec<i64>;
    fn weighted_distribution(
        &self,
    ) -> Result<rand::distributions::WeightedIndex<i64>, rand::distributions::WeightedError> {
        rand::distributions::WeightedIndex::new(self.weights())
    }

    fn elements(&self) -> Vec<Self::Element>;
}

impl<T: Clone> WeightedHeapArray<T> {
    pub fn map_elements<F, Result>(&self, mut func: F) -> WeightedHeapArray<Result>
    where
        F: FnMut(T) -> Result,
        Result: Clone,
    {
        WeightedHeapArray(
            self.0
                .iter()
                .map(|x| WeightedElement {
                    element: func(x.element.clone()),
                    weight: x.weight,
                })
                .collect::<Vec<WeightedElement<Result>>>()
                .into_boxed_slice()
                .into(),
        )
    }
}

impl<T: Clone> WeightedContainer for WeightedHeapArray<T> {
    type Element = T;
    fn weights(&self) -> Vec<i64> {
        self.0.iter().map(|element| element.weight).collect()
    }
    fn elements(&self) -> Vec<T> {
        self.0
            .iter()
            .map(|element| element.element.clone())
            .collect()
    }
}

impl<T: Clone> WeightedVector<T> {
    pub fn map_elements<F, Result>(&self, mut func: F) -> WeightedVector<Result>
    where
        F: FnMut(T) -> Result,
        Result: Clone,
    {
        WeightedVector(
            self.0
                .iter()
                .map(|x| WeightedElement {
                    element: func(x.element.clone()),
                    weight: x.weight,
                })
                .collect::<Vec<WeightedElement<Result>>>(),
        )
    }
}

impl<T: Clone> WeightedContainer for WeightedVector<T> {
    type Element = T;
    fn weights(&self) -> Vec<i64> {
        self.0.iter().map(|element| element.weight).collect()
    }
    fn elements(&self) -> Vec<T> {
        self.0
            .iter()
            .map(|element| element.element.clone())
            .collect()
    }
}
// vim: ts=2 sts=2 sw=2 et cc=79
