use crate::{F64Bound, F64Seq, I32Bound};

#[derive(Copy, Clone, Debug)]
pub struct Arg(i32, i32);

impl From<()> for Arg {
  fn from(_: ()) -> Self {
    Self(0, 0)
  }
}

impl From<(i32, i32)> for Arg {
  fn from((a, b): (i32, i32)) -> Self {
    debug_assert_ne!(a, b, "a must not be equal to b!");
    Self(a, b)
  }
}

/// It defines a sequence between two values.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct I32Seq(i32, i32);

impl I32Seq {
  pub fn new(arg: impl Into<Arg>) -> Self {
    let Arg(a, b) = arg.into();
    Self(a, b)
  }

  pub const fn get_a(&self) -> i32 {
    self.0
  }

  pub const fn get_b(&self) -> i32 {
    self.1
  }
}

impl From<F64Bound> for I32Seq {
  fn from(bound: F64Bound) -> Self {
    debug_assert_ne!(
      bound.get_min(),
      bound.get_max(),
      "f64 bound min must not be equal to f64 bound max!"
    );
    Self::new((bound.get_min() as _, bound.get_max() as _))
  }
}

impl From<I32Bound> for I32Seq {
  fn from(bound: I32Bound) -> Self {
    debug_assert_ne!(
      bound.get_min(),
      bound.get_max(),
      "i32 bound min must not be equal to i32 bound max!"
    );
    Self::new((bound.get_min(), bound.get_max()))
  }
}

impl From<F64Seq> for I32Seq {
  fn from(seq: F64Seq) -> Self {
    Self::new((seq.get_a() as _, seq.get_b() as _))
  }
}
