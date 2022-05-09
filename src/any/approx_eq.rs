pub trait ApproxEq {
  type Rhs;

  fn approx_eq(&self, other: Self::Rhs) -> bool;
}
