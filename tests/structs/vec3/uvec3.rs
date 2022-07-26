use iron_ingot::{DVec3, FVec3, IVec3, UVec3};

#[test]
fn test_from_u32() {
  assert_eq!(UVec3::new(0), UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new(1), UVec3::new((1, 1, 1)));
  assert_eq!(UVec3::new(2), UVec3::new((2, 2, 2)));
}

#[test]
fn test_with_x() {
  assert_eq!(UVec3::new((0, 0, 0)).with_x(0), UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((1, 0, 0)).with_x(0), UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((2, 0, 0)).with_x(0), UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((2, 1, 0)).with_x(0), UVec3::new((0, 1, 0)));
  assert_eq!(UVec3::new((2, 2, 0)).with_x(0), UVec3::new((0, 2, 0)));
  assert_eq!(UVec3::new((2, 2, 0)).with_x(1), UVec3::new((1, 2, 0)));
  assert_eq!(UVec3::new((2, 2, 0)).with_x(2), UVec3::new((2, 2, 0)));
  assert_eq!(UVec3::new((2, 2, 1)).with_x(2), UVec3::new((2, 2, 1)));
  assert_eq!(UVec3::new((2, 2, 2)).with_x(2), UVec3::new((2, 2, 2)));
}

#[test]
fn test_with_y() {
  assert_eq!(UVec3::new((0, 0, 0)).with_y(0), UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((1, 0, 0)).with_y(0), UVec3::new((1, 0, 0)));
  assert_eq!(UVec3::new((2, 0, 0)).with_y(0), UVec3::new((2, 0, 0)));
  assert_eq!(UVec3::new((2, 1, 0)).with_y(0), UVec3::new((2, 0, 0)));
  assert_eq!(UVec3::new((2, 2, 0)).with_y(0), UVec3::new((2, 0, 0)));
  assert_eq!(UVec3::new((2, 2, 0)).with_y(1), UVec3::new((2, 1, 0)));
  assert_eq!(UVec3::new((2, 2, 0)).with_y(2), UVec3::new((2, 2, 0)));
  assert_eq!(UVec3::new((2, 2, 1)).with_y(2), UVec3::new((2, 2, 1)));
  assert_eq!(UVec3::new((2, 2, 2)).with_y(2), UVec3::new((2, 2, 2)));
}

#[test]
fn test_with_z() {
  assert_eq!(UVec3::new((0, 0, 0)).with_z(0), UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((1, 0, 0)).with_z(0), UVec3::new((1, 0, 0)));
  assert_eq!(UVec3::new((2, 0, 0)).with_z(0), UVec3::new((2, 0, 0)));
  assert_eq!(UVec3::new((2, 1, 0)).with_z(0), UVec3::new((2, 1, 0)));
  assert_eq!(UVec3::new((2, 2, 0)).with_z(0), UVec3::new((2, 2, 0)));
  assert_eq!(UVec3::new((2, 2, 0)).with_z(1), UVec3::new((2, 2, 1)));
  assert_eq!(UVec3::new((2, 2, 0)).with_z(2), UVec3::new((2, 2, 2)));
  assert_eq!(UVec3::new((2, 2, 1)).with_z(2), UVec3::new((2, 2, 2)));
  assert_eq!(UVec3::new((2, 2, 2)).with_z(2), UVec3::new((2, 2, 2)));
}

