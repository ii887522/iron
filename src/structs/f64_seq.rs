use crate::{F64Bound, I32Bound, I32Seq};

#[derive(Copy, Clone, Debug)]
pub struct Arg(f64, f64);

impl From<()> for Arg {
  fn from(_: ()) -> Self {
    Self(0.0, 0.0)
  }
}

impl From<(f64, f64)> for Arg {
  fn from((a, b): (f64, f64)) -> Self {
    debug_assert!(!a.is_nan(), "a must be a number!");
    debug_assert!(!b.is_nan(), "b must be a number!");
    debug_assert_ne!(a, b, "a must not be equal to b!");
    Self(a, b)
  }
}

/// It defines a sequence between two values.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct F64Seq(f64, f64);

impl F64Seq {
  pub fn new(arg: impl Into<Arg>) -> Self {
    let Arg(a, b) = arg.into();
    Self(a, b)
  }

  pub const fn get_a(&self) -> f64 {
    self.0
  }

  pub const fn get_b(&self) -> f64 {
    self.1
  }

  /// It maps the `value` given in this sequence to a value between 0 and 1.
  ///
  /// `value`: The value to map from.
  ///
  /// It returns a value between 0 and 1.
  pub fn normalize(&self, value: f64) -> f64 {
    debug_assert!(!value.is_nan(), "value must be a number!");
    (value - self.0) / (self.1 - self.0)
  }

  /// It maps the `value` given between 0 and 1 to a value belongs to this sequence.
  ///
  /// `value`: The value to map from.
  ///
  /// It returns a value belongs to this sequence.
  pub fn unnormalize(&self, value: f64) -> f64 {
    debug_assert!(!value.is_nan(), "value must be a number!");
    f64::mul_add(value, self.1 - self.0, self.0)
  }
}

impl From<F64Bound> for F64Seq {
  fn from(bound: F64Bound) -> Self {
    debug_assert_ne!(
      bound.get_min(),
      bound.get_max(),
      "f64 bound min must not be equal to f64 bound max!"
    );
    Self::new((bound.get_min(), bound.get_max()))
  }
}

impl From<I32Seq> for F64Seq {
  fn from(seq: I32Seq) -> Self {
    Self::new((seq.get_a() as _, seq.get_b() as _))
  }
}

impl From<I32Bound> for F64Seq {
  fn from(bound: I32Bound) -> Self {
    debug_assert_ne!(
      bound.get_min(),
      bound.get_max(),
      "i32 bound min must not be equal to i32 bound max!"
    );
    Self::new((bound.get_min() as _, bound.get_max() as _))
  }
}
