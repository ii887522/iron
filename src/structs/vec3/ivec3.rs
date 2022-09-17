use std::{
  fmt::{self, Display, Formatter},
  ops::{
    Add, AddAssign, BitAnd, BitAndAssign, Div, DivAssign, Mul, MulAssign, Shl, ShlAssign, Shr,
    ShrAssign, Sub, SubAssign,
  },
};

use crate::{DVec3, FVec3, UVec3};

#[derive(Copy, Clone, Debug)]
pub struct Arg {
  x: i32,
  y: i32,
  z: i32,
}

impl From<()> for Arg {
  fn from(_: ()) -> Self {
    Self { x: 0, y: 0, z: 0 }
  }
}

impl From<i32> for Arg {
  fn from(value: i32) -> Self {
    Self {
      x: value,
      y: value,
      z: value,
    }
  }
}

impl From<(i32, i32, i32)> for Arg {
  fn from((x, y, z): (i32, i32, i32)) -> Self {
    Self { x, y, z }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct IVec3 {
  x: i32,
  y: i32,
  z: i32,
}

impl IVec3 {
  pub fn new(arg: impl Into<Arg>) -> Self {
    let Arg { x, y, z } = arg.into();
    Self { x, y, z }
  }

  pub const fn get_x(&self) -> i32 {
    self.x
  }

  pub fn set_x(&mut self, value: i32) {
    self.x = value;
  }

  pub fn with_x(&self, value: i32) -> Self {
    Self::new((value, self.y, self.z))
  }

  pub const fn get_y(&self) -> i32 {
    self.y
  }

  pub fn set_y(&mut self, value: i32) {
    self.y = value;
  }

  pub fn with_y(&self, value: i32) -> Self {
    Self::new((self.x, value, self.z))
  }

  pub const fn get_z(&self) -> i32 {
    self.z
  }

  pub fn set_z(&mut self, value: i32) {
    self.z = value;
  }

  pub fn with_z(&self, value: i32) -> Self {
    Self::new((self.x, self.y, value))
  }

  pub fn set(&mut self, other: Self) {
    self.x = other.x;
    self.y = other.y;
    self.z = other.z;
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

impl Add for IVec3 {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self::new((self.x + other.x, self.y + other.y, self.z + other.z))
  }
}

impl AddAssign for IVec3 {
  fn add_assign(&mut self, other: Self) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}

impl Sub for IVec3 {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self::new((self.x - other.x, self.y - other.y, self.z - other.z))
  }
}

impl SubAssign for IVec3 {
  fn sub_assign(&mut self, other: Self) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
  }
}

impl Mul<i32> for IVec3 {
  type Output = Self;

  fn mul(self, value: i32) -> Self::Output {
    Self::new((self.x * value, self.y * value, self.z * value))
  }
}

impl MulAssign<i32> for IVec3 {
  fn mul_assign(&mut self, value: i32) {
    self.x *= value;
    self.y *= value;
    self.z *= value;
  }
}

impl Mul for IVec3 {
  type Output = Self;

  fn mul(self, other: Self) -> Self::Output {
    Self::new((self.x * other.x, self.y * other.y, self.z * other.z))
  }
}

impl MulAssign for IVec3 {
  fn mul_assign(&mut self, other: Self) {
    self.x *= other.x;
    self.y *= other.y;
    self.z *= other.z;
  }
}

impl Div<i32> for IVec3 {
  type Output = Self;

  fn div(self, value: i32) -> Self::Output {
    debug_assert_ne!(
      value, 0,
      "value must not be equal to 0 to avoid causing division by zero error!"
    );
    Self::new((self.x / value, self.y / value, self.z / value))
  }
}

impl DivAssign<i32> for IVec3 {
  fn div_assign(&mut self, value: i32) {
    debug_assert_ne!(
      value, 0,
      "value must not be equal to 0 to avoid causing division by zero error!"
    );
    self.x /= value;
    self.y /= value;
    self.z /= value;
  }
}

impl Div for IVec3 {
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
    debug_assert_ne!(
      other.z, 0,
      "other.z must not be equal to 0 to avoid causing division by zero error!"
    );
    Self::new((self.x / other.x, self.y / other.y, self.z / other.z))
  }
}

