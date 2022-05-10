use crate::math::ApproxEq;

use std::{
  fmt::{self, Display, Formatter},
  ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Copy, Clone, Debug)]
pub struct Arg {
  x: f64,
  y: f64,
  z: f64,
  w: f64,
}

impl From<()> for Arg {
  fn from(_: ()) -> Self {
    Self {
      x: 0.0,
      y: 0.0,
      z: 0.0,
      w: 0.0,
    }
  }
}

impl From<f64> for Arg {
  fn from(value: f64) -> Self {
    Self {
      x: value,
      y: value,
      z: value,
      w: value,
    }
  }
}

impl From<(f64, f64, f64, f64)> for Arg {
  fn from((x, y, z, w): (f64, f64, f64, f64)) -> Self {
    Self { x, y, z, w }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec4 {
  x: f64,
  y: f64,
  z: f64,
  w: f64,
  len: f64,
  is_len_valid: bool,
}

impl Vec4 {
  pub fn new(arg: impl Into<Arg>) -> Self {
    let Arg { x, y, z, w } = arg.into();
    Self {
      x,
      y,
      z,
      w,
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
    Self::new((value, self.y, self.z, self.w))
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
    Self::new((self.x, value, self.z, self.w))
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
    Self::new((self.x, self.y, value, self.w))
  }

  pub const fn get_w(&self) -> f64 {
    self.w
  }

  pub fn set_w(&mut self, value: f64) {
    if self.w == value {
      return;
    }
    self.w = value;
    self.is_len_valid = false;
  }

  pub fn with_w(&self, value: f64) -> Self {
    Self::new((self.x, self.y, self.z, value))
  }

  pub fn set(&mut self, other: Self) {
    if *self == other {
      return;
    }
    self.x = other.x;
    self.y = other.y;
    self.z = other.z;
    self.w = other.w;
    self.is_len_valid = false
  }

  pub fn get_squared_len(&self) -> f64 {
    f64::mul_add(
      self.x,
      self.x,
      f64::mul_add(
        self.y,
        self.y,
        f64::mul_add(self.z, self.z, self.w * self.w),
      ),
    )
  }

  pub fn get_len(&mut self) -> f64 {
    if self.is_len_valid {
      return self.len;
    }
    let result = self.x.hypot(self.y).hypot(self.z).hypot(self.w);
    self.len = result;
    self.is_len_valid = true;
    result
  }

  pub fn get_normalized(&mut self) -> Self {
    *self / self.get_len()
  }

  pub fn get_x_fliped(&self) -> Self {
    Self::new((-self.x, self.y, self.z, self.w))
  }

  pub fn get_y_fliped(&self) -> Self {
    Self::new((self.x, -self.y, self.z, self.w))
  }

  pub fn get_z_fliped(&self) -> Self {
    Self::new((self.x, self.y, -self.z, self.w))
  }

  pub fn get_w_fliped(&self) -> Self {
    Self::new((self.x, self.y, self.z, -self.w))
  }

  pub fn get_fliped(&self) -> Self {
    Self::new((-self.x, -self.y, -self.z, -self.w))
  }
}

impl Add for Vec4 {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self::new((
      self.x + other.x,
      self.y + other.y,
      self.z + other.z,
      self.w + other.w,
    ))
  }
}

impl AddAssign for Vec4 {
  fn add_assign(&mut self, other: Self) {
    if other == Self::new(()) {
      return;
    }
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
    self.w += other.w;
    self.is_len_valid = false;
  }
}

impl Sub for Vec4 {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self::new((
      self.x - other.x,
      self.y - other.y,
      self.z - other.z,
      self.w - other.w,
    ))
  }
}

impl SubAssign for Vec4 {
  fn sub_assign(&mut self, other: Self) {
    if other == Self::new(()) {
      return;
    }
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
    self.w -= other.w;
    self.is_len_valid = false;
  }
}

impl Mul<f64> for Vec4 {
  type Output = Self;

  fn mul(self, value: f64) -> Self::Output {
    Self::new((
      self.x * value,
      self.y * value,
      self.z * value,
      self.w * value,
    ))
  }
}

impl MulAssign<f64> for Vec4 {
  fn mul_assign(&mut self, value: f64) {
    if value == 1.0 {
      return;
    }
    self.x *= value;
    self.y *= value;
    self.z *= value;
    self.w *= value;
    self.is_len_valid = false;
  }
}

impl Div<f64> for Vec4 {
  type Output = Self;

  fn div(self, value: f64) -> Self::Output {
    Self::new((
      self.x / value,
      self.y / value,
      self.z / value,
      self.w / value,
    ))
  }
}

impl DivAssign<f64> for Vec4 {
  fn div_assign(&mut self, value: f64) {
    if value == 1.0 {
      return;
    }
    self.x /= value;
    self.y /= value;
    self.z /= value;
    self.w /= value;
    self.is_len_valid = false;
  }
}

impl Display for Vec4 {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    write!(formatter, "{},{},{},{}", self.x, self.y, self.z, self.w)
  }
}

impl ApproxEq for Vec4 {
  type Rhs = Self;

  fn approx_eq(&self, other: Self::Rhs) -> bool {
    self.x.approx_eq(other.x)
      && self.y.approx_eq(other.y)
      && self.z.approx_eq(other.z)
      && self.w.approx_eq(other.w)
  }
}
