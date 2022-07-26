use iron_ingot::{DVec2, FVec2, IVec2, UVec2};

#[test]
fn test_from_u32() {
  assert_eq!(UVec2::new(0), UVec2::new((0, 0)));
  assert_eq!(UVec2::new(1), UVec2::new((1, 1)));
  assert_eq!(UVec2::new(2), UVec2::new((2, 2)));
}

#[test]
fn test_with_x() {
  assert_eq!(UVec2::new((0, 0)).with_x(0), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((1, 0)).with_x(0), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 0)).with_x(0), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 1)).with_x(0), UVec2::new((0, 1)));
  assert_eq!(UVec2::new((2, 2)).with_x(0), UVec2::new((0, 2)));
  assert_eq!(UVec2::new((2, 2)).with_x(1), UVec2::new((1, 2)));
  assert_eq!(UVec2::new((2, 2)).with_x(2), UVec2::new((2, 2)));
}

#[test]
fn test_with_y() {
  assert_eq!(UVec2::new((0, 0)).with_y(0), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((1, 0)).with_y(0), UVec2::new((1, 0)));
  assert_eq!(UVec2::new((2, 0)).with_y(0), UVec2::new((2, 0)));
  assert_eq!(UVec2::new((2, 1)).with_y(0), UVec2::new((2, 0)));
  assert_eq!(UVec2::new((2, 2)).with_y(0), UVec2::new((2, 0)));
  assert_eq!(UVec2::new((2, 2)).with_y(1), UVec2::new((2, 1)));
  assert_eq!(UVec2::new((2, 2)).with_y(2), UVec2::new((2, 2)));
}

#[test]
fn test_add() {
  assert_eq!(UVec2::new((0, 0)) + UVec2::new((0, 0)), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((1, 0)) + UVec2::new((0, 0)), UVec2::new((1, 0)));
  assert_eq!(UVec2::new((2, 0)) + UVec2::new((0, 0)), UVec2::new((2, 0)));
  assert_eq!(UVec2::new((2, 1)) + UVec2::new((0, 0)), UVec2::new((2, 1)));
  assert_eq!(UVec2::new((2, 2)) + UVec2::new((0, 0)), UVec2::new((2, 2)));
  assert_eq!(UVec2::new((2, 2)) + UVec2::new((1, 0)), UVec2::new((3, 2)));
  assert_eq!(UVec2::new((2, 2)) + UVec2::new((2, 0)), UVec2::new((4, 2)));
  assert_eq!(UVec2::new((2, 2)) + UVec2::new((2, 1)), UVec2::new((4, 3)));
  assert_eq!(UVec2::new((2, 2)) + UVec2::new((2, 2)), UVec2::new((4, 4)));
}

#[test]
fn test_add_assign() {
  let mut vec = UVec2::new(());
  vec += UVec2::new((0, 0));
  assert_eq!(vec, UVec2::new((0, 0)));
  vec += UVec2::new((1, 0));
  assert_eq!(vec, UVec2::new((1, 0)));
  vec += UVec2::new((2, 0));
  assert_eq!(vec, UVec2::new((3, 0)));
  vec += UVec2::new((2, 1));
  assert_eq!(vec, UVec2::new((5, 1)));
  vec += UVec2::new((2, 2));
  assert_eq!(vec, UVec2::new((7, 3)));
}

#[test]
fn test_sub() {
  assert_eq!(UVec2::new((0, 0)) - UVec2::new((0, 0)), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((1, 0)) - UVec2::new((0, 0)), UVec2::new((1, 0)));
  assert_eq!(UVec2::new((2, 0)) - UVec2::new((0, 0)), UVec2::new((2, 0)));
  assert_eq!(UVec2::new((2, 1)) - UVec2::new((0, 0)), UVec2::new((2, 1)));
  assert_eq!(UVec2::new((2, 2)) - UVec2::new((0, 0)), UVec2::new((2, 2)));
  assert_eq!(UVec2::new((2, 2)) - UVec2::new((1, 0)), UVec2::new((1, 2)));
  assert_eq!(UVec2::new((2, 2)) - UVec2::new((2, 0)), UVec2::new((0, 2)));
  assert_eq!(UVec2::new((2, 2)) - UVec2::new((2, 1)), UVec2::new((0, 1)));
  assert_eq!(UVec2::new((2, 2)) - UVec2::new((2, 2)), UVec2::new((0, 0)));
}

