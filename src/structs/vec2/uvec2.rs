use crate::{DVec2, FVec2, IVec2};

use std::{
  fmt::{self, Display, Formatter},
  ops::{
    Add, AddAssign, BitAnd, BitAndAssign, Div, DivAssign, Mul, MulAssign, Shl, ShlAssign, Shr,
    ShrAssign, Sub, SubAssign,
  },
};

#[derive(Copy, Clone, Debug)]
pub struct Arg {
  x: u32,
  y: u32,
}

impl From<()> for Arg {
  fn from(_: ()) -> Self {
    Self { x: 0, y: 0 }
  }
}

impl From<u32> for Arg {
  fn from(value: u32) -> Self {
    Self { x: value, y: value }
  }
}

impl From<(u32, u32)> for Arg {
  fn from((x, y): (u32, u32)) -> Self {
    Self { x, y }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UVec2 {
  x: u32,
  y: u32,
}

impl UVec2 {
  pub fn new(arg: impl Into<Arg>) -> Self {
    let Arg { x, y } = arg.into();
    Self { x, y }
  }

  pub const fn get_x(&self) -> u32 {
    self.x
  }

  pub fn set_x(&mut self, value: u32) {
    self.x = value;
  }

  pub fn with_x(&self, value: u32) -> Self {
    Self::new((value, self.y))
  }

  pub const fn get_y(&self) -> u32 {
    self.y
  }

  pub fn set_y(&mut self, value: u32) {
    self.y = value;
  }

  pub fn with_y(&self, value: u32) -> Self {
    Self::new((self.x, value))
  }

  pub fn set(&mut self, other: Self) {
    self.x = other.x;
    self.y = other.y;
  }
}

impl Add for UVec2 {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self::new((self.x + other.x, self.y + other.y))
  }
}

impl AddAssign for UVec2 {
  fn add_assign(&mut self, other: Self) {
    self.x += other.x;
    self.y += other.y;
  }
}

impl Sub for UVec2 {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self::new((self.x - other.x, self.y - other.y))
  }
}

impl SubAssign for UVec2 {
  fn sub_assign(&mut self, other: Self) {
    self.x -= other.x;
    self.y -= other.y;
  }
}

impl Mul<u32> for UVec2 {
  type Output = Self;

  fn mul(self, value: u32) -> Self::Output {
    Self::new((self.x * value, self.y * value))
  }
}

impl MulAssign<u32> for UVec2 {
  fn mul_assign(&mut self, value: u32) {
    self.x *= value;
    self.y *= value;
  }
}

impl Div<u32> for UVec2 {
  type Output = Self;

  fn div(self, value: u32) -> Self::Output {
    debug_assert_ne!(
      value, 0,
      "value must not be equal to 0 to avoid causing division by zero error!"
    );
    Self::new((self.x / value, self.y / value))
  }
}

impl DivAssign<u32> for UVec2 {
  fn div_assign(&mut self, value: u32) {
    debug_assert_ne!(
      value, 0,
      "value must not be equal to 0 to avoid causing division by zero error!"
    );
    self.x /= value;
    self.y /= value;
  }
}

impl BitAnd<u32> for UVec2 {
  type Output = UVec2;

  fn bitand(self, value: u32) -> Self::Output {
    Self::new((self.x & value, self.y & value))
  }
}

impl BitAndAssign<u32> for UVec2 {
  fn bitand_assign(&mut self, value: u32) {
    self.x &= value;
    self.y &= value;
  }
}

impl BitAnd for UVec2 {
  type Output = UVec2;

  fn bitand(self, other: Self) -> Self::Output {
    Self::new((self.x & other.x, self.y & other.y))
  }
}

impl BitAndAssign for UVec2 {
  fn bitand_assign(&mut self, other: Self) {
    self.x &= other.x;
    self.y &= other.y;
  }
}

impl Shl<u32> for UVec2 {
  type Output = UVec2;

  fn shl(self, value: u32) -> Self::Output {
    Self::new((self.x << value, self.y << value))
  }
}

impl ShlAssign<u32> for UVec2 {
  fn shl_assign(&mut self, value: u32) {
    self.x <<= value;
    self.y <<= value;
  }
}

impl Shl for UVec2 {
  type Output = UVec2;

  fn shl(self, other: Self) -> Self::Output {
    Self::new((self.x << other.x, self.y << other.y))
  }
}

impl ShlAssign for UVec2 {
  fn shl_assign(&mut self, other: Self) {
    self.x <<= other.x;
    self.y <<= other.y;
  }
}

impl Shr<u32> for UVec2 {
  type Output = UVec2;

  fn shr(self, value: u32) -> Self::Output {
    Self::new((self.x >> value, self.y >> value))
  }
}

impl ShrAssign<u32> for UVec2 {
  fn shr_assign(&mut self, value: u32) {
    self.x >>= value;
    self.y >>= value;
  }
}

impl Shr for UVec2 {
  type Output = UVec2;

  fn shr(self, other: Self) -> Self::Output {
    Self::new((self.x >> other.x, self.y >> other.y))
  }
}

impl ShrAssign for UVec2 {
  fn shr_assign(&mut self, other: Self) {
    self.x >>= other.x;
    self.y >>= other.y;
  }
}

impl Display for UVec2 {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    write!(formatter, "{},{}", self.x, self.y)
  }
}

impl From<DVec2> for UVec2 {
  fn from(vec: DVec2) -> Self {
    Self::new((vec.get_x() as _, vec.get_y() as _))
  }
}

impl From<FVec2> for UVec2 {
  fn from(vec: FVec2) -> Self {
    Self::new((vec.get_x() as _, vec.get_y() as _))
  }
}

impl From<IVec2> for UVec2 {
  fn from(vec: IVec2) -> Self {
    Self::new((vec.get_x() as _, vec.get_y() as _))
  }
}
