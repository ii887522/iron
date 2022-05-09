use crate::ApproxEq;

impl ApproxEq for f64 {
  type Rhs = Self;

  fn approx_eq(&self, other: Self::Rhs) -> bool {
    (self - f64::EPSILON * self..=self + f64::EPSILON * self).contains(&other)
  }
}
