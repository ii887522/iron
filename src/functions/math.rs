use crate::{DSeq, DVec2, DVec3, DVec4, FSeq, FVec2, FVec3, FVec4};
pub trait ApproxEq {
  type Rhs;

  fn approx_eq(&self, other: Self::Rhs) -> bool;
}

impl ApproxEq for f64 {
  type Rhs = Self;

  fn approx_eq(&self, other: Self::Rhs) -> bool {
    ((1.0 - f64::EPSILON) * self..=(1.0 + f64::EPSILON) * self).contains(&other)
  }
}

impl ApproxEq for f32 {
  type Rhs = Self;

  fn approx_eq(&self, other: Self::Rhs) -> bool {
    ((1.0 - f32::EPSILON) * self..=(1.0 + f32::EPSILON) * self).contains(&other)
  }
}

pub trait PropChecker {
  /// It checks whether this is an odd number.
  ///
  /// It returns true if this is an odd number, false otherwise.
  fn is_odd(&self) -> bool;

  /// It checks whether this is an even number.
  ///
  /// It returns true if this is an even number, false otherwise.
  fn is_even(&self) -> bool;

  /// It checks whether this is a prime number.
  ///
  /// It returns true if this is a prime number, false otherwise.
  fn is_prime(&self) -> bool;

  /// It checks whether this is equal to 2 to the power of n where n is an integer.
  ///
  /// It returns true if this is equal to 2 to the power of n where n is an integer, false otherwise.
  fn is_pow_of_2(&self) -> bool;
}

impl PropChecker for i32 {
  fn is_odd(&self) -> bool {
    self & 1 == 1
  }

  fn is_even(&self) -> bool {
    self & 1 == 0
  }

  fn is_prime(&self) -> bool {
    if *self <= 1 {
      return false;
    }
    if *self == 2 {
      return true;
    }
    if self.is_even() {
      return false;
    }
    if *self < 9 {
      return true;
    }
    let sqrt = (*self as f64).sqrt().floor() as i32;
    if sqrt * sqrt == *self {
      return false;
    }
    if *self < 15 {
      return true;
    }
    for i in (3..=sqrt).step_by(2) {
      if (*self as f64 / i as f64).floor() as i32 * i == *self {
        return false;
      }
    }
    true
  }

  fn is_pow_of_2(&self) -> bool {
    *self & (*self - 1) == 0
  }
}

/// It performs a linear interpolation to find a value at time `t` when t = 0, value is `a`; t = 1, value is `b`.
///
/// `t`: The time to requset for a linear interpolation result.
///
/// `a`: The value when `t` = 0 .
///
/// `b`: The value when `t` = 1 .
///
/// It returns a value at time `t`.
pub fn lerp_f32(t: f32, a: f32, b: f32) -> f32 {
  debug_assert!(!t.is_nan(), "t must be a number!");
  debug_assert!(!a.is_nan(), "a must be a number!");
  debug_assert!(!b.is_nan(), "b must be a number!");
  f32::mul_add(b - a, t, a)
}

/// It performs a linear interpolation to find a value at time `t` when t = 0, value is `a`; t = 1, value is `b`.
///
/// `t`: The time to requset for a linear interpolation result.
///
/// `a`: The value when `t` = 0 .
///
/// `b`: The value when `t` = 1 .
///
/// It returns a value at time `t`.
pub fn lerp_f64(t: f64, a: f64, b: f64) -> f64 {
  debug_assert!(!t.is_nan(), "t must be a number!");
  debug_assert!(!a.is_nan(), "a must be a number!");
  debug_assert!(!b.is_nan(), "b must be a number!");
  f64::mul_add(b - a, t, a)
}

