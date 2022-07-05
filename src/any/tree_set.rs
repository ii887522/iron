use crate::TreeMap;
use std::borrow::Cow;

#[derive(Copy, Clone, Debug)]
pub struct Arg(usize);

impl From<()> for Arg {
  fn from(_: ()) -> Self {
    Self(32)
  }
}

impl From<usize> for Arg {
  fn from(initial_capacity: usize) -> Self {
    Self(initial_capacity)
  }
}

#[derive(Debug)]
pub struct TreeSet<T>(TreeMap<T, ()>);

impl<T: Default + PartialOrd, Iter: Iterator<Item = T>> From<Iter> for TreeSet<T> {
  fn from(iter: Iter) -> Self {
    Self(TreeMap::from(iter.map(|value| (value, ()))))
  }
}

impl<T> TreeSet<T> {
  pub fn new(arg: impl Into<Arg>) -> Self {
    let Arg(initial_capacity) = arg.into();
    Self(TreeMap::new(initial_capacity))
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn to_slice(&self) -> Cow<[&T]> {
    self.0.to_slice().iter().map(|&(key, _)| key).collect()
  }

  pub fn clear(&mut self) {
    self.0.clear();
  }
}

impl<T: PartialOrd> TreeSet<T> {
  pub fn has(&self, value: T) -> bool {
    self.0.has(value)
  }

  pub fn remove(&mut self, value: T) {
    self.0.remove(value);
  }

  pub fn min(&self) -> Option<&T> {
    self.0.min().map(|(key, _)| key)
  }

  pub fn max(&self) -> Option<&T> {
    self.0.max().map(|(key, _)| key)
  }
}

impl<T: Default + PartialOrd> TreeSet<T> {
  pub fn bulk_put(&mut self, iter: impl Iterator<Item = T>) {
    self.0.bulk_put(iter.map(|key| (key, ())))
  }

  pub fn put(&mut self, value: T) {
    self.0.put(value, ())
  }
}