#[test]
fn test_add() {
  assert_eq!(
    UVec3::new((0, 0, 0)) + UVec3::new((0, 0, 0)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((1, 0, 0)) + UVec3::new((0, 0, 0)),
    UVec3::new((1, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 0, 0)) + UVec3::new((0, 0, 0)),
    UVec3::new((2, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 1, 0)) + UVec3::new((0, 0, 0)),
    UVec3::new((2, 1, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) + UVec3::new((0, 0, 0)),
    UVec3::new((2, 2, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) + UVec3::new((1, 0, 0)),
    UVec3::new((3, 2, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) + UVec3::new((2, 0, 0)),
    UVec3::new((4, 2, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) + UVec3::new((2, 1, 0)),
    UVec3::new((4, 3, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) + UVec3::new((2, 2, 0)),
    UVec3::new((4, 4, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 1)) + UVec3::new((2, 2, 0)),
    UVec3::new((4, 4, 1))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) + UVec3::new((2, 2, 0)),
    UVec3::new((4, 4, 2))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) + UVec3::new((2, 2, 1)),
    UVec3::new((4, 4, 3))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) + UVec3::new((2, 2, 2)),
    UVec3::new((4, 4, 4))
  );
}

#[test]
fn test_add_assign() {
  let mut vec = UVec3::new(());
  vec += UVec3::new((0, 0, 0));
  assert_eq!(vec, UVec3::new((0, 0, 0)));
  vec += UVec3::new((1, 0, 0));
  assert_eq!(vec, UVec3::new((1, 0, 0)));
  vec += UVec3::new((2, 0, 0));
  assert_eq!(vec, UVec3::new((3, 0, 0)));
  vec += UVec3::new((2, 1, 0));
  assert_eq!(vec, UVec3::new((5, 1, 0)));
  vec += UVec3::new((2, 2, 0));
  assert_eq!(vec, UVec3::new((7, 3, 0)));
  vec += UVec3::new((2, 2, 1));
  assert_eq!(vec, UVec3::new((9, 5, 1)));
  vec += UVec3::new((2, 2, 2));
  assert_eq!(vec, UVec3::new((11, 7, 3)));
}

#[test]
fn test_sub() {
  assert_eq!(
    UVec3::new((0, 0, 0)) - UVec3::new((0, 0, 0)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((1, 0, 0)) - UVec3::new((0, 0, 0)),
    UVec3::new((1, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 0, 0)) - UVec3::new((0, 0, 0)),
    UVec3::new((2, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 1, 0)) - UVec3::new((0, 0, 0)),
    UVec3::new((2, 1, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) - UVec3::new((0, 0, 0)),
    UVec3::new((2, 2, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) - UVec3::new((1, 0, 0)),
    UVec3::new((1, 2, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) - UVec3::new((2, 0, 0)),
    UVec3::new((0, 2, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) - UVec3::new((2, 1, 0)),
    UVec3::new((0, 1, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) - UVec3::new((2, 2, 0)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 1)) - UVec3::new((2, 2, 0)),
    UVec3::new((0, 0, 1))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) - UVec3::new((2, 2, 0)),
    UVec3::new((0, 0, 2))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) - UVec3::new((2, 2, 1)),
    UVec3::new((0, 0, 1))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) - UVec3::new((2, 2, 2)),
    UVec3::new((0, 0, 0))
  );
}

#[test]
fn test_sub_assign() {
  let mut vec = UVec3::new((20, 20, 20));
  vec -= UVec3::new((0, 0, 0));
  assert_eq!(vec, UVec3::new((20, 20, 20)));
  vec -= UVec3::new((1, 0, 0));
  assert_eq!(vec, UVec3::new((19, 20, 20)));
  vec -= UVec3::new((2, 0, 0));
  assert_eq!(vec, UVec3::new((17, 20, 20)));
  vec -= UVec3::new((2, 1, 0));
  assert_eq!(vec, UVec3::new((15, 19, 20)));
  vec -= UVec3::new((2, 2, 0));
  assert_eq!(vec, UVec3::new((13, 17, 20)));
  vec -= UVec3::new((2, 2, 1));
  assert_eq!(vec, UVec3::new((11, 15, 19)));
  vec -= UVec3::new((2, 2, 2));
  assert_eq!(vec, UVec3::new((9, 13, 17)));
}

#[test]
fn test_mul_u32() {
  assert_eq!(UVec3::new((0, 0, 0)) * 0, UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((1, 0, 0)) * 0, UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((2, 0, 0)) * 0, UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((2, 1, 0)) * 0, UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((2, 2, 0)) * 0, UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((2, 2, 0)) * 1, UVec3::new((2, 2, 0)));
  assert_eq!(UVec3::new((2, 2, 0)) * 2, UVec3::new((4, 4, 0)));
  assert_eq!(UVec3::new((2, 2, 1)) * 2, UVec3::new((4, 4, 2)));
  assert_eq!(UVec3::new((2, 2, 2)) * 2, UVec3::new((4, 4, 4)));
}

#[test]
fn test_mul_u32_assign() {
  let mut vec = UVec3::new((1, 1, 1));
  vec *= 1;
  assert_eq!(vec, UVec3::new((1, 1, 1)));
  vec *= 2;
  assert_eq!(vec, UVec3::new((2, 2, 2)));
  vec *= 3;
  assert_eq!(vec, UVec3::new((6, 6, 6)));
}

#[test]
fn test_mul() {
  assert_eq!(
    UVec3::new((0, 0, 0)) * UVec3::new((0, 0, 0)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((1, 0, 0)) * UVec3::new((0, 0, 0)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 0, 0)) * UVec3::new((0, 0, 0)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 1, 0)) * UVec3::new((0, 0, 0)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) * UVec3::new((0, 0, 0)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 1)) * UVec3::new((0, 0, 0)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) * UVec3::new((0, 0, 0)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) * UVec3::new((1, 0, 0)),
    UVec3::new((2, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) * UVec3::new((2, 0, 0)),
    UVec3::new((4, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) * UVec3::new((2, 1, 0)),
    UVec3::new((4, 2, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) * UVec3::new((2, 2, 0)),
    UVec3::new((4, 4, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) * UVec3::new((2, 2, 1)),
    UVec3::new((4, 4, 2))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) * UVec3::new((2, 2, 2)),
    UVec3::new((4, 4, 4))
  );
}

#[test]
fn test_mul_assign() {
  let mut vec = UVec3::new((1, 1, 1));
  vec *= UVec3::new((1, 1, 1));
  assert_eq!(vec, UVec3::new((1, 1, 1)));
  vec *= UVec3::new((2, 1, 1));
  assert_eq!(vec, UVec3::new((2, 1, 1)));
  vec *= UVec3::new((3, 1, 1));
  assert_eq!(vec, UVec3::new((6, 1, 1)));
  vec *= UVec3::new((3, 2, 1));
  assert_eq!(vec, UVec3::new((18, 2, 1)));
  vec *= UVec3::new((3, 3, 1));
  assert_eq!(vec, UVec3::new((54, 6, 1)));
  vec *= UVec3::new((3, 3, 2));
  assert_eq!(vec, UVec3::new((162, 18, 2)));
  vec *= UVec3::new((3, 3, 3));
  assert_eq!(vec, UVec3::new((486, 54, 6)));
}

#[test]
fn test_div_u32() {
  assert_eq!(UVec3::new((0, 0, 0)) / 1, UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((1, 0, 0)) / 1, UVec3::new((1, 0, 0)));
  assert_eq!(UVec3::new((2, 0, 0)) / 1, UVec3::new((2, 0, 0)));
  assert_eq!(UVec3::new((2, 1, 0)) / 1, UVec3::new((2, 1, 0)));
  assert_eq!(UVec3::new((2, 2, 0)) / 1, UVec3::new((2, 2, 0)));
  assert_eq!(UVec3::new((2, 2, 0)) / 2, UVec3::new((1, 1, 0)));
  assert_eq!(UVec3::new((2, 2, 0)) / 4, UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((2, 2, 1)) / 4, UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((2, 2, 2)) / 4, UVec3::new((0, 0, 0)));
}

#[test]
fn test_div_u32_assign() {
  let mut vec = UVec3::new((1, 1, 1));
  vec /= 1;
  assert_eq!(vec, UVec3::new((1, 1, 1)));
  vec /= 2;
  assert_eq!(vec, UVec3::new((0, 0, 0)));
  vec /= 5;
  assert_eq!(vec, UVec3::new((0, 0, 0)));
}

#[test]
fn test_div() {
  assert_eq!(
    UVec3::new((0, 0, 0)) / UVec3::new((1, 1, 1)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((1, 0, 0)) / UVec3::new((1, 1, 1)),
    UVec3::new((1, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 0, 0)) / UVec3::new((1, 1, 1)),
    UVec3::new((2, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 1, 0)) / UVec3::new((1, 1, 1)),
    UVec3::new((2, 1, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) / UVec3::new((1, 1, 1)),
    UVec3::new((2, 2, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 1)) / UVec3::new((1, 1, 1)),
    UVec3::new((2, 2, 1))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) / UVec3::new((1, 1, 1)),
    UVec3::new((2, 2, 2))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) / UVec3::new((2, 1, 1)),
    UVec3::new((1, 2, 2))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) / UVec3::new((4, 1, 1)),
    UVec3::new((0, 2, 2))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) / UVec3::new((4, 2, 1)),
    UVec3::new((0, 1, 2))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) / UVec3::new((4, 4, 1)),
    UVec3::new((0, 0, 2))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) / UVec3::new((4, 4, 2)),
    UVec3::new((0, 0, 1))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) / UVec3::new((4, 4, 4)),
    UVec3::new((0, 0, 0))
  );
}

#[test]
fn test_div_assign() {
  let mut vec = UVec3::new((1, 1, 1));
  vec /= UVec3::new((1, 1, 1));
  assert_eq!(vec, UVec3::new((1, 1, 1)));
  vec /= UVec3::new((2, 1, 1));
  assert_eq!(vec, UVec3::new((0, 1, 1)));
  vec /= UVec3::new((5, 1, 1));
  assert_eq!(vec, UVec3::new((0, 1, 1)));
  vec /= UVec3::new((5, 2, 1));
  assert_eq!(vec, UVec3::new((0, 0, 1)));
  vec /= UVec3::new((5, 5, 1));
  assert_eq!(vec, UVec3::new((0, 0, 1)));
  vec /= UVec3::new((5, 5, 2));
  assert_eq!(vec, UVec3::new((0, 0, 0)));
  vec /= UVec3::new((5, 5, 5));
  assert_eq!(vec, UVec3::new((0, 0, 0)));
}

#[test]
fn test_set() {
  let mut vec = UVec3::new(());
  vec.set(UVec3::new((0, 0, 0)));
  assert_eq!(vec, UVec3::new((0, 0, 0)));
  vec.set(UVec3::new((1, 0, 0)));
  assert_eq!(vec, UVec3::new((1, 0, 0)));
  vec.set(UVec3::new((2, 0, 0)));
  assert_eq!(vec, UVec3::new((2, 0, 0)));
  vec.set(UVec3::new((2, 1, 0)));
  assert_eq!(vec, UVec3::new((2, 1, 0)));
  vec.set(UVec3::new((2, 2, 0)));
  assert_eq!(vec, UVec3::new((2, 2, 0)));
  vec.set(UVec3::new((2, 2, 1)));
  assert_eq!(vec, UVec3::new((2, 2, 1)));
  vec.set(UVec3::new((2, 2, 2)));
  assert_eq!(vec, UVec3::new((2, 2, 2)));
}

#[test]
fn test_bitand_u32() {
  assert_eq!(UVec3::new((0, 0, 0)) & 0, UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((1, 0, 0)) & 0, UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((2, 0, 0)) & 0, UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((2, 1, 0)) & 0, UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((2, 2, 0)) & 0, UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((2, 2, 1)) & 0, UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((2, 2, 2)) & 0, UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((2, 2, 2)) & 1, UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((2, 2, 2)) & 3, UVec3::new((2, 2, 2)));
}

#[test]
fn test_bitand_assign_u32() {
  let mut vec = UVec3::new((3, 3, 3));
  vec &= 3;
  assert_eq!(vec, UVec3::new((3, 3, 3)));
  vec &= 1;
  assert_eq!(vec, UVec3::new((1, 1, 1)));
  vec &= 0;
  assert_eq!(vec, UVec3::new((0, 0, 0)));
}

#[test]
fn test_bitand_uvec3() {
  assert_eq!(
    UVec3::new((0, 0, 0)) & UVec3::new((0, 0, 0)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((1, 0, 0)) & UVec3::new((0, 0, 0)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 0, 0)) & UVec3::new((0, 0, 0)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 1, 0)) & UVec3::new((0, 0, 0)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) & UVec3::new((0, 0, 0)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) & UVec3::new((1, 0, 0)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) & UVec3::new((3, 0, 0)),
    UVec3::new((2, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) & UVec3::new((3, 1, 0)),
    UVec3::new((2, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) & UVec3::new((3, 3, 0)),
    UVec3::new((2, 2, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 1)) & UVec3::new((3, 3, 0)),
    UVec3::new((2, 2, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) & UVec3::new((3, 3, 0)),
    UVec3::new((2, 2, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) & UVec3::new((3, 3, 1)),
    UVec3::new((2, 2, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) & UVec3::new((3, 3, 3)),
    UVec3::new((2, 2, 2))
  );
}

#[test]
fn test_bitand_assign_uvec3() {
  let mut vec = UVec3::new((3, 3, 3));
  vec &= UVec3::new((3, 3, 3));
  assert_eq!(vec, UVec3::new((3, 3, 3)));
  vec &= UVec3::new((3, 1, 3));
  assert_eq!(vec, UVec3::new((3, 1, 3)));
  vec &= UVec3::new((3, 0, 3));
  assert_eq!(vec, UVec3::new((3, 0, 3)));
  vec &= UVec3::new((1, 0, 3));
  assert_eq!(vec, UVec3::new((1, 0, 3)));
  vec &= UVec3::new((0, 0, 3));
  assert_eq!(vec, UVec3::new((0, 0, 3)));
  vec &= UVec3::new((0, 0, 1));
  assert_eq!(vec, UVec3::new((0, 0, 1)));
  vec &= UVec3::new((0, 0, 0));
  assert_eq!(vec, UVec3::new((0, 0, 0)));
}

#[test]
fn test_shl_u32() {
  assert_eq!(UVec3::new((0, 0, 0)) << 0, UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((1, 0, 0)) << 0, UVec3::new((1, 0, 0)));
  assert_eq!(UVec3::new((2, 0, 0)) << 0, UVec3::new((2, 0, 0)));
  assert_eq!(UVec3::new((2, 1, 0)) << 0, UVec3::new((2, 1, 0)));
  assert_eq!(UVec3::new((2, 2, 0)) << 0, UVec3::new((2, 2, 0)));
  assert_eq!(UVec3::new((2, 2, 1)) << 0, UVec3::new((2, 2, 1)));
  assert_eq!(UVec3::new((2, 2, 2)) << 0, UVec3::new((2, 2, 2)));
  assert_eq!(UVec3::new((2, 2, 2)) << 1, UVec3::new((4, 4, 4)));
  assert_eq!(UVec3::new((2, 2, 2)) << 2, UVec3::new((8, 8, 8)));
}

#[test]
fn test_shl_assign_u32() {
  let mut vec = UVec3::new((1, 1, 1));
  vec <<= 0;
  assert_eq!(vec, UVec3::new((1, 1, 1)));
  vec <<= 1;
  assert_eq!(vec, UVec3::new((2, 2, 2)));
  vec <<= 2;
  assert_eq!(vec, UVec3::new((8, 8, 8)));
}

#[test]
fn test_shl_uvec3() {
  assert_eq!(
    UVec3::new((0, 0, 0)) << UVec3::new((0, 0, 0)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((1, 0, 0)) << UVec3::new((0, 0, 0)),
    UVec3::new((1, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 0, 0)) << UVec3::new((0, 0, 0)),
    UVec3::new((2, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 1, 0)) << UVec3::new((0, 0, 0)),
    UVec3::new((2, 1, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) << UVec3::new((0, 0, 0)),
    UVec3::new((2, 2, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 1)) << UVec3::new((0, 0, 0)),
    UVec3::new((2, 2, 1))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) << UVec3::new((0, 0, 0)),
    UVec3::new((2, 2, 2))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) << UVec3::new((1, 0, 0)),
    UVec3::new((4, 2, 2))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) << UVec3::new((2, 0, 0)),
    UVec3::new((8, 2, 2))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) << UVec3::new((2, 1, 0)),
    UVec3::new((8, 4, 2))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) << UVec3::new((2, 2, 0)),
    UVec3::new((8, 8, 2))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) << UVec3::new((2, 2, 1)),
    UVec3::new((8, 8, 4))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) << UVec3::new((2, 2, 2)),
    UVec3::new((8, 8, 8))
  );
}