/// It performs a linear interpolation to find a value at time `t` when t = 0, value is `a`; t = 1, value is `b`.
///
/// `t`: The time to requset for a linear interpolation result.
///
/// `a`: The value when `t` = 0 .
///
/// `b`: The value when `t` = 1 .
///
/// It returns a value at time `t`.
pub fn lerp_fvec2(t: f32, a: FVec2, b: FVec2) -> FVec2 {
  debug_assert!(!t.is_nan(), "t must be a number!");
  FVec2::new((
    lerp_f32(t, a.get_x(), b.get_x()),
    lerp_f32(t, a.get_y(), b.get_y()),
  ))
}

/// It performs a linear interpolation to find a value at time `t` when t = 0, value is `a`; t = 1, value is `b`.
///
/// `t`: The time to requset for a linear interpolation result.
///
/// `a`: The value when `t` = 0 .
///
/// `b`: The value when `t` = 1 .
///
/// It returns a value at time `t`.
pub fn lerp_dvec2(t: f64, a: DVec2, b: DVec2) -> DVec2 {
  debug_assert!(!t.is_nan(), "t must be a number!");
  DVec2::new((
    lerp_f64(t, a.get_x(), b.get_x()),
    lerp_f64(t, a.get_y(), b.get_y()),
  ))
}

/// It performs a linear interpolation to find a value at time `t` when t = 0, value is `a`; t = 1, value is `b`.
///
/// `t`: The time to requset for a linear interpolation result.
///
/// `a`: The value when `t` = 0 .
///
/// `b`: The value when `t` = 1 .
///
/// It returns a value at time `t`.
pub fn lerp_fvec3(t: f32, a: FVec3, b: FVec3) -> FVec3 {
  debug_assert!(!t.is_nan(), "t must be a number!");
  FVec3::new((
    lerp_f32(t, a.get_x(), b.get_x()),
    lerp_f32(t, a.get_y(), b.get_y()),
    lerp_f32(t, a.get_z(), b.get_z()),
  ))
}

/// It performs a linear interpolation to find a value at time `t` when t = 0, value is `a`; t = 1, value is `b`.
///
/// `t`: The time to requset for a linear interpolation result.
///
/// `a`: The value when `t` = 0 .
///
/// `b`: The value when `t` = 1 .
///
/// It returns a value at time `t`.
pub fn lerp_dvec3(t: f64, a: DVec3, b: DVec3) -> DVec3 {
  debug_assert!(!t.is_nan(), "t must be a number!");
  DVec3::new((
    lerp_f64(t, a.get_x(), b.get_x()),
    lerp_f64(t, a.get_y(), b.get_y()),
    lerp_f64(t, a.get_z(), b.get_z()),
  ))
}

/// It performs a linear interpolation to find a value at time `t` when t = 0, value is `a`; t = 1, value is `b`.
///
/// `t`: The time to requset for a linear interpolation result.
///
/// `a`: The value when `t` = 0 .
///
/// `b`: The value when `t` = 1 .
///
/// It returns a value at time `t`.
pub fn lerp_fvec4(t: f32, a: FVec4, b: FVec4) -> FVec4 {
  debug_assert!(!t.is_nan(), "t must be a number!");
  FVec4::new((
    lerp_f32(t, a.get_x(), b.get_x()),
    lerp_f32(t, a.get_y(), b.get_y()),
    lerp_f32(t, a.get_z(), b.get_z()),
    lerp_f32(t, a.get_w(), b.get_w()),
  ))
}

/// It performs a linear interpolation to find a value at time `t` when t = 0, value is `a`; t = 1, value is `b`.
///
/// `t`: The time to requset for a linear interpolation result.
///
/// `a`: The value when `t` = 0 .
///
/// `b`: The value when `t` = 1 .
///
/// It returns a value at time `t`.
pub fn lerp_dvec4(t: f64, a: DVec4, b: DVec4) -> DVec4 {
  debug_assert!(!t.is_nan(), "t must be a number!");
  DVec4::new((
    lerp_f64(t, a.get_x(), b.get_x()),
    lerp_f64(t, a.get_y(), b.get_y()),
    lerp_f64(t, a.get_z(), b.get_z()),
    lerp_f64(t, a.get_w(), b.get_w()),
  ))
}

