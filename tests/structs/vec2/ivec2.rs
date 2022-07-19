use iron_ingot::{DVec2, FVec2, IVec2, UVec2};

#[test]
fn test_from_i32() {
  assert_eq!(IVec2::new(0), IVec2::new((0, 0)));
  assert_eq!(IVec2::new(1), IVec2::new((1, 1)));
  assert_eq!(IVec2::new(2), IVec2::new((2, 2)));
}

#[test]
fn test_with_x() {
  assert_eq!(IVec2::new((0, 0)).with_x(0), IVec2::new((0, 0)));
  assert_eq!(IVec2::new((1, 0)).with_x(0), IVec2::new((0, 0)));
  assert_eq!(IVec2::new((2, 0)).with_x(0), IVec2::new((0, 0)));
  assert_eq!(IVec2::new((2, 1)).with_x(0), IVec2::new((0, 1)));
  assert_eq!(IVec2::new((2, 2)).with_x(0), IVec2::new((0, 2)));
  assert_eq!(IVec2::new((2, 2)).with_x(1), IVec2::new((1, 2)));
  assert_eq!(IVec2::new((2, 2)).with_x(2), IVec2::new((2, 2)));
}

#[test]
fn test_with_y() {
  assert_eq!(IVec2::new((0, 0)).with_y(0), IVec2::new((0, 0)));
  assert_eq!(IVec2::new((1, 0)).with_y(0), IVec2::new((1, 0)));
  assert_eq!(IVec2::new((2, 0)).with_y(0), IVec2::new((2, 0)));
  assert_eq!(IVec2::new((2, 1)).with_y(0), IVec2::new((2, 0)));
  assert_eq!(IVec2::new((2, 2)).with_y(0), IVec2::new((2, 0)));
  assert_eq!(IVec2::new((2, 2)).with_y(1), IVec2::new((2, 1)));
  assert_eq!(IVec2::new((2, 2)).with_y(2), IVec2::new((2, 2)));
}

#[test]
fn test_set() {
  let mut vec = IVec2::new(());
  vec.set(IVec2::new((0, 0)));
  assert_eq!(vec, IVec2::new((0, 0)));
  vec.set(IVec2::new((1, 0)));
  assert_eq!(vec, IVec2::new((1, 0)));
  vec.set(IVec2::new((2, 0)));
  assert_eq!(vec, IVec2::new((2, 0)));
  vec.set(IVec2::new((2, 1)));
  assert_eq!(vec, IVec2::new((2, 1)));
  vec.set(IVec2::new((2, 2)));
  assert_eq!(vec, IVec2::new((2, 2)));
}

#[test]
fn test_add() {
  assert_eq!(IVec2::new((0, 0)) + IVec2::new((0, 0)), IVec2::new((0, 0)));
  assert_eq!(IVec2::new((1, 0)) + IVec2::new((0, 0)), IVec2::new((1, 0)));
  assert_eq!(IVec2::new((2, 0)) + IVec2::new((0, 0)), IVec2::new((2, 0)));
  assert_eq!(IVec2::new((2, 1)) + IVec2::new((0, 0)), IVec2::new((2, 1)));
  assert_eq!(IVec2::new((2, 2)) + IVec2::new((0, 0)), IVec2::new((2, 2)));
  assert_eq!(IVec2::new((2, 2)) + IVec2::new((1, 0)), IVec2::new((3, 2)));
  assert_eq!(IVec2::new((2, 2)) + IVec2::new((2, 0)), IVec2::new((4, 2)));
  assert_eq!(IVec2::new((2, 2)) + IVec2::new((2, 1)), IVec2::new((4, 3)));
  assert_eq!(IVec2::new((2, 2)) + IVec2::new((2, 2)), IVec2::new((4, 4)));
}

#[test]
fn test_add_assign() {
  let mut vec = IVec2::new(());
  vec += IVec2::new((0, 0));
  assert_eq!(vec, IVec2::new((0, 0)));
  vec += IVec2::new((1, 0));
  assert_eq!(vec, IVec2::new((1, 0)));
  vec += IVec2::new((2, 0));
  assert_eq!(vec, IVec2::new((3, 0)));
  vec += IVec2::new((2, 1));
  assert_eq!(vec, IVec2::new((5, 1)));
  vec += IVec2::new((2, 2));
  assert_eq!(vec, IVec2::new((7, 3)));
}

