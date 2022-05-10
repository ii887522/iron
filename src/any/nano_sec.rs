use crate::Sec;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NanoSec(pub f64);

impl From<Sec> for NanoSec {
  fn from(value: Sec) -> Self {
    Self(value.0 * 1e9)
  }
}