pub trait ExtremumFinder {
  type Item;

  /// It finds a minimum positive value and its associated index from this object.
  fn min_pos(&self) -> Option<(usize, Self::Item)>;

  /// It finds a maximum positive value and its associated index from this object.
  fn max_pos(&self) -> Option<(usize, Self::Item)>;

  /// It finds a minimum negative value and its associated index from this object.
  fn min_neg(&self) -> Option<(usize, Self::Item)>;

  /// It finds a maximum negative value and its associated index from this object.
  fn max_neg(&self) -> Option<(usize, Self::Item)>;
}

impl ExtremumFinder for [f32] {
  type Item = f32;

  fn min_pos(&self) -> Option<(usize, Self::Item)> {
    debug_assert!(
      self.iter().all(|&value| !value.is_nan()),
      "This array must contains number only!"
    );
    let mut min_pos = f32::INFINITY;
    let mut index = None;
    for (i, &value) in self.iter().enumerate() {
      if value < 0.0 || value >= min_pos {
        continue;
      }
      min_pos = value;
      index = Some(i);
    }
    index.map(|index| (index, min_pos))
  }

  fn max_pos(&self) -> Option<(usize, Self::Item)> {
    debug_assert!(
      self.iter().all(|&value| !value.is_nan()),
      "This array must contains number only!"
    );
    let mut max_pos = f32::NEG_INFINITY;
    let mut index = None;
    for (i, &value) in self.iter().enumerate() {
      if value < 0.0 || value <= max_pos {
        continue;
      }
      max_pos = value;
      index = Some(i);
    }
    index.map(|index| (index, max_pos))
  }

  fn min_neg(&self) -> Option<(usize, Self::Item)> {
    debug_assert!(
      self.iter().all(|&value| !value.is_nan()),
      "This array must contains number only!"
    );
    let mut min_neg = f32::INFINITY;
    let mut index = None;
    for (i, &value) in self.iter().enumerate() {
      if value >= 0.0 || value >= min_neg {
        continue;
      }
      min_neg = value;
      index = Some(i);
    }
    index.map(|index| (index, min_neg))
  }

  fn max_neg(&self) -> Option<(usize, Self::Item)> {
    debug_assert!(
      self.iter().all(|&value| !value.is_nan()),
      "This array must contains number only!"
    );
    let mut max_neg = f32::NEG_INFINITY;
    let mut index = None;
    for (i, &value) in self.iter().enumerate() {
      if value >= 0.0 || value <= max_neg {
        continue;
      }
      max_neg = value;
      index = Some(i);
    }
    index.map(|index| (index, max_neg))
  }
}

impl ExtremumFinder for [f64] {
  type Item = f64;

  fn min_pos(&self) -> Option<(usize, Self::Item)> {
    debug_assert!(
      self.iter().all(|&value| !value.is_nan()),
      "This array must contains number only!"
    );
    let mut min_pos = f64::INFINITY;
    let mut index = None;
    for (i, &value) in self.iter().enumerate() {
      if value < 0.0 || value >= min_pos {
        continue;
      }
      min_pos = value;
      index = Some(i);
    }
    index.map(|index| (index, min_pos))
  }

  fn max_pos(&self) -> Option<(usize, Self::Item)> {
    debug_assert!(
      self.iter().all(|&value| !value.is_nan()),
      "This array must contains number only!"
    );
    let mut max_pos = f64::NEG_INFINITY;
    let mut index = None;
    for (i, &value) in self.iter().enumerate() {
      if value < 0.0 || value <= max_pos {
        continue;
      }
      max_pos = value;
      index = Some(i);
    }
    index.map(|index| (index, max_pos))
  }

  fn min_neg(&self) -> Option<(usize, Self::Item)> {
    debug_assert!(
      self.iter().all(|&value| !value.is_nan()),
      "This array must contains number only!"
    );
    let mut min_neg = f64::INFINITY;
    let mut index = None;
    for (i, &value) in self.iter().enumerate() {
      if value >= 0.0 || value >= min_neg {
        continue;
      }
      min_neg = value;
      index = Some(i);
    }
    index.map(|index| (index, min_neg))
  }