#[test]
fn test_shl_assign_uvec3() {
  let mut vec = UVec3::new((1, 1, 1));
  vec <<= UVec3::new((0, 0, 0));
  assert_eq!(vec, UVec3::new((1, 1, 1)));
  vec <<= UVec3::new((1, 0, 0));
  assert_eq!(vec, UVec3::new((2, 1, 1)));
  vec <<= UVec3::new((2, 0, 0));
  assert_eq!(vec, UVec3::new((8, 1, 1)));
  vec <<= UVec3::new((2, 1, 0));
  assert_eq!(vec, UVec3::new((32, 2, 1)));
  vec <<= UVec3::new((2, 2, 0));
  assert_eq!(vec, UVec3::new((128, 8, 1)));
  vec <<= UVec3::new((2, 2, 1));
  assert_eq!(vec, UVec3::new((512, 32, 2)));
  vec <<= UVec3::new((2, 2, 2));
  assert_eq!(vec, UVec3::new((2048, 128, 8)));
}

#[test]
fn test_shr_u32() {
  assert_eq!(UVec3::new((0, 0, 0)) >> 0, UVec3::new((0, 0, 0)));
  assert_eq!(UVec3::new((1, 0, 0)) >> 0, UVec3::new((1, 0, 0)));
  assert_eq!(UVec3::new((2, 0, 0)) >> 0, UVec3::new((2, 0, 0)));
  assert_eq!(UVec3::new((2, 1, 0)) >> 0, UVec3::new((2, 1, 0)));
  assert_eq!(UVec3::new((2, 2, 0)) >> 0, UVec3::new((2, 2, 0)));
  assert_eq!(UVec3::new((2, 2, 1)) >> 0, UVec3::new((2, 2, 1)));
  assert_eq!(UVec3::new((2, 2, 2)) >> 0, UVec3::new((2, 2, 2)));
  assert_eq!(UVec3::new((4, 4, 4)) >> 1, UVec3::new((2, 2, 2)));
  assert_eq!(UVec3::new((8, 8, 8)) >> 2, UVec3::new((2, 2, 2)));
}

