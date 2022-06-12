use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct Late<T>(Option<T>);

impl<T> Late<T> {
  pub const fn new() -> Self {
    Self(None)
  }

  pub fn set(&mut self, value: T) {
    if self.0.is_none() {
      self.0 = Some(value);
    } else {
      panic!("Value has already been initialized!");
    }
  }
}

impl<T> Deref for Late<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self
      .0
      .as_ref()
      .expect("Value has not been initialized yet!")
  }
}

impl<T> DerefMut for Late<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self
      .0
      .as_mut()
      .expect("Value has not been initialized yet!")
  }
}
