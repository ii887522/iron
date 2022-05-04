use std::ops::Deref;

#[derive(Debug)]
pub struct Immutable<T> {
  value: T,
}

impl<T> Immutable<T> {
  pub const fn new(value: T) -> Self {
    Self { value }
  }
}

impl<T> Deref for Immutable<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

impl<T> From<T> for Immutable<T> {
  fn from(value: T) -> Self {
    Self { value }
  }
}
