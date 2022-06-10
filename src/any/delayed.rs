//! A value holder that allows new value to be assigned in the future by the `timeout` given.
//!
//! Users of this module must keep calling [Delayed::step](./struct.Delayed.html#method.step) method after created an
//! object through [Delayed::new](./struct.Delayed.html#method.new) function and assigned a new value through
//! [Delayed::set_value](./struct.Delayed.html#method.set_value) method to simulate delayed assignment of new value.

/// An argument object to be passed to [Delayed::new](./struct.Delayed.html#method.new) to construct a new
/// [Delayed](./struct.Delayed.html) object.
///
/// An argument object can be constructed either from [a single value](./struct.Arg.html#impl-From<%26%27a%20T>) or
/// [a value and a timeout](./struct.Arg.html#impl-From<(%26%27a%20T%2C%20f32)>).
#[derive(Copy, Clone, Debug)]
pub struct Arg<'a, T: ?Sized> {
  timeout: f32,
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

impl<'a, T: ?Sized> From<(&'a T, f32)> for Arg<'a, T> {
  /// Performs the conversion from a `value` and a certain amount of `timeout` to be elapsed before the new `value` is
  /// being assigned and held.
  ///
  /// # Panics
  ///
  /// Panics if `timeout` is not a number or less than or equal to 0.0 .
  fn from((value, timeout): (&'a T, f32)) -> Self {
    debug_assert!(
      timeout > 0.0,
      "timeout must be a number that is greater than 0.0!"
    );
    Self { timeout, value }
  }
}

/// A value holder that allows new value to be assigned in the future by the `timeout` given.
///
/// A `Delayed` object can be constructed by calling [Delayed::new](./struct.Delayed.html#method.new) function.
#[derive(Copy, Clone, Debug)]
pub struct Delayed<'a, T: ?Sized> {
  t: f32,
  timeout: f32,
  pending_value: &'a T,
  value: &'a T,
}

impl<'a, T: ?Sized> Delayed<'a, T> {
  /// Constructs a new [Delayed](./struct.Delayed.html) object from the given `arg`. See [Arg](./struct.Arg.html) for
  /// more information on how to create an argument object to be passed into here.
  pub fn new(arg: impl Into<Arg<'a, T>>) -> Self {
    let Arg { timeout, value } = arg.into();
    Self {
      t: 0.0,
      timeout,
      pending_value: value,
      value,
    }
  }

  /// Retrieves a value held by this object.
  ///
  /// Note that the value retrieved may or may not be the latest value assigned by
  /// [Delayed::set_value](./struct.Delayed.html#method.set_value) method.
  ///
  /// # Examples
  ///
  /// ```
  /// use iron_ingot::Delayed;
  ///
  /// let mut delayed_value = Delayed::new(&0);
  ///
  /// // The latest value is the initial value when creating the delayed_value object. When there is
  /// // no set_value method call in between, get_value method call should return the latest value
  /// // which is 0.
  /// assert_eq!(delayed_value.get_value(), &0);
  ///
  /// // Change the latest value to become 1.
  /// delayed_value.set_value(&1);
  ///
  /// // Since the user has not called step method to simulate delayed assignment of latest value.
  /// // get_value method call below still return the old value which is 0.
  /// assert_eq!(delayed_value.get_value(), &0);
  /// ```
  pub fn get_value(&self) -> &T {
    self.value
  }

  /// Assigns a new `value` to be held by this object in the future.
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

  /// Assigns a new `value` to be held by this object immediately without needing to wait and
  /// [step](./struct.Delayed.html#method.step).
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

  /// Advances the time being tracked by a small amount of time called `dt` for simulating delayed assignment of new
  /// value.
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
  pub fn step(&mut self, dt: f32) {
    debug_assert!(dt >= 0.0, "dt must be a positive number!");
    self.t += dt;
    if self.t < self.timeout {
      return;
    }
    self.t %= self.timeout;
    self.value = self.pending_value;
  }
}
