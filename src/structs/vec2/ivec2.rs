use crate::{DVec2, FVec2, UVec2};

use std::{
  fmt::{self, Display, Formatter},
  ops::{
    Add, AddAssign, BitAnd, BitAndAssign, Div, DivAssign, Mul, MulAssign, Shl, ShlAssign, Shr,
    ShrAssign, Sub, SubAssign,
  },
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl Mul for IVec2 {
  type Output = Self;

  fn mul(self, other: Self) -> Self::Output {
    Self::new((self.x * other.x, self.y * other.y))
  }
}

impl MulAssign for IVec2 {
  fn mul_assign(&mut self, other: Self) {
    self.x *= other.x;
    self.y *= other.y;
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

impl Div for IVec2 {
  type Output = Self;

  fn div(self, other: Self) -> Self::Output {
    debug_assert_ne!(
      other.x, 0,
      "other.x must not be equal to 0 to avoid causing division by zero error!"
    );
    debug_assert_ne!(
      other.y, 0,
      "other.y must not be equal to 0 to avoid causing division by zero error!"
    );
    Self::new((self.x / other.x, self.y / other.y))
  }
}

impl DivAssign for IVec2 {
  fn div_assign(&mut self, other: Self) {
    debug_assert_ne!(
      other.x, 0,
      "other.x must not be equal to 0 to avoid causing division by zero error!"
    );
    debug_assert_ne!(
      other.y, 0,
      "other.y must not be equal to 0 to avoid causing division by zero error!"
    );
    self.x /= other.x;
    self.y /= other.y;
  }
}

impl BitAnd<i32> for IVec2 {
  type Output = IVec2;

  fn bitand(self, value: i32) -> Self::Output {
    Self::new((self.x & value, self.y & value))
  }
}

impl BitAndAssign<i32> for IVec2 {
  fn bitand_assign(&mut self, value: i32) {
    self.x &= value;
    self.y &= value;
  }
}

impl BitAnd for IVec2 {
  type Output = IVec2;

  fn bitand(self, other: Self) -> Self::Output {
    Self::new((self.x & other.x, self.y & other.y))
  }
}

impl BitAndAssign for IVec2 {
  fn bitand_assign(&mut self, other: Self) {
    self.x &= other.x;
    self.y &= other.y;
  }
}

impl Shl<i32> for IVec2 {
  type Output = IVec2;

  fn shl(self, value: i32) -> Self::Output {
    Self::new((self.x << value, self.y << value))
  }
}

impl ShlAssign<i32> for IVec2 {
  fn shl_assign(&mut self, value: i32) {
    self.x <<= value;
    self.y <<= value;
  }
}

impl Shl for IVec2 {
  type Output = IVec2;

  fn shl(self, other: Self) -> Self::Output {
    Self::new((self.x << other.x, self.y << other.y))
  }
}

impl ShlAssign for IVec2 {
  fn shl_assign(&mut self, other: Self) {
    self.x <<= other.x;
    self.y <<= other.y;
  }
}

impl Shr<i32> for IVec2 {
  type Output = IVec2;

  fn shr(self, value: i32) -> Self::Output {
    Self::new((self.x >> value, self.y >> value))
  }
}

impl ShrAssign<i32> for IVec2 {
  fn shr_assign(&mut self, value: i32) {
    self.x >>= value;
    self.y >>= value;
  }
}

impl Shr for IVec2 {
  type Output = IVec2;

  fn shr(self, other: Self) -> Self::Output {
    Self::new((self.x >> other.x, self.y >> other.y))
  }
}

impl ShrAssign for IVec2 {
  fn shr_assign(&mut self, other: Self) {
    self.x >>= other.x;
    self.y >>= other.y;
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
