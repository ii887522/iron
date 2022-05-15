//! A holder that allows user to initialize the value in any methods other than constructors.

use std::{
  error::Error,
  fmt::{self, Display, Formatter},
};

/// An error that will occured if trying to get a value from an uninitialized holder.
#[derive(Copy, Clone, Debug)]
pub struct NotInitYetError;

impl Display for NotInitYetError {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    write!(formatter, "Value has not been initialized yet!")
  }
}

impl Error for NotInitYetError {}

/// An error that will occured if trying to initialize the same holder again.
#[derive(Copy, Clone, Debug)]
pub struct AlreadyInitError;

impl Display for AlreadyInitError {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    write!(formatter, "Value has already been initialized!")
  }
}

impl Error for AlreadyInitError {}

/// A holder that allows user to initialize the value in any methods other than constructors.
#[derive(Copy, Clone, Debug, Default)]
pub struct Late<T>(Option<T>);

impl<T> Late<T> {
  /// Constructs a new holder that has not been initialized yet.
  pub const fn new() -> Self {
    Self(None)
  }

  /// Retrieves the initialized value from this holder.
  ///
  /// This method will return a `NotInitYetError` if the user has not explicitly give this holder a value to initialze
  /// this holder.
  ///
  /// # Examples
  ///
  /// ```
  /// use iron_ingot::Late;
  ///
  /// let mut late_value = Late::new();
  ///
  /// // The late_value has not been initialized yet, so cannot get value from it.
  /// assert!(late_value.get_value().is_err());
  ///
  /// // Initialize the late_value with 0.
  /// assert!(late_value.set_value(0).is_ok());
  ///
  /// // late_value has already been initialized, so can get value from it.
  /// assert!(late_value.get_value().is_ok());
  /// assert_eq!(late_value.get_value().unwrap_or(&-1), &0);
  /// ```
  pub fn get_value(&self) -> Result<&T, NotInitYetError> {
    if let Some(value) = &self.0 {
      Ok(value)
    } else {
      Err(NotInitYetError)
    }
  }

  /// Intialize this holder with the `value` given.
  ///
  /// This method will return an `AlreadyInitError` if `set_value` method for this holder has been called.
  ///
  /// # Examples
  /// ```
  /// use iron_ingot::Late;
  ///
  /// let mut late_value = Late::new();
  ///
  /// // Initialize the `late_value` with 0.
  /// assert!(late_value.set_value(0).is_ok());
  ///
  /// assert!(late_value.get_value().is_ok());
  /// assert_eq!(late_value.get_value().unwrap_or(&-1), &0);
  ///
  /// // Since `late_value` has already been initialized by the above set_value method call,
  /// // try to initialize the same holder again will cause error.
  /// assert!(late_value.set_value(1).is_err());
  /// ```
  pub fn set_value(&mut self, value: T) -> Result<(), AlreadyInitError> {
    if self.0.is_none() {
      self.0 = Some(value);
      Ok(())
    } else {
      Err(AlreadyInitError)
    }
  }
}
