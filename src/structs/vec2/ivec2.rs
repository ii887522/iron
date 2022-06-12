use crate::{DVec2, FVec2, UVec2};

use std::{
  fmt::{self, Display, Formatter},
  ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Copy, Clone, Debug)]
pub struct Arg {
  x: i32,
  y: i32,
}

impl From<()> for Arg {
  fn from(_: ()) -> Self {
    Self { x: 0, y: 0 }
  }
}

impl From<i32> for Arg {
  fn from(value: i32) -> Self {
    Self { x: value, y: value }
  }
}

impl From<(i32, i32)> for Arg {
  fn from((x, y): (i32, i32)) -> Self {
    Self { x, y }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IVec2 {
  x: i32,
  y: i32,
}

impl IVec2 {
  pub fn new(arg: impl Into<Arg>) -> Self {
    let Arg { x, y } = arg.into();
    Self { x, y }
  }

  pub const fn get_x(&self) -> i32 {
    self.x
  }

  pub fn set_x(&mut self, value: i32) {
    self.x = value;
  }

  pub fn with_x(&self, value: i32) -> Self {
    Self::new((value, self.y))
  }

  pub const fn get_y(&self) -> i32 {
    self.y
  }

  pub fn set_y(&mut self, value: i32) {
    self.y = value;
  }

  pub fn with_y(&self, value: i32) -> Self {
    Self::new((self.x, value))
  }

  pub fn set(&mut self, other: Self) {
    self.x = other.x;
    self.y = other.y;
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

impl Add for IVec2 {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self::new((self.x + other.x, self.y + other.y))
  }
}

impl AddAssign for IVec2 {
  fn add_assign(&mut self, other: Self) {
    self.x += other.x;
    self.y += other.y;
  }
}

impl Sub for IVec2 {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self::new((self.x - other.x, self.y - other.y))
  }
}

impl SubAssign for IVec2 {
  fn sub_assign(&mut self, other: Self) {
    self.x -= other.x;
    self.y -= other.y;
  }
}

impl Mul<i32> for IVec2 {
  type Output = Self;

  fn mul(self, value: i32) -> Self::Output {
    Self::new((self.x * value, self.y * value))
  }
}

impl MulAssign<i32> for IVec2 {
  fn mul_assign(&mut self, value: i32) {
    self.x *= value;
    self.y *= value;
  }
}

impl Div<i32> for IVec2 {
  type Output = Self;

  fn div(self, value: i32) -> Self::Output {
    debug_assert_ne!(
      value, 0,
      "value must not be equal to 0 to avoid causing division by zero error!"
    );
    Self::new((self.x / value, self.y / value))
  }
}

impl DivAssign<i32> for IVec2 {
  fn div_assign(&mut self, value: i32) {
    debug_assert_ne!(
      value, 0,
      "value must not be equal to 0 to avoid causing division by zero error!"
    );
    self.x /= value;
    self.y /= value;
  }
}

impl Display for IVec2 {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    write!(formatter, "{},{}", self.x, self.y)
  }
}

impl From<DVec2> for IVec2 {
  fn from(vec: DVec2) -> Self {
    Self::new((vec.get_x() as _, vec.get_y() as _))
  }
}

impl From<FVec2> for IVec2 {
  fn from(vec: FVec2) -> Self {
    Self::new((vec.get_x() as _, vec.get_y() as _))
  }
}

impl From<UVec2> for IVec2 {
  fn from(vec: UVec2) -> Self {
    Self::new((vec.get_x() as _, vec.get_y() as _))
  }
}