#[test]
fn test_shr_assign_u32() {
  let mut vec = UVec3::new((8, 8, 8));
  vec >>= 0;
  assert_eq!(vec, UVec3::new((8, 8, 8)));
  vec >>= 1;
  assert_eq!(vec, UVec3::new((4, 4, 4)));
  vec >>= 2;
  assert_eq!(vec, UVec3::new((1, 1, 1)));
}

#[test]
fn test_shr_uvec3() {
  assert_eq!(
    UVec3::new((0, 0, 0)) >> UVec3::new((0, 0, 0)),
    UVec3::new((0, 0, 0))
  );
  assert_eq!(
    UVec3::new((1, 0, 0)) >> UVec3::new((0, 0, 0)),
    UVec3::new((1, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 0, 0)) >> UVec3::new((0, 0, 0)),
    UVec3::new((2, 0, 0))
  );
  assert_eq!(
    UVec3::new((2, 1, 0)) >> UVec3::new((0, 0, 0)),
    UVec3::new((2, 1, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 0)) >> UVec3::new((0, 0, 0)),
    UVec3::new((2, 2, 0))
  );
  assert_eq!(
    UVec3::new((2, 2, 1)) >> UVec3::new((0, 0, 0)),
    UVec3::new((2, 2, 1))
  );
  assert_eq!(
    UVec3::new((2, 2, 2)) >> UVec3::new((0, 0, 0)),
    UVec3::new((2, 2, 2))
  );
  assert_eq!(
    UVec3::new((4, 2, 2)) >> UVec3::new((1, 0, 0)),
    UVec3::new((2, 2, 2))
  );
  assert_eq!(
    UVec3::new((8, 2, 2)) >> UVec3::new((2, 0, 0)),
    UVec3::new((2, 2, 2))
  );
  assert_eq!(
    UVec3::new((8, 4, 2)) >> UVec3::new((2, 1, 0)),
    UVec3::new((2, 2, 2))
  );
  assert_eq!(
    UVec3::new((8, 8, 2)) >> UVec3::new((2, 2, 0)),
    UVec3::new((2, 2, 2))
  );
  assert_eq!(
    UVec3::new((8, 8, 4)) >> UVec3::new((2, 2, 1)),
    UVec3::new((2, 2, 2))
  );
  assert_eq!(
    UVec3::new((8, 8, 8)) >> UVec3::new((2, 2, 2)),
    UVec3::new((2, 2, 2))
  );
}

