use crate::Readonly;

#[derive(Copy, Clone, Debug)]
pub struct Arg<T, F: FnMut() -> T> {
  interval: f64,
  next_value: F,
}

impl<T, F: FnMut() -> T> From<F> for Arg<T, F> {
  fn from(next_value: F) -> Self {
    Self {
      interval: 1.0,
      next_value,
    }
  }
}

impl<T, F: FnMut() -> T> From<(F, f64)> for Arg<T, F> {
  fn from((next_value, interval): (F, f64)) -> Self {
    Self {
      interval,
      next_value,
    }
  }
}

/// It is a value holder that keeps changing its value by the `interval` given. Users of this class must keep calling
///
/// `step(dt)` method to simulate frequently changing value.
#[derive(Copy, Clone, Debug)]
pub struct Dynamic<T, F: FnMut() -> T> {
  t: f64,
  interval: Readonly<f64>,
  value: T,
  next_value: F,
}

impl<T, F: FnMut() -> T> Dynamic<T, F> {
  pub fn new(arg: impl Into<Arg<T, F>>) -> Self {
    let Arg {
      interval,
      mut next_value,
    } = arg.into();
    Self {
      t: 0.0,
      interval: interval.into(),
      value: next_value(),
      next_value,
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