impl DivAssign for IVec3 {
  fn div_assign(&mut self, other: Self) {
    debug_assert_ne!(
      other.x, 0,
      "other.x must not be equal to 0 to avoid causing division by zero error!"
    );
    debug_assert_ne!(
      other.y, 0,
      "other.y must not be equal to 0 to avoid causing division by zero error!"
    );
    debug_assert_ne!(
      other.z, 0,
      "other.z must not be equal to 0 to avoid causing division by zero error!"
    );
    self.x /= other.x;
    self.y /= other.y;
    self.z /= other.z;
  }
}

impl BitAnd<i32> for IVec3 {
  type Output = IVec3;

  fn bitand(self, value: i32) -> Self::Output {
    Self::new((self.x & value, self.y & value, self.z & value))
  }
}

impl BitAndAssign<i32> for IVec3 {
  fn bitand_assign(&mut self, value: i32) {
    self.x &= value;
    self.y &= value;
    self.z &= value;
  }
}

impl BitAnd for IVec3 {
  type Output = IVec3;

  fn bitand(self, other: Self) -> Self::Output {
    Self::new((self.x & other.x, self.y & other.y, self.z & other.z))
  }
}

impl BitAndAssign for IVec3 {
  fn bitand_assign(&mut self, other: Self) {
    self.x &= other.x;
    self.y &= other.y;
    self.z &= other.z;
  }
}

impl Shl<i32> for IVec3 {
  type Output = IVec3;

  fn shl(self, value: i32) -> Self::Output {
    Self::new((self.x << value, self.y << value, self.z << value))
  }
}

impl ShlAssign<i32> for IVec3 {
  fn shl_assign(&mut self, value: i32) {
    self.x <<= value;
    self.y <<= value;
    self.z <<= value;
  }
}

impl Shl for IVec3 {
  type Output = IVec3;

  fn shl(self, other: Self) -> Self::Output {
    Self::new((self.x << other.x, self.y << other.y, self.z << other.z))
  }
}

impl ShlAssign for IVec3 {
  fn shl_assign(&mut self, other: Self) {
    self.x <<= other.x;
    self.y <<= other.y;
    self.z <<= other.z;
  }
}

impl Shr<i32> for IVec3 {
  type Output = IVec3;

  fn shr(self, value: i32) -> Self::Output {
    Self::new((self.x >> value, self.y >> value, self.z >> value))
  }
}

impl ShrAssign<i32> for IVec3 {
  fn shr_assign(&mut self, value: i32) {
    self.x >>= value;
    self.y >>= value;
    self.z >>= value;
  }
}

impl Shr for IVec3 {
  type Output = IVec3;

  fn shr(self, other: Self) -> Self::Output {
    Self::new((self.x >> other.x, self.y >> other.y, self.z >> other.z))
  }
}

impl ShrAssign for IVec3 {
  fn shr_assign(&mut self, other: Self) {
    self.x >>= other.x;
    self.y >>= other.y;
    self.z >>= other.z;
  }
}

impl Display for IVec3 {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    write!(formatter, "{},{},{}", self.x, self.y, self.z)
  }
}

impl From<DVec3> for IVec3 {
  fn from(vec: DVec3) -> Self {
    Self::new((vec.get_x() as _, vec.get_y() as _, vec.get_z() as _))
  }
}

impl From<FVec3> for IVec3 {
  fn from(vec: FVec3) -> Self {
    Self::new((vec.get_x() as _, vec.get_y() as _, vec.get_z() as _))
  }
}

impl From<UVec3> for IVec3 {
  fn from(vec: UVec3) -> Self {
    Self::new((vec.get_x() as _, vec.get_y() as _, vec.get_z() as _))
  }
}