#[test]
fn test_shr_assign_uvec3() {
  let mut vec = UVec3::new((2048, 128, 8));
  vec >>= UVec3::new((0, 0, 0));
  assert_eq!(vec, UVec3::new((2048, 128, 8)));
  vec >>= UVec3::new((1, 0, 0));
  assert_eq!(vec, UVec3::new((1024, 128, 8)));
  vec >>= UVec3::new((2, 0, 0));
  assert_eq!(vec, UVec3::new((256, 128, 8)));
  vec >>= UVec3::new((2, 1, 0));
  assert_eq!(vec, UVec3::new((64, 64, 8)));
  vec >>= UVec3::new((2, 2, 0));
  assert_eq!(vec, UVec3::new((16, 16, 8)));
  vec >>= UVec3::new((2, 2, 1));
  assert_eq!(vec, UVec3::new((4, 4, 4)));
  vec >>= UVec3::new((2, 2, 2));
  assert_eq!(vec, UVec3::new((1, 1, 1)));
}

#[test]
fn test_display() {
  assert_eq!(format!("{}", UVec3::new((0, 0, 0))), "0,0,0");
  assert_eq!(format!("{}", UVec3::new((1, 0, 0))), "1,0,0");
  assert_eq!(format!("{}", UVec3::new((2, 0, 0))), "2,0,0");
  assert_eq!(format!("{}", UVec3::new((2, 1, 0))), "2,1,0");
  assert_eq!(format!("{}", UVec3::new((2, 2, 0))), "2,2,0");
  assert_eq!(format!("{}", UVec3::new((2, 2, 1))), "2,2,1");
  assert_eq!(format!("{}", UVec3::new((2, 2, 2))), "2,2,2");
}