#[test]
fn test_sub() {
  assert_eq!(IVec2::new((0, 0)) - IVec2::new((0, 0)), IVec2::new((0, 0)));
  assert_eq!(IVec2::new((1, 0)) - IVec2::new((0, 0)), IVec2::new((1, 0)));
  assert_eq!(IVec2::new((2, 0)) - IVec2::new((0, 0)), IVec2::new((2, 0)));
  assert_eq!(IVec2::new((2, 1)) - IVec2::new((0, 0)), IVec2::new((2, 1)));
  assert_eq!(IVec2::new((2, 2)) - IVec2::new((0, 0)), IVec2::new((2, 2)));
  assert_eq!(IVec2::new((2, 2)) - IVec2::new((1, 0)), IVec2::new((1, 2)));
  assert_eq!(IVec2::new((2, 2)) - IVec2::new((2, 0)), IVec2::new((0, 2)));
  assert_eq!(IVec2::new((2, 2)) - IVec2::new((2, 1)), IVec2::new((0, 1)));
  assert_eq!(IVec2::new((2, 2)) - IVec2::new((2, 2)), IVec2::new((0, 0)));
}

#[test]
fn test_sub_assign() {
  let mut vec = IVec2::new(());
  vec -= IVec2::new((0, 0));
  assert_eq!(vec, IVec2::new((0, 0)));
  vec -= IVec2::new((1, 0));
  assert_eq!(vec, IVec2::new((-1, 0)));
  vec -= IVec2::new((2, 0));
  assert_eq!(vec, IVec2::new((-3, 0)));
  vec -= IVec2::new((2, 1));
  assert_eq!(vec, IVec2::new((-5, -1)));
  vec -= IVec2::new((2, 2));
  assert_eq!(vec, IVec2::new((-7, -3)));
}

#[test]
fn test_mul() {
  assert_eq!(IVec2::new((0, 0)) * 0, IVec2::new((0, 0)));
  assert_eq!(IVec2::new((1, 0)) * 0, IVec2::new((0, 0)));
  assert_eq!(IVec2::new((2, 0)) * 0, IVec2::new((0, 0)));
  assert_eq!(IVec2::new((2, 1)) * 0, IVec2::new((0, 0)));
  assert_eq!(IVec2::new((2, 2)) * 0, IVec2::new((0, 0)));
  assert_eq!(IVec2::new((2, 2)) * 1, IVec2::new((2, 2)));
  assert_eq!(IVec2::new((2, 2)) * 2, IVec2::new((4, 4)));
}

#[test]
fn test_mul_assign() {
  let mut vec = IVec2::new((1, 1));
  vec *= 1;
  assert_eq!(vec, IVec2::new((1, 1)));
  vec *= 2;
  assert_eq!(vec, IVec2::new((2, 2)));
  vec *= 3;
  assert_eq!(vec, IVec2::new((6, 6)));
}

#[test]
fn test_div() {
  assert_eq!(IVec2::new((0, 0)) / 1, IVec2::new((0, 0)));
  assert_eq!(IVec2::new((1, 0)) / 1, IVec2::new((1, 0)));
  assert_eq!(IVec2::new((2, 0)) / 1, IVec2::new((2, 0)));
  assert_eq!(IVec2::new((2, 1)) / 1, IVec2::new((2, 1)));
  assert_eq!(IVec2::new((2, 2)) / 1, IVec2::new((2, 2)));
  assert_eq!(IVec2::new((2, 2)) / 2, IVec2::new((1, 1)));
  assert_eq!(IVec2::new((2, 2)) / 4, IVec2::new((0, 0)));
}

#[test]
fn test_div_assign() {
  let mut vec = IVec2::new((1, 1));
  vec /= 1;
  assert_eq!(vec, IVec2::new((1, 1)));
  vec /= 2;
  assert_eq!(vec, IVec2::new((0, 0)));
  vec /= 5;
  assert_eq!(vec, IVec2::new((0, 0)));
}

