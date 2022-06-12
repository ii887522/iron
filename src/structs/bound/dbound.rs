use crate::{DSeq, FBound, FSeq, IBound, UBound};
use rand::prelude::*;

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
    Self(a, b)
  }
}

/// It defines a boundary between the minimum and maximum value.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DBound {
  min: f64,
  max: f64,
}

impl DBound {
  pub fn new(arg: impl Into<Arg>) -> Self {
    let Arg(a, b) = arg.into();
    Self {
      min: a.min(b),
      max: a.max(b),
    }
  }

  pub const fn get_min(&self) -> f64 {
    self.min
  }

  pub fn get_middle(&self) -> f64 {
    (self.min + self.max) * 0.5
  }

  pub const fn get_max(&self) -> f64 {
    self.max
  }

  /// It checks whether the boundary received overlaps with this boundary.
  ///
  /// `other` The boundary to be checked with.
  ///
  /// It returns true if both boundaries overlaps, false otherwise.
  pub fn is_intersect(&self, other: DBound) -> bool {
    self.min <= other.max && self.max >= other.min
  }

  /// It finds out the intersection between both boundaries received as a boundary.
  ///
  /// `other`: The boundary to intersect with.
  ///
  /// It returns an intersection as a boundary if exist.
  pub fn intersect(&self, other: DBound) -> Option<DBound> {
    if self.is_intersect(other) {
      Some(DBound::new((
        self.min.max(other.min),
        self.max.min(other.max),
      )))
    } else {
      None
    }
  }

  pub fn has(&self, value: f64) -> bool {
    value >= self.min && value <= self.max
  }

  /// It returns a random value in this boundary.
  pub fn rand(&self) -> f64 {
    thread_rng().gen_range(self.min..=self.max)
  }

  pub fn clamp(&self, value: f64) -> f64 {
    if value < self.min {
      self.min
    } else if value > self.max {
      self.max
    } else {
      value
    }
  }
}

impl From<FBound> for DBound {
  fn from(bound: FBound) -> Self {
    Self::new((bound.get_min() as _, bound.get_max() as _))
  }
}

impl From<IBound> for DBound {
  fn from(bound: IBound) -> Self {
    Self::new((bound.get_min() as _, bound.get_max() as _))
  }
}

impl From<UBound> for DBound {
  fn from(bound: UBound) -> Self {
    Self::new((bound.get_min() as _, bound.get_max() as _))
  }
}

impl From<DSeq> for DBound {
  fn from(seq: DSeq) -> Self {
    Self::new((seq.get_a(), seq.get_b()))
  }
}

impl From<FSeq> for DBound {
  fn from(seq: FSeq) -> Self {
    Self::new((seq.get_a() as _, seq.get_b() as _))
  }
}
