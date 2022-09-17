use std::{
  fmt::{self, Display, Formatter},
  ops::{
    Add, AddAssign, BitAnd, BitAndAssign, Div, DivAssign, Mul, MulAssign, Shl, ShlAssign, Shr,
    ShrAssign, Sub, SubAssign,
  },
};

use crate::{DVec4, FVec4, IVec4};

#[derive(Copy, Clone, Debug)]
pub struct Arg {
  x: u32,
  y: u32,
  z: u32,
  w: u32,
}

impl From<()> for Arg {
  fn from(_: ()) -> Self {
    Self {
      x: 0,
      y: 0,
      z: 0,
      w: 0,
    }
  }
}

impl From<u32> for Arg {
  fn from(value: u32) -> Self {
    Self {
      x: value,
      y: value,
      z: value,
      w: value,
    }
  }
}

impl From<(u32, u32, u32, u32)> for Arg {
  fn from((x, y, z, w): (u32, u32, u32, u32)) -> Self {
    Self { x, y, z, w }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct UVec4 {
  x: u32,
  y: u32,
  z: u32,
  w: u32,
}

impl UVec4 {
  pub fn new(arg: impl Into<Arg>) -> Self {
    let Arg { x, y, z, w } = arg.into();
    Self { x, y, z, w }
  }

  pub const fn get_x(&self) -> u32 {
    self.x
  }

  pub fn set_x(&mut self, value: u32) {
    self.x = value;
  }

  pub fn with_x(&self, value: u32) -> Self {
    Self::new((value, self.y, self.z, self.w))
  }

  pub const fn get_y(&self) -> u32 {
    self.y
  }

  pub fn set_y(&mut self, value: u32) {
    self.y = value;
  }

  pub fn with_y(&self, value: u32) -> Self {
    Self::new((self.x, value, self.z, self.w))
  }

  pub const fn get_z(&self) -> u32 {
    self.z
  }

  pub fn set_z(&mut self, value: u32) {
    self.z = value;
  }

  pub fn with_z(&self, value: u32) -> Self {
    Self::new((self.x, self.y, value, self.w))
  }

  pub const fn get_w(&self) -> u32 {
    self.w
  }

  pub fn set_w(&mut self, value: u32) {
    self.w = value;
  }

  pub fn with_w(&self, value: u32) -> Self {
    Self::new((self.x, self.y, self.z, value))
  }

  pub fn set(&mut self, other: Self) {
    self.x = other.x;
    self.y = other.y;
    self.z = other.z;
    self.w = other.w;
  }
}

impl Add for UVec4 {
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

impl AddAssign for UVec4 {
  fn add_assign(&mut self, other: Self) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
    self.w += other.w;
  }
}

impl Sub for UVec4 {
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

impl SubAssign for UVec4 {
  fn sub_assign(&mut self, other: Self) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
    self.w -= other.w;
  }
}

impl Mul<u32> for UVec4 {
  type Output = Self;

  fn mul(self, value: u32) -> Self::Output {
    Self::new((
      self.x * value,
      self.y * value,
      self.z * value,
      self.w * value,
    ))
  }
}

impl MulAssign<u32> for UVec4 {
  fn mul_assign(&mut self, value: u32) {
    self.x *= value;
    self.y *= value;
    self.z *= value;
    self.w *= value;
  }
}

impl Mul for UVec4 {
  type Output = Self;

  fn mul(self, other: Self) -> Self::Output {
    Self::new((
      self.x * other.x,
      self.y * other.y,
      self.z * other.z,
      self.w * other.w,
    ))
  }
}

impl MulAssign for UVec4 {
  fn mul_assign(&mut self, other: Self) {
    self.x *= other.x;
    self.y *= other.y;
    self.z *= other.z;
    self.w *= other.w;
  }
}

impl Div<u32> for UVec4 {
  type Output = Self;

  fn div(self, value: u32) -> Self::Output {
    debug_assert_ne!(
      value, 0,
      "value must not be equal to 0 to avoid causing division by zero error!"
    );
    Self::new((
      self.x / value,
      self.y / value,
      self.z / value,
      self.w / value,
    ))
  }
}

impl DivAssign<u32> for UVec4 {
  fn div_assign(&mut self, value: u32) {
    debug_assert_ne!(
      value, 0,
      "value must not be equal to 0 to avoid causing division by zero error!"
    );
    self.x /= value;
    self.y /= value;
    self.z /= value;
    self.w /= value;
  }
}

impl Div for UVec4 {
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
    debug_assert_ne!(
      other.w, 0,
      "other.w must not be equal to 0 to avoid causing division by zero error!"
    );
    Self::new((
      self.x / other.x,
      self.y / other.y,
      self.z / other.z,
      self.w / other.w,
    ))
  }
}

impl DivAssign for UVec4 {
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
    debug_assert_ne!(
      other.w, 0,
      "other.w must not be equal to 0 to avoid causing division by zero error!"
    );
    self.x /= other.x;
    self.y /= other.y;
    self.z /= other.z;
    self.w /= other.w;
  }
}

