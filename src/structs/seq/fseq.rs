use crate::{DBound, DSeq, FBound, IBound, UBound};

/// It defines a sequence between two values.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct FSeq(f32, f32);

impl FSeq {
  pub fn new(a: f32, b: f32) -> Self {
    debug_assert!(!a.is_nan(), "a must be a number!");
    debug_assert!(!b.is_nan(), "b must be a number!");
    debug_assert_ne!(a, b, "a must not be equal to b!");
    Self(a, b)
  }

  pub const fn get_a(&self) -> f32 {
    self.0
  }

  pub const fn get_b(&self) -> f32 {
    self.1
  }

  /// It maps the `value` given in this sequence to a value between 0 and 1.
  ///
  /// `value`: The value to map from.
  ///
  /// It returns a value between 0 and 1.
  pub fn normalize(&self, value: f32) -> f32 {
    debug_assert!(!value.is_nan(), "value must be a number!");
    (value - self.0) / (self.1 - self.0)
  }

  /// It maps the `value` given between 0 and 1 to a value belongs to this sequence.
  ///
  /// `value`: The value to map from.
  ///
  /// It returns a value belongs to this sequence.
  pub fn unnormalize(&self, value: f32) -> f32 {
    debug_assert!(!value.is_nan(), "value must be a number!");
    f32::mul_add(value, self.1 - self.0, self.0)
  }
}

impl From<DSeq> for FSeq {
  fn from(seq: DSeq) -> Self {
    Self::new(seq.get_a() as _, seq.get_b() as _)
  }
}

impl From<DBound> for FSeq {
  fn from(bound: DBound) -> Self {
    debug_assert_ne!(
      bound.get_min(),
      bound.get_max(),
      "DBound min must not be equal to DBound max!"
    );
    Self::new(bound.get_min() as _, bound.get_max() as _)
  }
}

impl From<FBound> for FSeq {
  fn from(bound: FBound) -> Self {
    debug_assert_ne!(
      bound.get_min(),
      bound.get_max(),
      "FBound min must not be equal to FBound max!"
    );
    Self::new(bound.get_min() as _, bound.get_max() as _)
  }
}

impl From<IBound> for FSeq {
  fn from(bound: IBound) -> Self {
    debug_assert_ne!(
      bound.get_min(),
      bound.get_max(),
      "IBound min must not be equal to IBound max!"
    );
    Self::new(bound.get_min() as _, bound.get_max() as _)
  }
}

impl From<UBound> for FSeq {
  fn from(bound: UBound) -> Self {
    debug_assert_ne!(
      bound.get_min(),
      bound.get_max(),
      "UBound min must not be equal to UBound max!"
    );
    Self::new(bound.get_min() as _, bound.get_max() as _)
  }
}
