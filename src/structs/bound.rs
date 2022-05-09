use crate::Seq;
use rand::prelude::*;
use std::cmp::max_by;
use std::cmp::min_by;

#[derive(Copy, Clone, Debug)]
pub struct Arg(f64, f64);

impl From<()> for Arg {
  fn from(_: ()) -> Self {
    Self(0.0, 0.0)
  }
}

impl From<(f64, f64)> for Arg {
  /// `a`: The first value
  ///
  /// `b`: The second value
  fn from((a, b): (f64, f64)) -> Self {
    Self(a, b)
  }
}

/// It defines a boundary between the minimum and maximum value.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bound {
  pub min: f64,
  pub max: f64,
}

impl Bound {
  pub fn new(arg: impl Into<Arg>) -> Self {
    let Arg(a, b) = arg.into();
    Self {
      min: min_by(a, b, |&a, b| a.partial_cmp(b).unwrap()),
      max: max_by(a, b, |&a, b| a.partial_cmp(b).unwrap()),
    }
  }

  pub fn get_middle(&self) -> f64 {
    (self.min + self.max) * 0.5
  }

  /// It checks whether the boundary received overlaps with this boundary.
  ///
  /// `other` The boundary to be checked with.
  ///
  /// It returns true if both boundaries overlaps, false otherwise.
  pub fn is_intersect(&self, other: Bound) -> bool {
    self.min <= other.max && self.max >= other.min
  }

  /// It finds out the intersection between both boundaries received as a boundary.
  ///
  /// `other`: The boundary to intersect with.
  ///
  /// It returns an intersection as a boundary if exist.
  pub fn intersect(&self, other: Bound) -> Option<Bound> {
    if self.is_intersect(other) {
      Some(Bound::new((
        max_by(self.min, other.min, |&a, b| a.partial_cmp(b).unwrap()),
        min_by(self.max, other.max, |&a, b| a.partial_cmp(b).unwrap()),
      )))
    } else {
      None
    }
  }

  /// It constrains the `value` given in this boundary and returns the result.
  ///
  /// `value`: The value to be constrained.
  ///
  /// It returns the constrained value.
  pub fn clamp(&self, value: f64) -> f64 {
    if value < self.min {
      self.min
    } else if value > self.max {
      self.max
    } else {
      value
    }
  }

  /// It returns a random value in this boundary.
  pub fn rand(&self) -> f64 {
    thread_rng().gen_range(self.min..=self.max)
  }
}

impl From<Seq> for Bound {
  fn from(Seq(a, b): Seq) -> Self {
    Self::new((a, b))
  }
}