#[test]
fn test_bitand_i32() {
  assert_eq!(IVec2::new((0, 0)) & 0, IVec2::new((0, 0)));
  assert_eq!(IVec2::new((1, 0)) & 0, IVec2::new((0, 0)));
  assert_eq!(IVec2::new((2, 0)) & 0, IVec2::new((0, 0)));
  assert_eq!(IVec2::new((2, 1)) & 0, IVec2::new((0, 0)));
  assert_eq!(IVec2::new((2, 2)) & 0, IVec2::new((0, 0)));
  assert_eq!(IVec2::new((2, 2)) & 1, IVec2::new((0, 0)));
  assert_eq!(IVec2::new((2, 2)) & 3, IVec2::new((2, 2)));
}

#[test]
fn test_bitand_assign_i32() {
  let mut vec = IVec2::new((3, 3));
  vec &= 3;
  assert_eq!(vec, IVec2::new((3, 3)));
  vec &= 1;
  assert_eq!(vec, IVec2::new((1, 1)));
  vec &= 0;
  assert_eq!(vec, IVec2::new((0, 0)));
}

#[test]
fn test_bitand_ivec2() {
  assert_eq!(IVec2::new((0, 0)) & IVec2::new((0, 0)), IVec2::new((0, 0)));
  assert_eq!(IVec2::new((1, 0)) & IVec2::new((0, 0)), IVec2::new((0, 0)));
  assert_eq!(IVec2::new((2, 0)) & IVec2::new((0, 0)), IVec2::new((0, 0)));
  assert_eq!(IVec2::new((2, 1)) & IVec2::new((0, 0)), IVec2::new((0, 0)));
  assert_eq!(IVec2::new((2, 2)) & IVec2::new((0, 0)), IVec2::new((0, 0)));
  assert_eq!(IVec2::new((2, 2)) & IVec2::new((1, 0)), IVec2::new((0, 0)));
  assert_eq!(IVec2::new((2, 2)) & IVec2::new((3, 0)), IVec2::new((2, 0)));
  assert_eq!(IVec2::new((2, 2)) & IVec2::new((3, 1)), IVec2::new((2, 0)));
  assert_eq!(IVec2::new((2, 2)) & IVec2::new((3, 3)), IVec2::new((2, 2)));
}

#[test]
fn test_bitand_assign_ivec2() {
  let mut vec = IVec2::new((3, 3));
  vec &= IVec2::new((3, 3));
  assert_eq!(vec, IVec2::new((3, 3)));
  vec &= IVec2::new((3, 1));
  assert_eq!(vec, IVec2::new((3, 1)));
  vec &= IVec2::new((3, 0));
  assert_eq!(vec, IVec2::new((3, 0)));
  vec &= IVec2::new((1, 0));
  assert_eq!(vec, IVec2::new((1, 0)));
  vec &= IVec2::new((0, 0));
  assert_eq!(vec, IVec2::new((0, 0)));
}

#[test]
fn test_shl_i32() {
  assert_eq!(IVec2::new((0, 0)) << 0, IVec2::new((0, 0)));
  assert_eq!(IVec2::new((1, 0)) << 0, IVec2::new((1, 0)));
  assert_eq!(IVec2::new((2, 0)) << 0, IVec2::new((2, 0)));
  assert_eq!(IVec2::new((2, 1)) << 0, IVec2::new((2, 1)));
  assert_eq!(IVec2::new((2, 2)) << 0, IVec2::new((2, 2)));
  assert_eq!(IVec2::new((2, 2)) << 1, IVec2::new((4, 4)));
  assert_eq!(IVec2::new((2, 2)) << 2, IVec2::new((8, 8)));
}

#[test]
fn test_shl_assign_i32() {
  let mut vec = IVec2::new((1, 1));
  vec <<= 0;
  assert_eq!(vec, IVec2::new((1, 1)));
  vec <<= 1;
  assert_eq!(vec, IVec2::new((2, 2)));
  vec <<= 2;
  assert_eq!(vec, IVec2::new((8, 8)));
}

