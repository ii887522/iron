use crate::math::PropChecker;

#[derive(Copy, Clone, Debug)]
pub struct Arg(u32);

impl From<()> for Arg {
  fn from(_: ()) -> Self {
    Self(0)
  }
}

impl From<u32> for Arg {
  fn from(value: u32) -> Self {
    Self(value)
  }
}

/// An iterator used to generate a sequence of prime numbers.
#[derive(Copy, Clone, Debug, Default)]
pub struct PrimeIter(u32);

impl PrimeIter {
  pub fn new(value: impl Into<Arg>) -> Self {
    let Arg(value) = value.into();
    Self(value)
  }
}

impl Iterator for PrimeIter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.0 < 2 {
      self.0 = 2;
      return Some(self.0);
    }
    let mut result = (self.0 + if (self.0 as i32).is_odd() { 2 } else { 1 }) as i32;
    while !result.is_prime() {
      result += 2;
    }
    self.0 = result as _;
    Some(self.0)
  }
}
