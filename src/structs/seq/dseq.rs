use crate::{DBound, FBound, FSeq, IBound, UBound};

/// It defines a sequence between two values.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DSeq(f64, f64);

impl DSeq {
  pub fn new(a: f64, b: f64) -> Self {
    debug_assert!(!a.is_nan(), "a must be a number!");
    debug_assert!(!b.is_nan(), "b must be a number!");
    debug_assert_ne!(a, b, "a must not be equal to b!");
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

impl From<FSeq> for DSeq {
  fn from(seq: FSeq) -> Self {
    Self::new(seq.get_a() as _, seq.get_b() as _)
  }
}

impl From<DBound> for DSeq {
  fn from(bound: DBound) -> Self {
    debug_assert_ne!(
      bound.get_min(),
      bound.get_max(),
      "DBound min must not be equal to DBound max!"
    );
    Self::new(bound.get_min(), bound.get_max())
  }
}

impl From<FBound> for DSeq {
  fn from(bound: FBound) -> Self {
    debug_assert_ne!(
      bound.get_min(),
      bound.get_max(),
      "FBound min must not be equal to FBound max!"
    );
    Self::new(bound.get_min() as _, bound.get_max() as _)
  }
}

impl From<IBound> for DSeq {
  fn from(bound: IBound) -> Self {
    debug_assert_ne!(
      bound.get_min(),
      bound.get_max(),
      "IBound min must not be equal to IBound max!"
    );
    Self::new(bound.get_min() as _, bound.get_max() as _)
  }
}

impl From<UBound> for DSeq {
  fn from(bound: UBound) -> Self {
    debug_assert_ne!(
      bound.get_min(),
      bound.get_max(),
      "UBound min must not be equal to UBound max!"
    );
    Self::new(bound.get_min() as _, bound.get_max() as _)
  }
}