#[test]
fn test_shl_ivec2() {
  assert_eq!(IVec2::new((0, 0)) << IVec2::new((0, 0)), IVec2::new((0, 0)));
  assert_eq!(IVec2::new((1, 0)) << IVec2::new((0, 0)), IVec2::new((1, 0)));
  assert_eq!(IVec2::new((2, 0)) << IVec2::new((0, 0)), IVec2::new((2, 0)));
  assert_eq!(IVec2::new((2, 1)) << IVec2::new((0, 0)), IVec2::new((2, 1)));
  assert_eq!(IVec2::new((2, 2)) << IVec2::new((0, 0)), IVec2::new((2, 2)));
  assert_eq!(IVec2::new((2, 2)) << IVec2::new((1, 0)), IVec2::new((4, 2)));
  assert_eq!(IVec2::new((2, 2)) << IVec2::new((2, 0)), IVec2::new((8, 2)));
  assert_eq!(IVec2::new((2, 2)) << IVec2::new((2, 1)), IVec2::new((8, 4)));
  assert_eq!(IVec2::new((2, 2)) << IVec2::new((2, 2)), IVec2::new((8, 8)));
}

#[test]
fn test_shl_assign_ivec2() {
  let mut vec = IVec2::new((1, 1));
  vec <<= IVec2::new((0, 0));
  assert_eq!(vec, IVec2::new((1, 1)));
  vec <<= IVec2::new((1, 0));
  assert_eq!(vec, IVec2::new((2, 1)));
  vec <<= IVec2::new((2, 0));
  assert_eq!(vec, IVec2::new((8, 1)));
  vec <<= IVec2::new((2, 1));
  assert_eq!(vec, IVec2::new((32, 2)));
  vec <<= IVec2::new((2, 2));
  assert_eq!(vec, IVec2::new((128, 8)));
}

#[test]
fn test_shr_i32() {
  assert_eq!(IVec2::new((0, 0)) >> 0, IVec2::new((0, 0)));
  assert_eq!(IVec2::new((1, 0)) >> 0, IVec2::new((1, 0)));
  assert_eq!(IVec2::new((2, 0)) >> 0, IVec2::new((2, 0)));
  assert_eq!(IVec2::new((2, 1)) >> 0, IVec2::new((2, 1)));
  assert_eq!(IVec2::new((2, 2)) >> 0, IVec2::new((2, 2)));
  assert_eq!(IVec2::new((4, 4)) >> 1, IVec2::new((2, 2)));
  assert_eq!(IVec2::new((8, 8)) >> 2, IVec2::new((2, 2)));
}

#[test]
fn test_shr_assign_i32() {
  let mut vec = IVec2::new((8, 8));
  vec >>= 0;
  assert_eq!(vec, IVec2::new((8, 8)));
  vec >>= 1;
  assert_eq!(vec, IVec2::new((4, 4)));
  vec >>= 2;
  assert_eq!(vec, IVec2::new((1, 1)));
}

#[test]
fn test_shr_ivec2() {
  assert_eq!(IVec2::new((0, 0)) >> IVec2::new((0, 0)), IVec2::new((0, 0)));
  assert_eq!(IVec2::new((1, 0)) >> IVec2::new((0, 0)), IVec2::new((1, 0)));
  assert_eq!(IVec2::new((2, 0)) >> IVec2::new((0, 0)), IVec2::new((2, 0)));
  assert_eq!(IVec2::new((2, 1)) >> IVec2::new((0, 0)), IVec2::new((2, 1)));
  assert_eq!(IVec2::new((2, 2)) >> IVec2::new((0, 0)), IVec2::new((2, 2)));
  assert_eq!(IVec2::new((4, 2)) >> IVec2::new((1, 0)), IVec2::new((2, 2)));
  assert_eq!(IVec2::new((8, 2)) >> IVec2::new((2, 0)), IVec2::new((2, 2)));
  assert_eq!(IVec2::new((8, 4)) >> IVec2::new((2, 1)), IVec2::new((2, 2)));
  assert_eq!(IVec2::new((8, 8)) >> IVec2::new((2, 2)), IVec2::new((2, 2)));
}

#[test]
fn test_shr_assign_ivec2() {
  let mut vec = IVec2::new((128, 8));
  vec >>= IVec2::new((0, 0));
  assert_eq!(vec, IVec2::new((128, 8)));
  vec >>= IVec2::new((1, 0));
  assert_eq!(vec, IVec2::new((64, 8)));
  vec >>= IVec2::new((2, 0));
  assert_eq!(vec, IVec2::new((16, 8)));
  vec >>= IVec2::new((2, 1));
  assert_eq!(vec, IVec2::new((4, 4)));
  vec >>= IVec2::new((2, 2));
  assert_eq!(vec, IVec2::new((1, 1)));
}

