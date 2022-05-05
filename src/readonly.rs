use std::ops::Deref;

#[derive(Copy, Clone, Debug)]
pub struct Readonly<T> {
  value: T,
}

impl<T> Readonly<T> {
  pub const fn new(value: T) -> Self {
    Readonly { value }
  }
}

impl<T> Deref for Readonly<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

impl<T> From<T> for Readonly<T> {
  fn from(value: T) -> Self {
    Readonly { value }
  }
}
