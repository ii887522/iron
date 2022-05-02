use crate::Immutable;

pub struct Arg<T: Copy> {
  value: T,
  timeout: f64,
}

impl<T: Copy> From<T> for Arg<T> {
  /// `value`: The initial value held by this holder.
  fn from(value: T) -> Self {
    Self {
      value,
      timeout: 1.0,
    }
  }
}

impl<T: Copy> From<(T, f64)> for Arg<T> {
  /// `value`: The initial value held by this holder.
  ///
  /// `timeout`: The time to be elasped before the new value is being assigned to this holder.
  fn from((value, timeout): (T, f64)) -> Self {
    Self { value, timeout }
  }
}

/// It is a value holder that allows new value to be assigned in the future by the `timeout` given. Users of this class
/// must keep calling `step(dt)` method to simulate delayed assignment of new value.
pub struct Delayed<T: Copy> {
  t: f64,
  pending_value: T,
  value: T,
  timeout: Immutable<f64>,
}

impl<T: Copy> Delayed<T> {
  pub fn new<A: Into<Arg<T>>>(arg: A) -> Self {
    let arg = arg.into();
    Self {
      value: arg.value,
      timeout: arg.timeout.into(),
      t: 0.0,
      pending_value: arg.value,
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
