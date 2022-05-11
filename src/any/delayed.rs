//! It is a value holder that allows new value to be assigned in the future by the `timeout` given.
//!
//! Users of this module must keep calling [Delayed::step(dt)](./struct.Delayed.html#method.step) method after created
//! an object through [Delayed::new(arg)](./struct.Delayed.html#method.new) function and assigned a new value through
//! [Delayed::set_value(value)](./struct.Delayed.html#method.set_value) method to simulate delayed assignment of new
//! value.

use crate::Readonly;

/// It is an argument object to be passed to [Delayed::new(arg)](./struct.Delayed.html#method.new) to construct a new
/// [Delayed](./struct.Delayed.html) object.
///
/// An argument object can be constructed either from [a single value](./struct.Arg.html#impl-From<%26%27a%20T>) or
/// [a value and a timeout](./struct.Arg.html#impl-From<(%26%27a%20T%2C%20f64)>).
#[derive(Copy, Clone, Debug)]
pub struct Arg<'a, T: ?Sized> {
  timeout: f64,
  value: &'a T,
}

impl<'a, T: ?Sized> From<&'a T> for Arg<'a, T> {
  fn from(value: &'a T) -> Self {
    Self {
      timeout: 1.0,
      value,
    }
  }
}

impl<'a, T: ?Sized> From<(&'a T, f64)> for Arg<'a, T> {
  /// Performs the conversion.
  ///
  /// `timeout`: A certain amount of time to be elasped before the new `value` is being assigned and held. It must be a
  /// number that is greater than 0.0 .
  ///
  /// # Panics
  ///
  /// Panics if `timeout` is not a number or less than or equal to 0.0 .
  fn from((value, timeout): (&'a T, f64)) -> Self {
    debug_assert!(
      timeout > 0.0,
      "timeout must be a number that is greater than 0.0!"
    );
    Self { timeout, value }
  }
}

/// It is a value holder that allows new value to be assigned in the future by the `timeout` given.
///
/// A `Delayed` object can be constructed by calling [Delayed::new(arg)](./struct.Delayed.html#method.new) function.
#[derive(Copy, Clone, Debug)]
pub struct Delayed<'a, T: ?Sized> {
  t: f64,
  timeout: Readonly<f64>,
  pending_value: &'a T,
  value: &'a T,
}

impl<'a, T: ?Sized> Delayed<'a, T> {
  /// It constructs a new [Delayed](./struct.Delayed.html) object.
  ///
  /// `arg`: An argument object. See [Arg](./struct.Arg.html) for more information on how to create an argument object
  /// to be passed into here.
  pub fn new(arg: impl Into<Arg<'a, T>>) -> Self {
    let Arg { timeout, value } = arg.into();
    Self {
      t: 0.0,
      timeout: timeout.into(),
      pending_value: value,
      value,
    }
  }

  /// It retrieves a value held by this object. The value retrieved may or may not be the latest value assigned by
  /// [Delayed::set_value(value)](./struct.Delayed.html#method.set_value) method.
  ///
  /// # Examples
  ///
  /// ```
  /// use iron_ingot::Delayed;
  ///
  /// assert_eq!(Delayed::new(&0).get_value(), &0);
  /// ```
  pub fn get_value(&self) -> &T {
    self.value
  }

  /// It assigns a new `value` to be held by this object in the future.
  ///
  /// `value`: The value that can be retrieved in the future by calling
  /// [Delayed::get_value()](./struct.Delayed.html#method.get_value).
  ///
  /// # Examples
  ///
  /// ```
  /// use iron_ingot::Delayed;
  ///
  /// let mut delayed_value = Delayed::new((&0, 1.0));
  /// delayed_value.set_value(&1);
  ///
  /// // The latest value has not been assigned since the `delayed_value` hasn't timeout yet.
  /// assert_eq!(delayed_value.get_value(), &0);
  ///
  /// // Make `delayed_value` timeouts.
  /// delayed_value.step(1.0);
  ///
  /// // The latest value has been assigned since the `delayed_value` has timeout.
  /// assert_eq!(delayed_value.get_value(), &1);
  /// ```
  pub fn set_value(&mut self, value: &'a T) {
    self.pending_value = value;
  }

  /// It assigns a new `value` to be held by this object immediately without needing to wait and
  /// [step](./struct.Delayed.html#method.step).
  ///
  /// `value`: The value that can be retrieved immediately by calling
  /// [Delayed::get_value()](./struct.Delayed.html#method.get_value).
  ///
  /// # Examples
  ///
  /// ```
  /// use iron_ingot::Delayed;
  ///
  /// let mut delayed_value = Delayed::new(&0);
  /// delayed_value.set_now(&3);
  /// assert_eq!(delayed_value.get_value(), &3);
  /// ```
  pub fn set_now(&mut self, value: &'a T) {
    self.pending_value = value;
    self.value = value;
  }

  /// It advances the time being tracked by the given `dt` for simulating delayed assignment of new value.
  ///
  /// `dt`: Some small amount of time to advance. It must be a positive number.
  ///
  /// # Panics
  ///
  /// Panics if the `dt` is not a number or less than 0.0 .
  ///
  /// # Examples
  ///
  /// ```
  /// use iron_ingot::Delayed;
  ///
  /// let mut delayed_value = Delayed::new((&0, 1.0));
  /// delayed_value.set_value(&1);
  ///
  /// // The latest value has not been assigned since the `delayed_value` hasn't timeout yet.
  /// assert_eq!(delayed_value.get_value(), &0);
  ///
  /// // Make `delayed_value` timeouts.
  /// delayed_value.step(1.0);
  ///
  /// // The latest value has been assigned since the `delayed_value` has timeout.
  /// assert_eq!(delayed_value.get_value(), &1);
  /// ```
  pub fn step(&mut self, dt: f64) {
    debug_assert!(dt >= 0.0, "dt must be a positive number!");
    self.t += dt;
    if self.t < *self.timeout {
      return;
    }
    self.t %= *self.timeout;
    self.value = self.pending_value;
  }
}
