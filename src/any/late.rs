use std::{
  error::Error,
  fmt::{self, Display, Formatter},
};

#[derive(Copy, Clone, Debug)]
pub struct NotInitYetError;

impl Display for NotInitYetError {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    write!(formatter, "Value has not been initialized yet!")
  }
}

impl Error for NotInitYetError {}

#[derive(Copy, Clone, Debug)]
pub struct AlreadyInitError;

impl Display for AlreadyInitError {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    write!(formatter, "Value has already been initialized!")
  }
}

impl Error for AlreadyInitError {}

/// It is a holder that allows late initialization of the value being held.
#[derive(Copy, Clone, Debug, Default)]
pub struct Late<T>(Option<T>);

impl<T> Late<T> {
  pub const fn new() -> Self {
    Self(None)
  }

  pub fn get_value(&self) -> Result<&T, NotInitYetError> {
    if let Some(value) = &self.0 {
      Ok(value)
    } else {
      Err(NotInitYetError)
    }
  }

  pub fn set_value(&mut self, value: T) -> Result<(), AlreadyInitError> {
    if self.0.is_none() {
      self.0 = Some(value);
      Ok(())
    } else {
      Err(AlreadyInitError)
    }
  }
}
