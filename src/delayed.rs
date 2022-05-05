use crate::Readonly;

#[derive(Copy, Clone, Debug)]
pub struct Arg<T: Copy> {
  timeout: f64,
  value: T,
}

impl<T: Copy> From<T> for Arg<T> {
  /// `value`: The initial value held by this holder.
  fn from(value: T) -> Self {
    Self {
      timeout: 1.0,
      value,
    }
  }
}

impl<T: Copy> From<(T, f64)> for Arg<T> {
  /// `value`: The initial value held by this holder.
  ///
  /// `timeout`: The time to be elasped before the new value is being assigned to this holder.
  fn from((value, timeout): (T, f64)) -> Self {
    Self { timeout, value }
  }
}

/// It is a value holder that allows new value to be assigned in the future by the `timeout` given. Users of this class
/// must keep calling `step(dt)` method to simulate delayed assignment of new value.
#[derive(Copy, Clone, Debug)]
pub struct Delayed<T: Copy> {
  t: f64,
  timeout: Readonly<f64>,
  pending_value: T,
  value: T,
}

impl<T: Copy> Delayed<T> {
  pub fn new(arg: impl Into<Arg<T>>) -> Self {
    let Arg { timeout, value } = arg.into();
    Self {
      t: 0.0,
      timeout: timeout.into(),
      pending_value: value,
      value,
    }
  }

  pub fn get_value(&self) -> T {
    self.value
  }

  pub fn set_value(&mut self, value: T) {
    self.pending_value = value;
  }

  pub fn set_now(&mut self, value: T) {
    self.pending_value = value;
    self.value = value;
  }

  /// It advances the time being tracked by the given `dt` for simulating delayed assignment of new value.
  ///
  /// `dt`: Some small amount of time to advance.
  pub fn step(&mut self, dt: f64) {
    self.t += dt;
    if self.t < *self.timeout {
      return;
    }
    self.t %= *self.timeout;
    self.value = self.pending_value;
  }
}
