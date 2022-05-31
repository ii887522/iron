//! A value holder that keeps changing its value by the `interval` given.
//!
//! Users of this module must keep calling [Dynamic::step](./struct.Dynamic.html#method.step) method to simulate
//! frequently changing value.
//!
//! Note that when a [Dynamic](./struct.Dynamic.html) object is created, the `next_value` callback function will be
//! called to get its result to initialize the [Dynamic](./struct.Dynamic.html) object created just now.

/// An argument object to be passed to [Dynamic::new](./struct.Dynamic.html#method.new) to construct a new
/// [Dynamic](./struct.Dynamic.html) object.
///
/// An argument object can be constructed either from [a callback function](./struct.Arg.html#impl-From<F>) or
/// [a callback function and a interval](./struct.Arg.html#impl-From<(F%2C%20f64)>).
#[derive(Copy, Clone, Debug)]
pub struct Arg<T, F: FnMut() -> T> {
  interval: f64,
  next_value: F,
}

impl<T, F: FnMut() -> T> From<F> for Arg<T, F> {
  /// Performs the conversion from the `next_value` callback function that gets called when an `interval` has elasped
  /// through [Dynamic::step](./struct.Dynamic.html#method.step).
  fn from(next_value: F) -> Self {
    Self {
      interval: 1.0,
      next_value,
    }
  }
}

impl<T, F: FnMut() -> T> From<(F, f64)> for Arg<T, F> {
  /// Performs the conversion from the `next_value` callback function that gets called when an `interval` has elasped
  /// through [Dynamic::step](./struct.Dynamic.html#method.step) and a certain amount of time called `interval` to be
  /// elasped for the `next_value` callback function gets called and return a result to be assigned to this object.
  ///
  /// # Panics
  ///
  /// Panics if `interval` is not a number or less than or equal to 0.0 .
  fn from((next_value, interval): (F, f64)) -> Self {
    debug_assert!(
      interval > 0.0,
      "interval must be a number that is greater than 0.0!"
    );
    Self {
      interval,
      next_value,
    }
  }
}

/// A value holder that keeps changing its value by the `interval` given.
#[derive(Copy, Clone, Debug)]
pub struct Dynamic<T, F: FnMut() -> T> {
  t: f64,
  interval: f64,
  value: T,
  next_value: F,
}

impl<T, F: FnMut() -> T> Dynamic<T, F> {
  /// Constructs a new [Dynamic](./struct.Dynamic.html) object from an `arg`. See [Arg](./struct.Arg.html) for more
  /// information on how to create an argument object to be passed into here.
  pub fn new(arg: impl Into<Arg<T, F>>) -> Self {
    let Arg {
      interval,
      mut next_value,
    } = arg.into();
    Self {
      t: 0.0,
      interval,
      value: next_value(),
      next_value,
    }
  }

  /// Retrieves a value held by this object. The value retrieved is the result of calling the `next_value` function.
  ///
  /// # Examples
  ///
  /// ```
  /// use iron_ingot::Dynamic;
  ///
  /// let mut value = 0;
  /// let dynamic_value = Dynamic::new(|| {
  ///   value += 1;
  ///   value
  /// });
  /// assert_eq!(dynamic_value.get_value(), &1);
  /// ```
  pub fn get_value(&self) -> &T {
    &self.value
  }

  /// Advances the time being tracked by a small amount of time called `dt` for simulating frequently changing value.
  ///
  /// # Panics
  ///
  /// Panics if `dt` is not a number or less than 0.0 .
  ///
  /// # Examples
  ///
  /// ```
  /// use iron_ingot::Dynamic;
  ///
  /// let mut value = 0;
  /// let mut dynamic_value = Dynamic::new((|| {
  ///   value += 1;
  ///   value
  /// }, 1.0));
  /// assert_eq!(dynamic_value.get_value(), &1);
  /// dynamic_value.step(1.0);
  /// assert_eq!(dynamic_value.get_value(), &2);
  /// dynamic_value.step(2.0);
  /// assert_eq!(dynamic_value.get_value(), &3);
  /// dynamic_value.step(3.0);
  /// assert_eq!(dynamic_value.get_value(), &4);
  /// ```
  pub fn step(&mut self, dt: f64) {
    debug_assert!(dt >= 0.0, "dt must be a positive number!");
    self.t += dt;
    if self.t < self.interval {
      return;
    }
    self.t %= self.interval;
    self.value = (self.next_value)();
  }
}
