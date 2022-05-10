use crate::NanoSec;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sec(pub f64);

impl From<NanoSec> for Sec {
  fn from(value: NanoSec) -> Self {
    Self(value.0 * 1e-9)
  }
}