#[test]
fn test_sub_assign() {
  let mut vec = UVec2::new((10, 10));
  vec -= UVec2::new((0, 0));
  assert_eq!(vec, UVec2::new((10, 10)));
  vec -= UVec2::new((1, 0));
  assert_eq!(vec, UVec2::new((9, 10)));
  vec -= UVec2::new((2, 0));
  assert_eq!(vec, UVec2::new((7, 10)));
  vec -= UVec2::new((2, 1));
  assert_eq!(vec, UVec2::new((5, 9)));
  vec -= UVec2::new((2, 2));
  assert_eq!(vec, UVec2::new((3, 7)));
}

#[test]
fn test_mul_u32() {
  assert_eq!(UVec2::new((0, 0)) * 0, UVec2::new((0, 0)));
  assert_eq!(UVec2::new((1, 0)) * 0, UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 0)) * 0, UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 1)) * 0, UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 2)) * 0, UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 2)) * 1, UVec2::new((2, 2)));
  assert_eq!(UVec2::new((2, 2)) * 2, UVec2::new((4, 4)));
}

#[test]
fn test_mul_u32_assign() {
  let mut vec = UVec2::new((1, 1));
  vec *= 1;
  assert_eq!(vec, UVec2::new((1, 1)));
  vec *= 2;
  assert_eq!(vec, UVec2::new((2, 2)));
  vec *= 3;
  assert_eq!(vec, UVec2::new((6, 6)));
}

#[test]
fn test_mul() {
  assert_eq!(UVec2::new((0, 0)) * UVec2::new((0, 0)), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((1, 0)) * UVec2::new((0, 0)), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 0)) * UVec2::new((0, 0)), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 1)) * UVec2::new((0, 0)), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 2)) * UVec2::new((0, 0)), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 2)) * UVec2::new((1, 0)), UVec2::new((2, 0)));
  assert_eq!(UVec2::new((2, 2)) * UVec2::new((2, 0)), UVec2::new((4, 0)));
  assert_eq!(UVec2::new((2, 2)) * UVec2::new((2, 1)), UVec2::new((4, 2)));
  assert_eq!(UVec2::new((2, 2)) * UVec2::new((2, 2)), UVec2::new((4, 4)));
  assert_eq!(UVec2::new((2, 2)) * UVec2::new((2, 2)), UVec2::new((4, 4)));
}

#[test]
fn test_mul_assign() {
  let mut vec = UVec2::new((1, 1));
  vec *= UVec2::new((1, 1));
  assert_eq!(vec, UVec2::new((1, 1)));
  vec *= UVec2::new((2, 1));
  assert_eq!(vec, UVec2::new((2, 1)));
  vec *= UVec2::new((3, 1));
  assert_eq!(vec, UVec2::new((6, 1)));
  vec *= UVec2::new((3, 2));
  assert_eq!(vec, UVec2::new((18, 2)));
  vec *= UVec2::new((3, 3));
  assert_eq!(vec, UVec2::new((54, 6)));
}

#[test]
fn test_div_u32() {
  assert_eq!(UVec2::new((0, 0)) / 1, UVec2::new((0, 0)));
  assert_eq!(UVec2::new((1, 0)) / 1, UVec2::new((1, 0)));
  assert_eq!(UVec2::new((2, 0)) / 1, UVec2::new((2, 0)));
  assert_eq!(UVec2::new((2, 1)) / 1, UVec2::new((2, 1)));
  assert_eq!(UVec2::new((2, 2)) / 1, UVec2::new((2, 2)));
  assert_eq!(UVec2::new((2, 2)) / 2, UVec2::new((1, 1)));
  assert_eq!(UVec2::new((2, 2)) / 4, UVec2::new((0, 0)));
}

#[test]
fn test_div_u32_assign() {
  let mut vec = UVec2::new((1, 1));
  vec /= 1;
  assert_eq!(vec, UVec2::new((1, 1)));
  vec /= 2;
  assert_eq!(vec, UVec2::new((0, 0)));
  vec /= 5;
  assert_eq!(vec, UVec2::new((0, 0)));
}

