use crate::Bound;

#[derive(Copy, Clone, Debug)]
pub struct Arg(f64, f64);

impl From<()> for Arg {
  fn from(_: ()) -> Self {
    Self(0.0, 0.0)
  }
}

impl From<(f64, f64)> for Arg {
  /// `a`: The first
  ///
  /// `b`: The second value
  fn from((a, b): (f64, f64)) -> Self {
    Self(a, b)
  }
}

/// It defines a sequence between two values.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Seq(pub f64, pub f64);

impl Seq {
  pub fn new(arg: impl Into<Arg>) -> Self {
    let Arg(a, b) = arg.into();
    Self(a, b)
  }

  /// It maps the `value` given in this sequence to a value between 0 and 1.
  ///
  /// `value`: The value to map from.
  ///
  /// It returns a value between 0 and 1.
  pub fn normalize(&self, value: f64) -> f64 {
    (value - self.0) / (self.1 - self.0)
  }

  /// It maps the `value` given between 0 and 1 to a value belongs to this sequence.
  ///
  /// `value`: The value to map from.
  ///
  /// It returns a value belongs to this sequence.
  pub fn unnormalize(&self, value: f64) -> f64 {
    f64::mul_add(value, self.1 - self.0, self.0)
  }
}

impl From<Bound> for Seq {
  fn from(Bound { min, max }: Bound) -> Self {
    Self::new((min, max))
  }
}