#[test]
fn test_get_x_fliped() {
  assert_eq!(IVec2::new((0, 0)).get_x_fliped(), IVec2::new((0, 0)));
  assert_eq!(IVec2::new((1, 0)).get_x_fliped(), IVec2::new((-1, 0)));
  assert_eq!(IVec2::new((2, 0)).get_x_fliped(), IVec2::new((-2, 0)));
  assert_eq!(IVec2::new((2, 1)).get_x_fliped(), IVec2::new((-2, 1)));
  assert_eq!(IVec2::new((2, 2)).get_x_fliped(), IVec2::new((-2, 2)));
}

#[test]
fn test_get_y_fliped() {
  assert_eq!(IVec2::new((0, 0)).get_y_fliped(), IVec2::new((0, 0)));
  assert_eq!(IVec2::new((1, 0)).get_y_fliped(), IVec2::new((1, 0)));
  assert_eq!(IVec2::new((2, 0)).get_y_fliped(), IVec2::new((2, 0)));
  assert_eq!(IVec2::new((2, 1)).get_y_fliped(), IVec2::new((2, -1)));
  assert_eq!(IVec2::new((2, 2)).get_y_fliped(), IVec2::new((2, -2)));
}

#[test]
fn test_get_fliped() {
  assert_eq!(IVec2::new((0, 0)).get_fliped(), IVec2::new((0, 0)));
  assert_eq!(IVec2::new((1, 0)).get_fliped(), IVec2::new((-1, 0)));
  assert_eq!(IVec2::new((2, 0)).get_fliped(), IVec2::new((-2, 0)));
  assert_eq!(IVec2::new((2, 1)).get_fliped(), IVec2::new((-2, -1)));
  assert_eq!(IVec2::new((2, 2)).get_fliped(), IVec2::new((-2, -2)));
}

#[test]
fn test_display() {
  assert_eq!(format!("{}", IVec2::new((0, 0))), "0,0");
  assert_eq!(format!("{}", IVec2::new((1, 0))), "1,0");
  assert_eq!(format!("{}", IVec2::new((2, 0))), "2,0");
  assert_eq!(format!("{}", IVec2::new((2, 1))), "2,1");
  assert_eq!(format!("{}", IVec2::new((2, 2))), "2,2");
}

#[test]
fn test_into_dvec2() {
  assert_eq!(DVec2::from(IVec2::new((0, 0))), DVec2::new((0.0, 0.0)));
  assert_eq!(DVec2::from(IVec2::new((1, 0))), DVec2::new((1.0, 0.0)));
  assert_eq!(DVec2::from(IVec2::new((2, 0))), DVec2::new((2.0, 0.0)));
  assert_eq!(DVec2::from(IVec2::new((2, 1))), DVec2::new((2.0, 1.0)));
  assert_eq!(DVec2::from(IVec2::new((2, 2))), DVec2::new((2.0, 2.0)));
}

#[test]
fn test_into_fvec2() {
  assert_eq!(FVec2::from(IVec2::new((0, 0))), FVec2::new((0.0, 0.0)));
  assert_eq!(FVec2::from(IVec2::new((1, 0))), FVec2::new((1.0, 0.0)));
  assert_eq!(FVec2::from(IVec2::new((2, 0))), FVec2::new((2.0, 0.0)));
  assert_eq!(FVec2::from(IVec2::new((2, 1))), FVec2::new((2.0, 1.0)));
  assert_eq!(FVec2::from(IVec2::new((2, 2))), FVec2::new((2.0, 2.0)));
}

#[test]
fn test_into_uvec2() {
  assert_eq!(UVec2::from(IVec2::new((0, 0))), UVec2::new((0, 0)));
  assert_eq!(UVec2::from(IVec2::new((1, 0))), UVec2::new((1, 0)));
  assert_eq!(UVec2::from(IVec2::new((2, 0))), UVec2::new((2, 0)));
  assert_eq!(UVec2::from(IVec2::new((2, 1))), UVec2::new((2, 1)));
  assert_eq!(UVec2::from(IVec2::new((2, 2))), UVec2::new((2, 2)));
}