#[test]
fn test_div() {
  assert_eq!(UVec2::new((0, 0)) / UVec2::new((1, 1)), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((1, 0)) / UVec2::new((1, 1)), UVec2::new((1, 0)));
  assert_eq!(UVec2::new((2, 0)) / UVec2::new((1, 1)), UVec2::new((2, 0)));
  assert_eq!(UVec2::new((2, 1)) / UVec2::new((1, 1)), UVec2::new((2, 1)));
  assert_eq!(UVec2::new((2, 2)) / UVec2::new((1, 1)), UVec2::new((2, 2)));
  assert_eq!(UVec2::new((2, 2)) / UVec2::new((2, 1)), UVec2::new((1, 2)));
  assert_eq!(UVec2::new((2, 2)) / UVec2::new((4, 1)), UVec2::new((0, 2)));
  assert_eq!(UVec2::new((2, 2)) / UVec2::new((4, 2)), UVec2::new((0, 1)));
  assert_eq!(UVec2::new((2, 2)) / UVec2::new((4, 4)), UVec2::new((0, 0)));
}

#[test]
fn test_div_assign() {
  let mut vec = UVec2::new((1, 1));
  vec /= UVec2::new((1, 1));
  assert_eq!(vec, UVec2::new((1, 1)));
  vec /= UVec2::new((2, 1));
  assert_eq!(vec, UVec2::new((0, 1)));
  vec /= UVec2::new((5, 1));
  assert_eq!(vec, UVec2::new((0, 1)));
  vec /= UVec2::new((5, 2));
  assert_eq!(vec, UVec2::new((0, 0)));
  vec /= UVec2::new((5, 5));
  assert_eq!(vec, UVec2::new((0, 0)));
}

#[test]
fn test_set() {
  let mut vec = UVec2::new(());
  vec.set(UVec2::new((0, 0)));
  assert_eq!(vec, UVec2::new((0, 0)));
  vec.set(UVec2::new((1, 0)));
  assert_eq!(vec, UVec2::new((1, 0)));
  vec.set(UVec2::new((2, 0)));
  assert_eq!(vec, UVec2::new((2, 0)));
  vec.set(UVec2::new((2, 1)));
  assert_eq!(vec, UVec2::new((2, 1)));
  vec.set(UVec2::new((2, 2)));
  assert_eq!(vec, UVec2::new((2, 2)));
}

#[test]
fn test_bitand_u32() {
  assert_eq!(UVec2::new((0, 0)) & 0, UVec2::new((0, 0)));
  assert_eq!(UVec2::new((1, 0)) & 0, UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 0)) & 0, UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 1)) & 0, UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 2)) & 0, UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 2)) & 1, UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 2)) & 3, UVec2::new((2, 2)));
}

#[test]
fn test_bitand_assign_u32() {
  let mut vec = UVec2::new((3, 3));
  vec &= 3;
  assert_eq!(vec, UVec2::new((3, 3)));
  vec &= 1;
  assert_eq!(vec, UVec2::new((1, 1)));
  vec &= 0;
  assert_eq!(vec, UVec2::new((0, 0)));
}

#[test]
fn test_bitand_uvec2() {
  assert_eq!(UVec2::new((0, 0)) & UVec2::new((0, 0)), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((1, 0)) & UVec2::new((0, 0)), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 0)) & UVec2::new((0, 0)), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 1)) & UVec2::new((0, 0)), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 2)) & UVec2::new((0, 0)), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 2)) & UVec2::new((1, 0)), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((2, 2)) & UVec2::new((3, 0)), UVec2::new((2, 0)));
  assert_eq!(UVec2::new((2, 2)) & UVec2::new((3, 1)), UVec2::new((2, 0)));
  assert_eq!(UVec2::new((2, 2)) & UVec2::new((3, 3)), UVec2::new((2, 2)));
}

#[test]
fn test_bitand_assign_uvec2() {
  let mut vec = UVec2::new((3, 3));
  vec &= UVec2::new((3, 3));
  assert_eq!(vec, UVec2::new((3, 3)));
  vec &= UVec2::new((3, 1));
  assert_eq!(vec, UVec2::new((3, 1)));
  vec &= UVec2::new((3, 0));
  assert_eq!(vec, UVec2::new((3, 0)));
  vec &= UVec2::new((1, 0));
  assert_eq!(vec, UVec2::new((1, 0)));
  vec &= UVec2::new((0, 0));
  assert_eq!(vec, UVec2::new((0, 0)));
}

