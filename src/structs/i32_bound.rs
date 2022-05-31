use crate::{F64Bound, F64Seq, I32Seq};
use rand::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Arg(i32, i32);

impl From<()> for Arg {
  fn from(_: ()) -> Self {
    Self(0, 0)
  }
}

impl From<(i32, i32)> for Arg {
  fn from((a, b): (i32, i32)) -> Self {
    Self(a, b)
  }
}

/// It defines a boundary between the minimum and maximum value.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct I32Bound {
  min: i32,
  max: i32,
}

impl I32Bound {
  pub fn new(arg: impl Into<Arg>) -> Self {
    let Arg(a, b) = arg.into();
    Self {
      min: a.min(b),
      max: a.max(b),
    }
  }

  pub const fn get_min(&self) -> i32 {
    self.min
  }

  pub fn get_middle(&self) -> i32 {
    (self.min + self.max) >> 1
  }

  pub const fn get_max(&self) -> i32 {
    self.max
  }

  /// It checks whether the boundary received overlaps with this boundary.
  ///
  /// `other` The boundary to be checked with.
  ///
  /// It returns true if both boundaries overlaps, false otherwise.
  pub fn is_intersect(&self, other: I32Bound) -> bool {
    self.min <= other.max && self.max >= other.min
  }

  /// It finds out the intersection between both boundaries received as a boundary.
  ///
  /// `other`: The boundary to intersect with.
  ///
  /// It returns an intersection as a boundary if exist.
  pub fn intersect(&self, other: I32Bound) -> Option<I32Bound> {
    if self.is_intersect(other) {
      Some(I32Bound::new((
        self.min.max(other.min),
        self.max.min(other.max),
      )))
    } else {
      None
    }
  }

  pub fn has(&self, value: i32) -> bool {
    value >= self.min && value <= self.max
  }

  /// It returns a random value in this boundary.
  pub fn rand(&self) -> i32 {
    thread_rng().gen_range(self.min..=self.max)
  }
}

impl From<I32Seq> for I32Bound {
  fn from(seq: I32Seq) -> Self {
    Self::new((seq.get_a(), seq.get_b()))
  }
}

impl From<F64Seq> for I32Bound {
  fn from(seq: F64Seq) -> Self {
    Self::new((seq.get_a() as _, seq.get_b() as _))
  }
}

impl From<F64Bound> for I32Bound {
  fn from(bound: F64Bound) -> Self {
    Self::new((bound.get_min() as _, bound.get_max() as _))
  }
}
