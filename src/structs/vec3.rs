use crate::ApproxEq;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

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
    Self {
      x: value,
      y: value,
      z: value,
    }
  }
}

impl From<(f64, f64, f64)> for Arg {
  fn from((x, y, z): (f64, f64, f64)) -> Self {
    Self { x, y, z }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
  x: f64,
  y: f64,
  z: f64,
  len: f64,
  is_len_valid: bool,
}

impl Vec3 {
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
    if self.x == value {
      return;
    }
    self.x = value;
    self.is_len_valid = false;
  }

  pub fn with_x(&self, value: f64) -> Self {
    Self::new((value, self.y, self.z))
  }

  pub const fn get_y(&self) -> f64 {
    self.y
  }

  pub fn set_y(&mut self, value: f64) {
    if self.y == value {
      return;
    }
    self.y = value;
    self.is_len_valid = false;
  }

  pub fn with_y(&self, value: f64) -> Self {
    Self::new((self.x, value, self.z))
  }

  pub const fn get_z(&self) -> f64 {
    self.z
  }

  pub fn set_z(&mut self, value: f64) {
    if self.z == value {
      return;
    }
    self.z = value;
    self.is_len_valid = false;
  }

  pub fn with_z(&self, value: f64) -> Self {
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

impl Add for Vec3 {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self::new((self.x + other.x, self.y + other.y, self.z + other.z))
  }
}

impl AddAssign for Vec3 {
  fn add_assign(&mut self, other: Self) {
    if other == Self::new(()) {
      return;
    }
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
    self.is_len_valid = false;
  }
}

impl Sub for Vec3 {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self::new((self.x - other.x, self.y - other.y, self.z - other.z))
  }
}

impl SubAssign for Vec3 {
  fn sub_assign(&mut self, other: Self) {
    if other == Self::new(()) {
      return;
    }
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
    self.is_len_valid = false;
  }
}

impl Mul<f64> for Vec3 {
  type Output = Self;

  fn mul(self, value: f64) -> Self::Output {
    Self::new((self.x * value, self.y * value, self.z * value))
  }
}

impl MulAssign<f64> for Vec3 {
  fn mul_assign(&mut self, value: f64) {
    if value == 1.0 {
      return;
    }
    self.x *= value;
    self.y *= value;
    self.z *= value;
    self.is_len_valid = false;
  }
}

impl Div<f64> for Vec3 {
  type Output = Self;

  fn div(self, value: f64) -> Self::Output {
    Self::new((self.x / value, self.y / value, self.z / value))
  }
}

impl DivAssign<f64> for Vec3 {
  fn div_assign(&mut self, value: f64) {
    if value == 1.0 {
      return;
    }
    self.x /= value;
    self.y /= value;
    self.z /= value;
    self.is_len_valid = false;
  }
}

impl Display for Vec3 {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    write!(formatter, "{},{},{}", self.x, self.y, self.z)
  }
}

impl ApproxEq for Vec3 {
  type Rhs = Self;

  fn approx_eq(&self, other: Self::Rhs) -> bool {
    self.x.approx_eq(other.x) && self.y.approx_eq(other.y) && self.z.approx_eq(other.z)
  }
}