#[test]
fn test_shl_u32() {
  assert_eq!(UVec2::new((0, 0)) << 0, UVec2::new((0, 0)));
  assert_eq!(UVec2::new((1, 0)) << 0, UVec2::new((1, 0)));
  assert_eq!(UVec2::new((2, 0)) << 0, UVec2::new((2, 0)));
  assert_eq!(UVec2::new((2, 1)) << 0, UVec2::new((2, 1)));
  assert_eq!(UVec2::new((2, 2)) << 0, UVec2::new((2, 2)));
  assert_eq!(UVec2::new((2, 2)) << 1, UVec2::new((4, 4)));
  assert_eq!(UVec2::new((2, 2)) << 2, UVec2::new((8, 8)));
}

#[test]
fn test_shl_assign_u32() {
  let mut vec = UVec2::new((1, 1));
  vec <<= 0;
  assert_eq!(vec, UVec2::new((1, 1)));
  vec <<= 1;
  assert_eq!(vec, UVec2::new((2, 2)));
  vec <<= 2;
  assert_eq!(vec, UVec2::new((8, 8)));
}

#[test]
fn test_shl_uvec2() {
  assert_eq!(UVec2::new((0, 0)) << UVec2::new((0, 0)), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((1, 0)) << UVec2::new((0, 0)), UVec2::new((1, 0)));
  assert_eq!(UVec2::new((2, 0)) << UVec2::new((0, 0)), UVec2::new((2, 0)));
  assert_eq!(UVec2::new((2, 1)) << UVec2::new((0, 0)), UVec2::new((2, 1)));
  assert_eq!(UVec2::new((2, 2)) << UVec2::new((0, 0)), UVec2::new((2, 2)));
  assert_eq!(UVec2::new((2, 2)) << UVec2::new((1, 0)), UVec2::new((4, 2)));
  assert_eq!(UVec2::new((2, 2)) << UVec2::new((2, 0)), UVec2::new((8, 2)));
  assert_eq!(UVec2::new((2, 2)) << UVec2::new((2, 1)), UVec2::new((8, 4)));
  assert_eq!(UVec2::new((2, 2)) << UVec2::new((2, 2)), UVec2::new((8, 8)));
}

#[test]
fn test_shl_assign_uvec2() {
  let mut vec = UVec2::new((1, 1));
  vec <<= UVec2::new((0, 0));
  assert_eq!(vec, UVec2::new((1, 1)));
  vec <<= UVec2::new((1, 0));
  assert_eq!(vec, UVec2::new((2, 1)));
  vec <<= UVec2::new((2, 0));
  assert_eq!(vec, UVec2::new((8, 1)));
  vec <<= UVec2::new((2, 1));
  assert_eq!(vec, UVec2::new((32, 2)));
  vec <<= UVec2::new((2, 2));
  assert_eq!(vec, UVec2::new((128, 8)));
}

#[test]
fn test_shr_u32() {
  assert_eq!(UVec2::new((0, 0)) >> 0, UVec2::new((0, 0)));
  assert_eq!(UVec2::new((1, 0)) >> 0, UVec2::new((1, 0)));
  assert_eq!(UVec2::new((2, 0)) >> 0, UVec2::new((2, 0)));
  assert_eq!(UVec2::new((2, 1)) >> 0, UVec2::new((2, 1)));
  assert_eq!(UVec2::new((2, 2)) >> 0, UVec2::new((2, 2)));
  assert_eq!(UVec2::new((4, 4)) >> 1, UVec2::new((2, 2)));
  assert_eq!(UVec2::new((8, 8)) >> 2, UVec2::new((2, 2)));
}

#[test]
fn test_shr_assign_u32() {
  let mut vec = UVec2::new((8, 8));
  vec >>= 0;
  assert_eq!(vec, UVec2::new((8, 8)));
  vec >>= 1;
  assert_eq!(vec, UVec2::new((4, 4)));
  vec >>= 2;
  assert_eq!(vec, UVec2::new((1, 1)));
}