#[test]
fn test_into_dvec3() {
  assert_eq!(
    DVec3::from(UVec3::new((0, 0, 0))),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::from(UVec3::new((1, 0, 0))),
    DVec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::from(UVec3::new((2, 0, 0))),
    DVec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec3::from(UVec3::new((2, 1, 0))),
    DVec3::new((2.0, 1.0, 0.0))
  );
  assert_eq!(
    DVec3::from(UVec3::new((2, 2, 0))),
    DVec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    DVec3::from(UVec3::new((2, 2, 1))),
    DVec3::new((2.0, 2.0, 1.0))
  );
  assert_eq!(
    DVec3::from(UVec3::new((2, 2, 2))),
    DVec3::new((2.0, 2.0, 2.0))
  );
}

#[test]
fn test_into_fvec3() {
  assert_eq!(
    FVec3::from(UVec3::new((0, 0, 0))),
    FVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    FVec3::from(UVec3::new((1, 0, 0))),
    FVec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    FVec3::from(UVec3::new((2, 0, 0))),
    FVec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    FVec3::from(UVec3::new((2, 1, 0))),
    FVec3::new((2.0, 1.0, 0.0))
  );
  assert_eq!(
    FVec3::from(UVec3::new((2, 2, 0))),
    FVec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    FVec3::from(UVec3::new((2, 2, 1))),
    FVec3::new((2.0, 2.0, 1.0))
  );
  assert_eq!(
    FVec3::from(UVec3::new((2, 2, 2))),
    FVec3::new((2.0, 2.0, 2.0))
  );
}

#[test]
fn test_into_ivec3() {
  assert_eq!(IVec3::from(UVec3::new((0, 0, 0))), IVec3::new((0, 0, 0)));
  assert_eq!(IVec3::from(UVec3::new((1, 0, 0))), IVec3::new((1, 0, 0)));
  assert_eq!(IVec3::from(UVec3::new((2, 0, 0))), IVec3::new((2, 0, 0)));
  assert_eq!(IVec3::from(UVec3::new((2, 1, 0))), IVec3::new((2, 1, 0)));
  assert_eq!(IVec3::from(UVec3::new((2, 2, 0))), IVec3::new((2, 2, 0)));
  assert_eq!(IVec3::from(UVec3::new((2, 2, 1))), IVec3::new((2, 2, 1)));
  assert_eq!(IVec3::from(UVec3::new((2, 2, 2))), IVec3::new((2, 2, 2)));
}