impl BitAnd<u32> for UVec4 {
  type Output = UVec4;

  fn bitand(self, value: u32) -> Self::Output {
    Self::new((
      self.x & value,
      self.y & value,
      self.z & value,
      self.w & value,
    ))
  }
}

impl BitAndAssign<u32> for UVec4 {
  fn bitand_assign(&mut self, value: u32) {
    self.x &= value;
    self.y &= value;
    self.z &= value;
    self.w &= value;
  }
}

impl BitAnd for UVec4 {
  type Output = UVec4;

  fn bitand(self, other: Self) -> Self::Output {
    Self::new((
      self.x & other.x,
      self.y & other.y,
      self.z & other.z,
      self.w & other.w,
    ))
  }
}

impl BitAndAssign for UVec4 {
  fn bitand_assign(&mut self, other: Self) {
    self.x &= other.x;
    self.y &= other.y;
    self.z &= other.z;
    self.w &= other.w;
  }
}

impl Shl<u32> for UVec4 {
  type Output = UVec4;

  fn shl(self, value: u32) -> Self::Output {
    Self::new((
      self.x << value,
      self.y << value,
      self.z << value,
      self.w << value,
    ))
  }
}

impl ShlAssign<u32> for UVec4 {
  fn shl_assign(&mut self, value: u32) {
    self.x <<= value;
    self.y <<= value;
    self.z <<= value;
    self.w <<= value;
  }
}

impl Shl for UVec4 {
  type Output = UVec4;

  fn shl(self, other: Self) -> Self::Output {
    Self::new((
      self.x << other.x,
      self.y << other.y,
      self.z << other.z,
      self.w << other.w,
    ))
  }
}

impl ShlAssign for UVec4 {
  fn shl_assign(&mut self, other: Self) {
    self.x <<= other.x;
    self.y <<= other.y;
    self.z <<= other.z;
    self.w <<= other.w;
  }
}

impl Shr<u32> for UVec4 {
  type Output = UVec4;

  fn shr(self, value: u32) -> Self::Output {
    Self::new((
      self.x >> value,
      self.y >> value,
      self.z >> value,
      self.w >> value,
    ))
  }
}

impl ShrAssign<u32> for UVec4 {
  fn shr_assign(&mut self, value: u32) {
    self.x >>= value;
    self.y >>= value;
    self.z >>= value;
    self.w >>= value;
  }
}

impl Shr for UVec4 {
  type Output = UVec4;

  fn shr(self, other: Self) -> Self::Output {
    Self::new((
      self.x >> other.x,
      self.y >> other.y,
      self.z >> other.z,
      self.w >> other.w,
    ))
  }
}

impl ShrAssign for UVec4 {
  fn shr_assign(&mut self, other: Self) {
    self.x >>= other.x;
    self.y >>= other.y;
    self.z >>= other.z;
    self.w >>= other.w;
  }
}

impl Display for UVec4 {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    write!(formatter, "{},{},{},{}", self.x, self.y, self.z, self.w)
  }
}

impl From<DVec4> for UVec4 {
  fn from(vec: DVec4) -> Self {
    Self::new((
      vec.get_x() as _,
      vec.get_y() as _,
      vec.get_z() as _,
      vec.get_w() as _,
    ))
  }
}

impl From<FVec4> for UVec4 {
  fn from(vec: FVec4) -> Self {
    Self::new((
      vec.get_x() as _,
      vec.get_y() as _,
      vec.get_z() as _,
      vec.get_w() as _,
    ))
  }
}

impl From<IVec4> for UVec4 {
  fn from(vec: IVec4) -> Self {
    Self::new((
      vec.get_x() as _,
      vec.get_y() as _,
      vec.get_z() as _,
      vec.get_w() as _,
    ))
  }
}