#[test]
fn test_shr_uvec2() {
  assert_eq!(UVec2::new((0, 0)) >> UVec2::new((0, 0)), UVec2::new((0, 0)));
  assert_eq!(UVec2::new((1, 0)) >> UVec2::new((0, 0)), UVec2::new((1, 0)));
  assert_eq!(UVec2::new((2, 0)) >> UVec2::new((0, 0)), UVec2::new((2, 0)));
  assert_eq!(UVec2::new((2, 1)) >> UVec2::new((0, 0)), UVec2::new((2, 1)));
  assert_eq!(UVec2::new((2, 2)) >> UVec2::new((0, 0)), UVec2::new((2, 2)));
  assert_eq!(UVec2::new((4, 2)) >> UVec2::new((1, 0)), UVec2::new((2, 2)));
  assert_eq!(UVec2::new((8, 2)) >> UVec2::new((2, 0)), UVec2::new((2, 2)));
  assert_eq!(UVec2::new((8, 4)) >> UVec2::new((2, 1)), UVec2::new((2, 2)));
  assert_eq!(UVec2::new((8, 8)) >> UVec2::new((2, 2)), UVec2::new((2, 2)));
}

#[test]
fn test_shr_assign_uvec2() {
  let mut vec = UVec2::new((128, 8));
  vec >>= UVec2::new((0, 0));
  assert_eq!(vec, UVec2::new((128, 8)));
  vec >>= UVec2::new((1, 0));
  assert_eq!(vec, UVec2::new((64, 8)));
  vec >>= UVec2::new((2, 0));
  assert_eq!(vec, UVec2::new((16, 8)));
  vec >>= UVec2::new((2, 1));
  assert_eq!(vec, UVec2::new((4, 4)));
  vec >>= UVec2::new((2, 2));
  assert_eq!(vec, UVec2::new((1, 1)));
}

#[test]
fn test_display() {
  assert_eq!(format!("{}", UVec2::new((0, 0))), "0,0");
  assert_eq!(format!("{}", UVec2::new((1, 0))), "1,0");
  assert_eq!(format!("{}", UVec2::new((2, 0))), "2,0");
  assert_eq!(format!("{}", UVec2::new((2, 1))), "2,1");
  assert_eq!(format!("{}", UVec2::new((2, 2))), "2,2");
}

#[test]
fn test_into_dvec2() {
  assert_eq!(DVec2::from(UVec2::new((0, 0))), DVec2::new((0.0, 0.0)));
  assert_eq!(DVec2::from(UVec2::new((1, 0))), DVec2::new((1.0, 0.0)));
  assert_eq!(DVec2::from(UVec2::new((2, 0))), DVec2::new((2.0, 0.0)));
  assert_eq!(DVec2::from(UVec2::new((2, 1))), DVec2::new((2.0, 1.0)));
  assert_eq!(DVec2::from(UVec2::new((2, 2))), DVec2::new((2.0, 2.0)));
}

#[test]
fn test_into_fvec2() {
  assert_eq!(FVec2::from(UVec2::new((0, 0))), FVec2::new((0.0, 0.0)));
  assert_eq!(FVec2::from(UVec2::new((1, 0))), FVec2::new((1.0, 0.0)));
  assert_eq!(FVec2::from(UVec2::new((2, 0))), FVec2::new((2.0, 0.0)));
  assert_eq!(FVec2::from(UVec2::new((2, 1))), FVec2::new((2.0, 1.0)));
  assert_eq!(FVec2::from(UVec2::new((2, 2))), FVec2::new((2.0, 2.0)));
}

#[test]
fn test_into_ivec2() {
  assert_eq!(IVec2::from(UVec2::new((0, 0))), IVec2::new((0, 0)));
  assert_eq!(IVec2::from(UVec2::new((1, 0))), IVec2::new((1, 0)));
  assert_eq!(IVec2::from(UVec2::new((2, 0))), IVec2::new((2, 0)));
  assert_eq!(IVec2::from(UVec2::new((2, 1))), IVec2::new((2, 1)));
  assert_eq!(IVec2::from(UVec2::new((2, 2))), IVec2::new((2, 2)));
}
