use std::{
  fmt::{self, Display, Formatter},
  ops::{
    Add, AddAssign, BitAnd, BitAndAssign, Div, DivAssign, Mul, MulAssign, Shl, ShlAssign, Shr,
    ShrAssign, Sub, SubAssign,
  },
};

use crate::{DVec4, FVec4, UVec4};

#[derive(Copy, Clone, Debug)]
pub struct Arg {
  x: i32,
  y: i32,
  z: i32,
  w: i32,
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

impl From<i32> for Arg {
  fn from(value: i32) -> Self {
    Self {
      x: value,
      y: value,
      z: value,
      w: value,
    }
  }
}

impl From<(i32, i32, i32, i32)> for Arg {
  fn from((x, y, z, w): (i32, i32, i32, i32)) -> Self {
    Self { x, y, z, w }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IVec4 {
  x: i32,
  y: i32,
  z: i32,
  w: i32,
}

impl IVec4 {
  pub fn new(arg: impl Into<Arg>) -> Self {
    let Arg { x, y, z, w } = arg.into();
    Self { x, y, z, w }
  }

  pub const fn get_x(&self) -> i32 {
    self.x
  }

  pub fn set_x(&mut self, value: i32) {
    self.x = value;
  }

  pub fn with_x(&self, value: i32) -> Self {
    Self::new((value, self.y, self.z, self.w))
  }

  pub const fn get_y(&self) -> i32 {
    self.y
  }

  pub fn set_y(&mut self, value: i32) {
    self.y = value;
  }

  pub fn with_y(&self, value: i32) -> Self {
    Self::new((self.x, value, self.z, self.w))
  }

  pub const fn get_z(&self) -> i32 {
    self.z
  }

  pub fn set_z(&mut self, value: i32) {
    self.z = value;
  }

  pub fn with_z(&self, value: i32) -> Self {
    Self::new((self.x, self.y, value, self.w))
  }

  pub const fn get_w(&self) -> i32 {
    self.w
  }

  pub fn set_w(&mut self, value: i32) {
    self.w = value;
  }

  pub fn with_w(&self, value: i32) -> Self {
    Self::new((self.x, self.y, self.z, value))
  }

  pub fn set(&mut self, other: Self) {
    self.x = other.x;
    self.y = other.y;
    self.z = other.z;
    self.w = other.w;
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

impl Add for IVec4 {
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

impl AddAssign for IVec4 {
  fn add_assign(&mut self, other: Self) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
    self.w += other.w;
  }
}

impl Sub for IVec4 {
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

impl SubAssign for IVec4 {
  fn sub_assign(&mut self, other: Self) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
    self.w -= other.w;
  }
}

impl Mul<i32> for IVec4 {
  type Output = Self;

  fn mul(self, value: i32) -> Self::Output {
    Self::new((
      self.x * value,
      self.y * value,
      self.z * value,
      self.w * value,
    ))
  }
}

impl MulAssign<i32> for IVec4 {
  fn mul_assign(&mut self, value: i32) {
    self.x *= value;
    self.y *= value;
    self.z *= value;
    self.w *= value;
  }
}

impl Div<i32> for IVec4 {
  type Output = Self;

  fn div(self, value: i32) -> Self::Output {
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

impl DivAssign<i32> for IVec4 {
  fn div_assign(&mut self, value: i32) {
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

impl BitAnd<i32> for IVec4 {
  type Output = IVec4;

  fn bitand(self, value: i32) -> Self::Output {
    Self::new((
      self.x & value,
      self.y & value,
      self.z & value,
      self.w & value,
    ))
  }
}

impl BitAndAssign<i32> for IVec4 {
  fn bitand_assign(&mut self, value: i32) {
    self.x &= value;
    self.y &= value;
    self.z &= value;
    self.w &= value;
  }
}

impl BitAnd for IVec4 {
  type Output = IVec4;

  fn bitand(self, other: Self) -> Self::Output {
    Self::new((
      self.x & other.x,
      self.y & other.y,
      self.z & other.z,
      self.w & other.w,
    ))
  }
}

impl BitAndAssign for IVec4 {
  fn bitand_assign(&mut self, other: Self) {
    self.x &= other.x;
    self.y &= other.y;
    self.z &= other.z;
    self.w &= other.w;
  }
}

impl Shl<i32> for IVec4 {
  type Output = IVec4;

  fn shl(self, value: i32) -> Self::Output {
    Self::new((
      self.x << value,
      self.y << value,
      self.z << value,
      self.w << value,
    ))
  }
}

impl ShlAssign<i32> for IVec4 {
  fn shl_assign(&mut self, value: i32) {
    self.x <<= value;
    self.y <<= value;
    self.z <<= value;
    self.w <<= value;
  }
}

impl Shl for IVec4 {
  type Output = IVec4;

  fn shl(self, other: Self) -> Self::Output {
    Self::new((
      self.x << other.x,
      self.y << other.y,
      self.z << other.z,
      self.w << other.w,
    ))
  }
}

impl ShlAssign for IVec4 {
  fn shl_assign(&mut self, other: Self) {
    self.x <<= other.x;
    self.y <<= other.y;
    self.z <<= other.z;
    self.w <<= other.w;
  }
}

impl Shr<i32> for IVec4 {
  type Output = IVec4;

  fn shr(self, value: i32) -> Self::Output {
    Self::new((
      self.x >> value,
      self.y >> value,
      self.z >> value,
      self.w >> value,
    ))
  }
}

impl ShrAssign<i32> for IVec4 {
  fn shr_assign(&mut self, value: i32) {
    self.x >>= value;
    self.y >>= value;
    self.z >>= value;
    self.w >>= value;
  }
}

impl Shr for IVec4 {
  type Output = IVec4;

  fn shr(self, other: Self) -> Self::Output {
    Self::new((
      self.x >> other.x,
      self.y >> other.y,
      self.z >> other.z,
      self.w >> other.w,
    ))
  }
}

impl ShrAssign for IVec4 {
  fn shr_assign(&mut self, other: Self) {
    self.x >>= other.x;
    self.y >>= other.y;
    self.z >>= other.z;
    self.w >>= other.w;
  }
}

impl Display for IVec4 {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    write!(formatter, "{},{},{},{}", self.x, self.y, self.z, self.w)
  }
}

impl From<DVec4> for IVec4 {
  fn from(vec: DVec4) -> Self {
    Self::new((
      vec.get_x() as _,
      vec.get_y() as _,
      vec.get_z() as _,
      vec.get_w() as _,
    ))
  }
}

impl From<FVec4> for IVec4 {
  fn from(vec: FVec4) -> Self {
    Self::new((
      vec.get_x() as _,
      vec.get_y() as _,
      vec.get_z() as _,
      vec.get_w() as _,
    ))
  }
}

impl From<UVec4> for IVec4 {
  fn from(vec: UVec4) -> Self {
    Self::new((
      vec.get_x() as _,
      vec.get_y() as _,
      vec.get_z() as _,
      vec.get_w() as _,
    ))
  }
}
