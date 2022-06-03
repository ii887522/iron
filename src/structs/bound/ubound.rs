use crate::{DBound, DSeq, FBound, FSeq, IBound};
use rand::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Arg(u32, u32);

impl From<()> for Arg {
  fn from(_: ()) -> Self {
    Self(0, 0)
  }
}

impl From<(u32, u32)> for Arg {
  fn from((a, b): (u32, u32)) -> Self {
    Self(a, b)
  }
}

/// It defines a boundary between the minimum and maximum value.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct UBound {
  min: u32,
  max: u32,
}

impl UBound {
  pub fn new(arg: impl Into<Arg>) -> Self {
    let Arg(a, b) = arg.into();
    Self {
      min: a.min(b),
      max: a.max(b),
    }
  }

  pub const fn get_min(&self) -> u32 {
    self.min
  }

  pub fn get_middle(&self) -> u32 {
    (self.min + self.max) >> 1
  }

  pub const fn get_max(&self) -> u32 {
    self.max
  }

  /// It checks whether the boundary received overlaps with this boundary.
  ///
  /// `other` The boundary to be checked with.
  ///
  /// It returns true if both boundaries overlaps, false otherwise.
  pub fn is_intersect(&self, other: UBound) -> bool {
    self.min <= other.max && self.max >= other.min
  }

  /// It finds out the intersection between both boundaries received as a boundary.
  ///
  /// `other`: The boundary to intersect with.
  ///
  /// It returns an intersection as a boundary if exist.
  pub fn intersect(&self, other: UBound) -> Option<UBound> {
    if self.is_intersect(other) {
      Some(UBound::new((
        self.min.max(other.min),
        self.max.min(other.max),
      )))
    } else {
      None
    }
  }

  pub fn has(&self, value: u32) -> bool {
    value >= self.min && value <= self.max
  }

  /// It returns a random value in this boundary.
  pub fn rand(&self) -> u32 {
    thread_rng().gen_range(self.min..=self.max)
  }
}

impl From<DBound> for UBound {
  fn from(bound: DBound) -> Self {
    Self::new((bound.get_min() as _, bound.get_max() as _))
  }
}

impl From<FBound> for UBound {
  fn from(bound: FBound) -> Self {
    Self::new((bound.get_min() as _, bound.get_max() as _))
  }
}

impl From<IBound> for UBound {
  fn from(bound: IBound) -> Self {
    Self::new((bound.get_min() as _, bound.get_max() as _))
  }
}

impl From<DSeq> for UBound {
  fn from(seq: DSeq) -> Self {
    Self::new((seq.get_a() as _, seq.get_b() as _))
  }
}

impl From<FSeq> for UBound {
  fn from(seq: FSeq) -> Self {
    Self::new((seq.get_a() as _, seq.get_b() as _))
  }
}