  fn max_neg(&self) -> Option<(usize, Self::Item)> {
    debug_assert!(
      self.iter().all(|&value| !value.is_nan()),
      "This array must contains number only!"
    );
    let mut max_neg = f64::NEG_INFINITY;
    let mut index = None;
    for (i, &value) in self.iter().enumerate() {
      if value >= 0.0 || value <= max_neg {
        continue;
      }
      max_neg = value;
      index = Some(i);
    }
    index.map(|index| (index, max_neg))
  }
}

/// It linearly maps the `value` from the first sequence given to the second sequence given.
///
/// `value`: The value to map from.
///
/// `from`: The sequence that the `value` belongs to.
///
/// `to`: The sequence that the `value` maps to.
///
/// It returns a mapped value that belongs to the second sequence given.
pub fn linear_map_f32(value: f32, from: FSeq, to: FSeq) -> f32 {
  debug_assert!(!value.is_nan(), "value must be a number!");
  to.unnormalize(from.normalize(value))
}

/// It linearly maps the `value` from the first sequence given to the second sequence given.
///
/// `value`: The value to map from.
///
/// `from`: The sequence that the `value` belongs to.
///
/// `to`: The sequence that the `value` maps to.
///
/// It returns a mapped value that belongs to the second sequence given.
pub fn linear_map_f64(value: f64, from: DSeq, to: DSeq) -> f64 {
  debug_assert!(!value.is_nan(), "value must be a number!");
  to.unnormalize(from.normalize(value))
}

/// It linearly maps the `value` from the first region given to the second region given.
///
/// `value` The value to map from.
///
/// `from_position`: The position of the region that the `value` belongs to.
///
/// `from_size`: The size of the region that the `value` belongs to.
///
/// `to_position`: The position of the region that the `value` maps to.
///
/// `to_size`: The size of the region that the `value` maps to.
///
/// It returns a mapped value that belongs to the second region given.
pub fn linear_map_fvec2(
  value: FVec2,
  from_position: FVec2,
  from_size: FVec2,
  to_position: FVec2,
  to_size: FVec2,
) -> FVec2 {
  FVec2::new((
    linear_map_f32(
      value.get_x(),
      FSeq::new(
        from_position.get_x(),
        from_position.get_x() + from_size.get_x(),
      ),
      FSeq::new(to_position.get_x(), to_position.get_x() + to_size.get_x()),
    ),
    linear_map_f32(
      value.get_y(),
      FSeq::new(
        from_position.get_y(),
        from_position.get_y() + from_size.get_y(),
      ),
      FSeq::new(to_position.get_y(), to_position.get_y() + to_size.get_y()),
    ),
  ))
}

/// It linearly maps the `value` from the first region given to the second region given.
///
/// `value` The value to map from.
///
/// `from_position`: The position of the region that the `value` belongs to.
///
/// `from_size`: The size of the region that the `value` belongs to.
///
/// `to_position`: The position of the region that the `value` maps to.
///
/// `to_size`: The size of the region that the `value` maps to.
///
/// It returns a mapped value that belongs to the second region given.
pub fn linear_map_dvec2(
  value: DVec2,
  from_position: DVec2,
  from_size: DVec2,
  to_position: DVec2,
  to_size: DVec2,
) -> DVec2 {
  DVec2::new((
    linear_map_f64(
      value.get_x(),
      DSeq::new(
        from_position.get_x(),
        from_position.get_x() + from_size.get_x(),
      ),
      DSeq::new(to_position.get_x(), to_position.get_x() + to_size.get_x()),
    ),
    linear_map_f64(
      value.get_y(),
      DSeq::new(
        from_position.get_y(),
        from_position.get_y() + from_size.get_y(),
      ),
      DSeq::new(to_position.get_y(), to_position.get_y() + to_size.get_y()),
    ),
  ))
}
