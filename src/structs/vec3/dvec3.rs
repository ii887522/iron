use crate::{math::ApproxEq, FVec3, IVec3, UVec3};

use std::{
  fmt::{self, Display, Formatter},
  ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Copy, Clone, Debug)]
pub struct Arg {
  x: f64,
  y: f64,
  z: f64,
}

impl From<()> for Arg {
  fn from(_: ()) -> Self {
    Self {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  }
}

impl From<f64> for Arg {
  fn from(value: f64) -> Self {
    debug_assert!(!value.is_nan(), "value must be a number!");
    Self {
      x: value,
      y: value,
      z: value,
    }
  }
}

impl From<(f64, f64, f64)> for Arg {
  fn from((x, y, z): (f64, f64, f64)) -> Self {
    debug_assert!(!x.is_nan(), "x must be a number!");
    debug_assert!(!y.is_nan(), "y must be a number!");
    debug_assert!(!z.is_nan(), "z must be a number!");
    Self { x, y, z }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct DVec3 {
  x: f64,
  y: f64,
  z: f64,
  len: f64,
  is_len_valid: bool,
}

impl DVec3 {
  pub fn new(arg: impl Into<Arg>) -> Self {
    let Arg { x, y, z } = arg.into();
    Self {
      x,
      y,
      z,
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
    Self::new((value, self.y, self.z))
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
    Self::new((self.x, value, self.z))
  }

  pub const fn get_z(&self) -> f64 {
    self.z
  }

  pub fn set_z(&mut self, value: f64) {
    debug_assert!(!value.is_nan(), "value must be a number!");
    if self.z == value {
      return;
    }
    self.z = value;
    self.is_len_valid = false;
  }

  pub fn with_z(&self, value: f64) -> Self {
    debug_assert!(!value.is_nan(), "value must be a number!");
    Self::new((self.x, self.y, value))
  }

  pub fn set(&mut self, other: Self) {
    if *self == other {
      return;
    }
    self.x = other.x;
    self.y = other.y;
    self.z = other.z;
    self.is_len_valid = false
  }

  pub fn get_squared_len(&self) -> f64 {
    f64::mul_add(
      self.x,
      self.x,
      f64::mul_add(self.y, self.y, self.z * self.z),
    )
  }

  pub fn get_len(&mut self) -> f64 {
    if self.is_len_valid {
      return self.len;
    }
    let result = self.x.hypot(self.y).hypot(self.z);
    self.len = result;
    self.is_len_valid = true;
    result
  }

  pub fn get_normalized(&mut self) -> Self {
    *self / self.get_len()
  }

  pub fn get_x_fliped(&self) -> Self {
    Self::new((-self.x, self.y, self.z))
  }

  pub fn get_y_fliped(&self) -> Self {
    Self::new((self.x, -self.y, self.z))
  }

  pub fn get_z_fliped(&self) -> Self {
    Self::new((self.x, self.y, -self.z))
  }

  pub fn get_fliped(&self) -> Self {
    Self::new((-self.x, -self.y, -self.z))
  }
}

impl Add for DVec3 {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    debug_assert!(!other.x.is_nan(), "other.x must be a number!");
    debug_assert!(!other.y.is_nan(), "other.y must be a number!");
    debug_assert!(!other.z.is_nan(), "other.z must be a number!");
    Self::new((self.x + other.x, self.y + other.y, self.z + other.z))
  }
}

impl AddAssign for DVec3 {
  fn add_assign(&mut self, other: Self) {
    debug_assert!(!other.x.is_nan(), "other.x must be a number!");
    debug_assert!(!other.y.is_nan(), "other.y must be a number!");
    debug_assert!(!other.z.is_nan(), "other.z must be a number!");
    if other == Self::new(()) {
      return;
    }
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
    self.is_len_valid = false;
  }
}

impl Sub for DVec3 {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    debug_assert!(!other.x.is_nan(), "other.x must be a number!");
    debug_assert!(!other.y.is_nan(), "other.y must be a number!");
    debug_assert!(!other.z.is_nan(), "other.z must be a number!");
    Self::new((self.x - other.x, self.y - other.y, self.z - other.z))
  }
}

impl SubAssign for DVec3 {
  fn sub_assign(&mut self, other: Self) {
    debug_assert!(!other.x.is_nan(), "other.x must be a number!");
    debug_assert!(!other.y.is_nan(), "other.y must be a number!");
    debug_assert!(!other.z.is_nan(), "other.z must be a number!");
    if other == Self::new(()) {
      return;
    }
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
    self.is_len_valid = false;
  }
}

impl Mul<f64> for DVec3 {
  type Output = Self;

