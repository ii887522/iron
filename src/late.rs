use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct NotInitYetError;

impl Display for NotInitYetError {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    write!(formatter, "Value has not been initialized yet!")
  }
}

impl Error for NotInitYetError {}

#[derive(Debug)]
pub struct AlreadyInitError;

impl Display for AlreadyInitError {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    write!(formatter, "Value has already been initialized!")
  }
}

impl Error for AlreadyInitError {}

/// It is a holder that allows late initialization of the value being held.
#[derive(Debug, Default)]
pub struct Late<T> {
  value: Option<T>,
}

impl<T> Late<T> {
  pub const fn new() -> Self {
    Self { value: None }
  }

  pub fn get_value(&self) -> Result<&T, NotInitYetError> {
    if self.value.is_none() {
      Err(NotInitYetError)
    } else {
      Ok(self.value.as_ref().unwrap())
    }
  }

  pub fn set_value(&mut self, value: T) -> Result<(), AlreadyInitError> {
    if self.value.is_some() {
      Err(AlreadyInitError)
    } else {
      self.value = Some(value);
      Ok(())
    }
  }
}
