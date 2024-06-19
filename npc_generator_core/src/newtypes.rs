use std::borrow::Borrow;
use std::collections::HashMap;

use rand::distributions::uniform::SampleUniform;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

type Weight = u32;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeightMap<T: std::hash::Hash + Eq>(pub HashMap<T, Weight>);

use std::collections::hash_map::{
    IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut,
};
impl<K: Hash + Eq> WeightMap<K> {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    #[inline]
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&Weight>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get(k)
    }

    pub fn keys(&self) -> Keys<'_, K, Weight> {
        self.0.keys()
    }
    #[inline]
    pub fn into_keys(self) -> IntoKeys<K, Weight> {
        self.0.into_keys()
    }
    pub fn values(&self) -> Values<'_, K, Weight> {
        self.0.values()
    }
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, Weight> {
        self.0.values_mut()
    }
    #[inline]
    pub fn into_values(self) -> IntoValues<K, Weight> {
        self.0.into_values()
    }
    pub fn iter(&self) -> Iter<'_, K, Weight> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, Weight> {
        self.0.iter_mut()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn split_weights_with_modifications<F>(
        &self,
        modify_weight_function: F,
    ) -> Result<
        (Vec<&K>, rand::distributions::WeightedIndex<Weight>),
        rand::distributions::WeightedError,
    >
    where
        Weight:
            SampleUniform + PartialOrd + Default + Clone + for<'a> core::ops::AddAssign<&'a Weight>,
        F: Fn(&K) -> Option<Weight>,
    {
        let mut weights: Vec<Weight> = Vec::new();
        let mut values = Vec::new();

        for (value, weight) in self.iter() {
            values.push(value);
            if let Some(new_weight) = modify_weight_function(&value) {
                weights.push(new_weight);
            } else {
                weights.push(*weight);
            }
        }

        Ok((values, rand::distributions::WeightedIndex::new(weights)?))
    }

    pub fn split_weights(
        &self,
    ) -> Result<
        (Vec<&K>, rand::distributions::WeightedIndex<Weight>),
        rand::distributions::WeightedError,
    > {
        self.split_weights_with_modifications(|_| None)
    }
}

impl<T: Hash + Eq> From<HashMap<T, Weight>> for WeightMap<T> {
    fn from(value: HashMap<T, Weight>) -> Self {
        Self(value)
    }
}
impl<K: Hash + Eq> Default for WeightMap<K> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K> Extend<(K, Weight)> for WeightMap<K>
where
    K: Eq + Hash,
{
    #[inline]
    fn extend<T: IntoIterator<Item = (K, Weight)>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl<'a, K> Extend<(&'a K, &'a Weight)> for WeightMap<K>
where
    K: Eq + Hash + Copy,
{
    #[inline]
    fn extend<T: IntoIterator<Item = (&'a K, &'a Weight)>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl<K> FromIterator<(K, Weight)> for WeightMap<K>
where
    K: Eq + Hash,
{
    fn from_iter<T: IntoIterator<Item = (K, Weight)>>(iter: T) -> WeightMap<K> {
        let mut map = WeightMap::new();
        map.extend(iter);
        map
    }
}

impl<'a, K: Eq + Hash> IntoIterator for &'a WeightMap<K> {
    type Item = (&'a K, &'a Weight);
    type IntoIter = Iter<'a, K, Weight>;

    #[inline]
    fn into_iter(self) -> Iter<'a, K, Weight> {
        self.0.iter()
    }
}
impl<'a, K: Eq + Hash> IntoIterator for &'a mut WeightMap<K> {
    type Item = (&'a K, &'a mut Weight);
    type IntoIter = IterMut<'a, K, Weight>;

    #[inline]
    fn into_iter(self) -> IterMut<'a, K, Weight> {
        self.iter_mut()
    }
}

impl<K: Eq + Hash> IntoIterator for WeightMap<K> {
    type Item = (K, Weight);
    type IntoIter = IntoIter<K, Weight>;

    /// Creates a consuming iterator, that is, one that moves each key-value
    /// pair out of the map in arbitrary order. The map cannot be used after
    /// calling this.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let map = HashMap::from([
    ///     ("a", 1),
    ///     ("b", 2),
    ///     ("c", 3),
    /// ]);
    ///
    /// // Not possible with .iter()
    /// let vec: Vec<(&str, i32)> = map.into_iter().collect();
    /// ```
    #[inline]
    fn into_iter(self) -> IntoIter<K, Weight> {
        self.0.into_iter()
    }
}