  fn mul(self, value: f64) -> Self::Output {
    debug_assert!(!value.is_nan(), "value must be a number!");
    Self::new((self.x * value, self.y * value, self.z * value))
  }
}

impl MulAssign<f64> for DVec3 {
  fn mul_assign(&mut self, value: f64) {
    debug_assert!(!value.is_nan(), "value must be a number!");
    if value == 1.0 {
      return;
    }
    self.x *= value;
    self.y *= value;
    self.z *= value;
    self.is_len_valid = false;
  }
}

impl Mul for DVec3 {
  type Output = Self;

  fn mul(self, other: Self) -> Self::Output {
    debug_assert!(!other.x.is_nan(), "other.x must be a number!");
    debug_assert!(!other.y.is_nan(), "other.y must be a number!");
    debug_assert!(!other.z.is_nan(), "other.z must be a number!");
    Self::new((self.x * other.x, self.y * other.y, self.z * other.z))
  }
}

impl MulAssign for DVec3 {
  fn mul_assign(&mut self, other: Self) {
    debug_assert!(!other.x.is_nan(), "other.x must be a number!");
    debug_assert!(!other.y.is_nan(), "other.y must be a number!");
    debug_assert!(!other.z.is_nan(), "other.z must be a number!");
    if other == Self::new(1.0) {
      return;
    }
    self.x *= other.x;
    self.y *= other.y;
    self.z *= other.z;
    self.is_len_valid = false;
  }
}

impl Div<f64> for DVec3 {
  type Output = Self;

  fn div(self, value: f64) -> Self::Output {
    debug_assert!(!value.is_nan(), "value must be a number!");
    debug_assert_ne!(
      value, 0.0,
      "value must not be equal to 0.0 to avoid causing division by zero error!"
    );
    Self::new((self.x / value, self.y / value, self.z / value))
  }
}

impl DivAssign<f64> for DVec3 {
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
    self.z /= value;
    self.is_len_valid = false;
  }
}

impl Div for DVec3 {
  type Output = Self;

  fn div(self, other: Self) -> Self::Output {
    debug_assert!(!other.x.is_nan(), "other.x must be a number!");
    debug_assert!(!other.y.is_nan(), "other.y must be a number!");
    debug_assert!(!other.z.is_nan(), "other.z must be a number!");
    debug_assert_ne!(
      other.x, 0.0,
      "other.x must not be equal to 0.0 to avoid causing division by zero error!"
    );
    debug_assert_ne!(
      other.y, 0.0,
      "other.y must not be equal to 0.0 to avoid causing division by zero error!"
    );
    debug_assert_ne!(
      other.z, 0.0,
      "other.z must not be equal to 0.0 to avoid causing division by zero error!"
    );
    Self::new((self.x / other.x, self.y / other.y, self.z / other.z))
  }
}

impl DivAssign for DVec3 {
  fn div_assign(&mut self, other: Self) {
    debug_assert!(!other.x.is_nan(), "other.x must be a number!");
    debug_assert!(!other.y.is_nan(), "other.y must be a number!");
    debug_assert!(!other.z.is_nan(), "other.z must be a number!");
    debug_assert_ne!(
      other.x, 0.0,
      "other.x must not be equal to 0.0 to avoid causing division by zero error!"
    );
    debug_assert_ne!(
      other.y, 0.0,
      "other.y must not be equal to 0.0 to avoid causing division by zero error!"
    );
    debug_assert_ne!(
      other.z, 0.0,
      "other.z must not be equal to 0.0 to avoid causing division by zero error!"
    );
    if other == Self::new(1.0) {
      return;
    }
    self.x /= other.x;
    self.y /= other.y;
    self.z /= other.z;
    self.is_len_valid = false;
  }
}

impl Display for DVec3 {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    write!(formatter, "{},{},{}", self.x, self.y, self.z)
  }
}

impl ApproxEq for DVec3 {
  type Rhs = Self;

  fn approx_eq(&self, other: Self::Rhs) -> bool {
    self.x.approx_eq(other.x) && self.y.approx_eq(other.y) && self.z.approx_eq(other.z)
  }
}

impl From<FVec3> for DVec3 {
  fn from(vec: FVec3) -> Self {
    Self::new((vec.get_x() as _, vec.get_y() as _, vec.get_z() as _))
  }
}

impl From<IVec3> for DVec3 {
  fn from(vec: IVec3) -> Self {
    Self::new((vec.get_x() as _, vec.get_y() as _, vec.get_z() as _))
  }
}

impl From<UVec3> for DVec3 {
  fn from(vec: UVec3) -> Self {
    Self::new((vec.get_x() as _, vec.get_y() as _, vec.get_z() as _))
  }
}
