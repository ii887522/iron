use crate::Immutable;

pub struct Arg<T, F: FnMut() -> T> {
  next_value: F,
  interval: f64,
}

impl<T, F: FnMut() -> T> From<F> for Arg<T, F> {
  /// `get_value`: The function that returns a new temporary value of this holder.
  fn from(next_value: F) -> Self {
    Self {
      next_value,
      interval: 1.0,
    }
  }
}

impl<T, F: FnMut() -> T> From<(F, f64)> for Arg<T, F> {
  /// `get_value`: The function that returns a new temporary value of this holder.
  ///
  /// `dt`: Some small amount of time to advance.
  fn from((next_value, interval): (F, f64)) -> Self {
    Self {
      next_value,
      interval,
    }
  }
}

/// It is a value holder that keeps changing its value by the `interval` given. Users of this class must keep calling
///
/// `step(dt)` method to simulate frequently changing value.
pub struct Dynamic<T, F: FnMut() -> T> {
  t: f64,
  value: T,
  next_value: F,
  interval: Immutable<f64>,
}

impl<T, F: FnMut() -> T> Dynamic<T, F> {
  pub fn new<A: Into<Arg<T, F>>>(arg: A) -> Self {
    let Arg {
      mut next_value,
      interval,
    } = arg.into();
    Self {
      t: 0.0,
      value: next_value(),
      next_value,
      interval: interval.into(),
    }
  }

  pub fn get_value(&self) -> &T {
    &self.value
  }

  /// It advances the time being tracked by the given `dt` for simulating frequently changing value.
  ///
  /// `dt`: Some small amount of time to advance.
  pub fn step(&mut self, dt: f64) {
    self.t += dt;
    if self.t < *self.interval {
      return;
    }
    self.t %= *self.interval;
    self.value = (self.next_value)();
  }
}
