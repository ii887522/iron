//! A holder to a value in nanoseconds.

use crate::Sec;

/// A holder to a value in nanoseconds.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NanoSec(pub f64);

impl From<Sec> for NanoSec {
  fn from(value: Sec) -> Self {
    Self(value.0 * 1e9)
  }
}
