use crate::{math::ApproxEq, I32Vec2};

use std::{
  fmt::{self, Display, Formatter},
  ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Copy, Clone, Debug)]
pub struct Arg {
  x: f64,
  y: f64,
}

impl From<()> for Arg {
  fn from(_: ()) -> Self {
    Self { x: 0.0, y: 0.0 }
  }
}

impl From<f64> for Arg {
  fn from(value: f64) -> Self {
    debug_assert!(!value.is_nan(), "value must be a number!");
    Self { x: value, y: value }
  }
}

impl From<(f64, f64)> for Arg {
  fn from((x, y): (f64, f64)) -> Self {
    debug_assert!(!x.is_nan(), "x must be a number!");
    debug_assert!(!y.is_nan(), "y must be a number!");
    Self { x, y }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct F64Vec2 {
  x: f64,
  y: f64,
  len: f64,
  is_len_valid: bool,
}

impl F64Vec2 {
  pub fn new(arg: impl Into<Arg>) -> Self {
    let Arg { x, y } = arg.into();
    Self {
      x,
      y,
      len: 0.0,
      is_len_valid: false,
    }
  }

  pub const fn get_x(&self) -> f64 {
    self.x
  }

  pub fn set_x(&mut self, value: f64) {
    debug_assert!(!value.is_nan(), "value must be a number!");
    if self.x == value {
      return;
    }
    self.x = value;
    self.is_len_valid = false;
  }

  pub fn with_x(&self, value: f64) -> Self {
    debug_assert!(!value.is_nan(), "value must be a number!");
    Self::new((value, self.y))
  }

  pub const fn get_y(&self) -> f64 {
    self.y
  }

  pub fn set_y(&mut self, value: f64) {
    debug_assert!(!value.is_nan(), "value must be a number!");
    if self.y == value {
      return;
    }
    self.y = value;
    self.is_len_valid = false;
  }

  pub fn with_y(&self, value: f64) -> Self {
    debug_assert!(!value.is_nan(), "value must be a number!");
    Self::new((self.x, value))
  }

  pub fn set(&mut self, other: Self) {
    if *self == other {
      return;
    }
    self.x = other.x;
    self.y = other.y;
    self.is_len_valid = false
  }

  pub fn get_squared_len(&self) -> f64 {
    f64::mul_add(self.x, self.x, self.y * self.y)
  }

  pub fn get_len(&mut self) -> f64 {
    if self.is_len_valid {
      return self.len;
    }
    let result = self.x.hypot(self.y);
    self.len = result;
    self.is_len_valid = true;
    result
  }

  pub fn get_normalized(&mut self) -> Self {
    *self / self.get_len()
  }

  pub fn get_x_fliped(&self) -> Self {
    Self::new((-self.x, self.y))
  }

  pub fn get_y_fliped(&self) -> Self {
    Self::new((self.x, -self.y))
  }

  pub fn get_fliped(&self) -> Self {
    Self::new((-self.x, -self.y))
  }
}

impl Add for F64Vec2 {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self::new((self.x + other.x, self.y + other.y))
  }
}

impl AddAssign for F64Vec2 {
  fn add_assign(&mut self, other: Self) {
    if other == Self::new(()) {
      return;
    }
    self.x += other.x;
    self.y += other.y;
    self.is_len_valid = false;
  }
}

impl Sub for F64Vec2 {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self::new((self.x - other.x, self.y - other.y))
  }
}

impl SubAssign for F64Vec2 {
  fn sub_assign(&mut self, other: Self) {
    if other == Self::new(()) {
      return;
    }
    self.x -= other.x;
    self.y -= other.y;
    self.is_len_valid = false;
  }
}

impl Mul<f64> for F64Vec2 {
  type Output = Self;

  fn mul(self, value: f64) -> Self::Output {
    debug_assert!(!value.is_nan(), "value must be a number!");
    Self::new((self.x * value, self.y * value))
  }
}

impl MulAssign<f64> for F64Vec2 {
  fn mul_assign(&mut self, value: f64) {
    if value == 1.0 {
      return;
    }
    self.x *= value;
    self.y *= value;
    self.is_len_valid = false;
  }
}

impl Div<f64> for F64Vec2 {
  type Output = Self;

  fn div(self, value: f64) -> Self::Output {
    debug_assert!(!value.is_nan(), "value must be a number!");
    debug_assert_ne!(
      value, 0.0,
      "value must not be equal to 0.0 to avoid causing division by zero error!"
    );
    Self::new((self.x / value, self.y / value))
  }
}

impl DivAssign<f64> for F64Vec2 {
  fn div_assign(&mut self, value: f64) {
    debug_assert!(!value.is_nan(), "value must be a number!");
    debug_assert_ne!(
      value, 0.0,
      "value must not be equal to 0.0 to avoid causing division by zero error!"
    );
    if value == 1.0 {
      return;
    }
    self.x /= value;
    self.y /= value;
    self.is_len_valid = false;
  }
}

impl Display for F64Vec2 {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    write!(formatter, "{},{}", self.x, self.y)
  }
}

impl ApproxEq for F64Vec2 {
  type Rhs = Self;

  fn approx_eq(&self, other: Self::Rhs) -> bool {
    self.x.approx_eq(other.x) && self.y.approx_eq(other.y)
  }
}

impl From<I32Vec2> for F64Vec2 {
  fn from(vec: I32Vec2) -> Self {
    Self::new((vec.get_x() as _, vec.get_y() as _))
  }
}
