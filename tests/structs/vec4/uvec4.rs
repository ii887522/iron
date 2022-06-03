use iron_ingot::{DVec4, FVec4, IVec4, UVec4};

#[test]
fn test_from_u32() {
  assert_eq!(UVec4::new(0), UVec4::new((0, 0, 0, 0)));
  assert_eq!(UVec4::new(1), UVec4::new((1, 1, 1, 1)));
  assert_eq!(UVec4::new(2), UVec4::new((2, 2, 2, 2)));
}

#[test]
fn test_with_x() {
  assert_eq!(UVec4::new((0, 0, 0, 0)).with_x(0), UVec4::new((0, 0, 0, 0)));
  assert_eq!(UVec4::new((1, 0, 0, 0)).with_x(0), UVec4::new((0, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 0, 0, 0)).with_x(0), UVec4::new((0, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 1, 0, 0)).with_x(0), UVec4::new((0, 1, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 0, 0)).with_x(0), UVec4::new((0, 2, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 0, 0)).with_x(1), UVec4::new((1, 2, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 0, 0)).with_x(2), UVec4::new((2, 2, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 1, 0)).with_x(2), UVec4::new((2, 2, 1, 0)));
  assert_eq!(UVec4::new((2, 2, 2, 0)).with_x(2), UVec4::new((2, 2, 2, 0)));
  assert_eq!(UVec4::new((2, 2, 2, 1)).with_x(2), UVec4::new((2, 2, 2, 1)));
  assert_eq!(UVec4::new((2, 2, 2, 2)).with_x(2), UVec4::new((2, 2, 2, 2)));
}

#[test]
fn test_with_y() {
  assert_eq!(UVec4::new((0, 0, 0, 0)).with_y(0), UVec4::new((0, 0, 0, 0)));
  assert_eq!(UVec4::new((1, 0, 0, 0)).with_y(0), UVec4::new((1, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 0, 0, 0)).with_y(0), UVec4::new((2, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 1, 0, 0)).with_y(0), UVec4::new((2, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 0, 0)).with_y(0), UVec4::new((2, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 0, 0)).with_y(1), UVec4::new((2, 1, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 0, 0)).with_y(2), UVec4::new((2, 2, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 1, 0)).with_y(2), UVec4::new((2, 2, 1, 0)));
  assert_eq!(UVec4::new((2, 2, 2, 0)).with_y(2), UVec4::new((2, 2, 2, 0)));
  assert_eq!(UVec4::new((2, 2, 2, 1)).with_y(2), UVec4::new((2, 2, 2, 1)));
  assert_eq!(UVec4::new((2, 2, 2, 2)).with_y(2), UVec4::new((2, 2, 2, 2)));
}

#[test]
fn test_with_z() {
  assert_eq!(UVec4::new((0, 0, 0, 0)).with_z(0), UVec4::new((0, 0, 0, 0)));
  assert_eq!(UVec4::new((1, 0, 0, 0)).with_z(0), UVec4::new((1, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 0, 0, 0)).with_z(0), UVec4::new((2, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 1, 0, 0)).with_z(0), UVec4::new((2, 1, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 0, 0)).with_z(0), UVec4::new((2, 2, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 0, 0)).with_z(1), UVec4::new((2, 2, 1, 0)));
  assert_eq!(UVec4::new((2, 2, 0, 0)).with_z(2), UVec4::new((2, 2, 2, 0)));
  assert_eq!(UVec4::new((2, 2, 1, 0)).with_z(2), UVec4::new((2, 2, 2, 0)));
  assert_eq!(UVec4::new((2, 2, 2, 0)).with_z(2), UVec4::new((2, 2, 2, 0)));
  assert_eq!(UVec4::new((2, 2, 2, 1)).with_z(2), UVec4::new((2, 2, 2, 1)));
  assert_eq!(UVec4::new((2, 2, 2, 2)).with_z(2), UVec4::new((2, 2, 2, 2)));
}

#[test]
fn test_with_w() {
  assert_eq!(UVec4::new((0, 0, 0, 0)).with_w(0), UVec4::new((0, 0, 0, 0)));
  assert_eq!(UVec4::new((1, 0, 0, 0)).with_w(0), UVec4::new((1, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 0, 0, 0)).with_w(0), UVec4::new((2, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 1, 0, 0)).with_w(0), UVec4::new((2, 1, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 0, 0)).with_w(0), UVec4::new((2, 2, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 0, 0)).with_w(1), UVec4::new((2, 2, 0, 1)));
  assert_eq!(UVec4::new((2, 2, 0, 0)).with_w(2), UVec4::new((2, 2, 0, 2)));
  assert_eq!(UVec4::new((2, 2, 1, 0)).with_w(2), UVec4::new((2, 2, 1, 2)));
  assert_eq!(UVec4::new((2, 2, 2, 0)).with_w(2), UVec4::new((2, 2, 2, 2)));
  assert_eq!(UVec4::new((2, 2, 2, 1)).with_w(2), UVec4::new((2, 2, 2, 2)));
  assert_eq!(UVec4::new((2, 2, 2, 2)).with_w(2), UVec4::new((2, 2, 2, 2)));
}

#[test]
fn test_add() {
  assert_eq!(
    UVec4::new((0, 0, 0, 0)) + UVec4::new((0, 0, 0, 0)),
    UVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    UVec4::new((1, 0, 0, 0)) + UVec4::new((0, 0, 0, 0)),
    UVec4::new((1, 0, 0, 0))
  );
  assert_eq!(
    UVec4::new((2, 0, 0, 0)) + UVec4::new((0, 0, 0, 0)),
    UVec4::new((2, 0, 0, 0))
  );
  assert_eq!(
    UVec4::new((2, 1, 0, 0)) + UVec4::new((0, 0, 0, 0)),
    UVec4::new((2, 1, 0, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 0, 0)) + UVec4::new((0, 0, 0, 0)),
    UVec4::new((2, 2, 0, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 0, 0)) + UVec4::new((1, 0, 0, 0)),
    UVec4::new((3, 2, 0, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 0, 0)) + UVec4::new((2, 0, 0, 0)),
    UVec4::new((4, 2, 0, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 0, 0)) + UVec4::new((2, 1, 0, 0)),
    UVec4::new((4, 3, 0, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 0, 0)) + UVec4::new((2, 2, 0, 0)),
    UVec4::new((4, 4, 0, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 1, 0)) + UVec4::new((2, 2, 0, 0)),
    UVec4::new((4, 4, 1, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 2, 0)) + UVec4::new((2, 2, 0, 0)),
    UVec4::new((4, 4, 2, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 2, 0)) + UVec4::new((2, 2, 1, 0)),
    UVec4::new((4, 4, 3, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 2, 0)) + UVec4::new((2, 2, 2, 0)),
    UVec4::new((4, 4, 4, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 2, 1)) + UVec4::new((2, 2, 2, 0)),
    UVec4::new((4, 4, 4, 1))
  );
  assert_eq!(
    UVec4::new((2, 2, 2, 2)) + UVec4::new((2, 2, 2, 0)),
    UVec4::new((4, 4, 4, 2))
  );
  assert_eq!(
    UVec4::new((2, 2, 2, 2)) + UVec4::new((2, 2, 2, 1)),
    UVec4::new((4, 4, 4, 3))
  );
  assert_eq!(
    UVec4::new((2, 2, 2, 2)) + UVec4::new((2, 2, 2, 2)),
    UVec4::new((4, 4, 4, 4))
  );
}

#[test]
fn test_sub() {
  assert_eq!(
    UVec4::new((0, 0, 0, 0)) - UVec4::new((0, 0, 0, 0)),
    UVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    UVec4::new((1, 0, 0, 0)) - UVec4::new((0, 0, 0, 0)),
    UVec4::new((1, 0, 0, 0))
  );
  assert_eq!(
    UVec4::new((2, 0, 0, 0)) - UVec4::new((0, 0, 0, 0)),
    UVec4::new((2, 0, 0, 0))
  );
  assert_eq!(
    UVec4::new((2, 1, 0, 0)) - UVec4::new((0, 0, 0, 0)),
    UVec4::new((2, 1, 0, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 0, 0)) - UVec4::new((0, 0, 0, 0)),
    UVec4::new((2, 2, 0, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 0, 0)) - UVec4::new((1, 0, 0, 0)),
    UVec4::new((1, 2, 0, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 0, 0)) - UVec4::new((2, 0, 0, 0)),
    UVec4::new((0, 2, 0, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 0, 0)) - UVec4::new((2, 1, 0, 0)),
    UVec4::new((0, 1, 0, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 0, 0)) - UVec4::new((2, 2, 0, 0)),
    UVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 1, 0)) - UVec4::new((2, 2, 0, 0)),
    UVec4::new((0, 0, 1, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 2, 0)) - UVec4::new((2, 2, 0, 0)),
    UVec4::new((0, 0, 2, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 2, 0)) - UVec4::new((2, 2, 1, 0)),
    UVec4::new((0, 0, 1, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 2, 0)) - UVec4::new((2, 2, 2, 0)),
    UVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    UVec4::new((2, 2, 2, 1)) - UVec4::new((2, 2, 2, 0)),
    UVec4::new((0, 0, 0, 1))
  );
  assert_eq!(
    UVec4::new((2, 2, 2, 2)) - UVec4::new((2, 2, 2, 0)),
    UVec4::new((0, 0, 0, 2))
  );
  assert_eq!(
    UVec4::new((2, 2, 2, 2)) - UVec4::new((2, 2, 2, 1)),
    UVec4::new((0, 0, 0, 1))
  );
  assert_eq!(
    UVec4::new((2, 2, 2, 2)) - UVec4::new((2, 2, 2, 2)),
    UVec4::new((0, 0, 0, 0))
  );
}

#[test]
fn test_mul() {
  assert_eq!(UVec4::new((0, 0, 0, 0)) * 0, UVec4::new((0, 0, 0, 0)));
  assert_eq!(UVec4::new((1, 0, 0, 0)) * 0, UVec4::new((0, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 0, 0, 0)) * 0, UVec4::new((0, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 1, 0, 0)) * 0, UVec4::new((0, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 0, 0)) * 0, UVec4::new((0, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 0, 0)) * 1, UVec4::new((2, 2, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 0, 0)) * 2, UVec4::new((4, 4, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 1, 0)) * 2, UVec4::new((4, 4, 2, 0)));
  assert_eq!(UVec4::new((2, 2, 2, 0)) * 2, UVec4::new((4, 4, 4, 0)));
  assert_eq!(UVec4::new((2, 2, 2, 1)) * 2, UVec4::new((4, 4, 4, 2)));
  assert_eq!(UVec4::new((2, 2, 2, 2)) * 2, UVec4::new((4, 4, 4, 4)));
}

#[test]
fn test_div() {
  assert_eq!(UVec4::new((0, 0, 0, 0)) / 1, UVec4::new((0, 0, 0, 0)));
  assert_eq!(UVec4::new((1, 0, 0, 0)) / 1, UVec4::new((1, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 0, 0, 0)) / 1, UVec4::new((2, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 1, 0, 0)) / 1, UVec4::new((2, 1, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 0, 0)) / 1, UVec4::new((2, 2, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 0, 0)) / 2, UVec4::new((1, 1, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 0, 0)) / 4, UVec4::new((0, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 1, 0)) / 4, UVec4::new((0, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 2, 0)) / 4, UVec4::new((0, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 2, 1)) / 4, UVec4::new((0, 0, 0, 0)));
  assert_eq!(UVec4::new((2, 2, 2, 2)) / 4, UVec4::new((0, 0, 0, 0)));
}

#[test]
fn test_set() {
  let mut vec = UVec4::new(());
  vec.set(UVec4::new((0, 0, 0, 0)));
  assert_eq!(vec, UVec4::new((0, 0, 0, 0)));
  vec.set(UVec4::new((1, 0, 0, 0)));
  assert_eq!(vec, UVec4::new((1, 0, 0, 0)));
  vec.set(UVec4::new((2, 0, 0, 0)));
  assert_eq!(vec, UVec4::new((2, 0, 0, 0)));
  vec.set(UVec4::new((2, 1, 0, 0)));
  assert_eq!(vec, UVec4::new((2, 1, 0, 0)));
  vec.set(UVec4::new((2, 2, 0, 0)));
  assert_eq!(vec, UVec4::new((2, 2, 0, 0)));
  vec.set(UVec4::new((2, 2, 1, 0)));
  assert_eq!(vec, UVec4::new((2, 2, 1, 0)));
  vec.set(UVec4::new((2, 2, 2, 0)));
  assert_eq!(vec, UVec4::new((2, 2, 2, 0)));
  vec.set(UVec4::new((2, 2, 2, 1)));
  assert_eq!(vec, UVec4::new((2, 2, 2, 1)));
  vec.set(UVec4::new((2, 2, 2, 2)));
  assert_eq!(vec, UVec4::new((2, 2, 2, 2)));
}

#[test]
fn test_add_assign() {
  let mut vec = UVec4::new(());
  vec += UVec4::new((0, 0, 0, 0));
  assert_eq!(vec, UVec4::new((0, 0, 0, 0)));
  vec += UVec4::new((1, 0, 0, 0));
  assert_eq!(vec, UVec4::new((1, 0, 0, 0)));
  vec += UVec4::new((2, 0, 0, 0));
  assert_eq!(vec, UVec4::new((3, 0, 0, 0)));
  vec += UVec4::new((2, 1, 0, 0));
  assert_eq!(vec, UVec4::new((5, 1, 0, 0)));
  vec += UVec4::new((2, 2, 0, 0));
  assert_eq!(vec, UVec4::new((7, 3, 0, 0)));
  vec += UVec4::new((2, 2, 1, 0));
  assert_eq!(vec, UVec4::new((9, 5, 1, 0)));
  vec += UVec4::new((2, 2, 2, 0));
  assert_eq!(vec, UVec4::new((11, 7, 3, 0)));
  vec += UVec4::new((2, 2, 2, 1));
  assert_eq!(vec, UVec4::new((13, 9, 5, 1)));
  vec += UVec4::new((2, 2, 2, 2));
  assert_eq!(vec, UVec4::new((15, 11, 7, 3)));
}

#[test]
fn test_sub_assign() {
  let mut vec = UVec4::new((20, 20, 20, 20));
  vec -= UVec4::new((0, 0, 0, 0));
  assert_eq!(vec, UVec4::new((20, 20, 20, 20)));
  vec -= UVec4::new((1, 0, 0, 0));
  assert_eq!(vec, UVec4::new((19, 20, 20, 20)));
  vec -= UVec4::new((2, 0, 0, 0));
  assert_eq!(vec, UVec4::new((17, 20, 20, 20)));
  vec -= UVec4::new((2, 1, 0, 0));
  assert_eq!(vec, UVec4::new((15, 19, 20, 20)));
  vec -= UVec4::new((2, 2, 0, 0));
  assert_eq!(vec, UVec4::new((13, 17, 20, 20)));
  vec -= UVec4::new((2, 2, 1, 0));
  assert_eq!(vec, UVec4::new((11, 15, 19, 20)));
  vec -= UVec4::new((2, 2, 2, 0));
  assert_eq!(vec, UVec4::new((9, 13, 17, 20)));
  vec -= UVec4::new((2, 2, 2, 1));
  assert_eq!(vec, UVec4::new((7, 11, 15, 19)));
  vec -= UVec4::new((2, 2, 2, 2));
  assert_eq!(vec, UVec4::new((5, 9, 13, 17)));
}

#[test]
fn test_mul_assign() {
  let mut vec = UVec4::new((1, 1, 1, 1));
  vec *= 1;
  assert_eq!(vec, UVec4::new((1, 1, 1, 1)));
  vec *= 2;
  assert_eq!(vec, UVec4::new((2, 2, 2, 2)));
  vec *= 3;
  assert_eq!(vec, UVec4::new((6, 6, 6, 6)));
}

#[test]
fn test_div_assign() {
  let mut vec = UVec4::new((1, 1, 1, 1));
  vec /= 1;
  assert_eq!(vec, UVec4::new((1, 1, 1, 1)));
  vec /= 2;
  assert_eq!(vec, UVec4::new((0, 0, 0, 0)));
  vec /= 5;
  assert_eq!(vec, UVec4::new((0, 0, 0, 0)));
}

#[test]
fn test_display() {
  assert_eq!(format!("{}", UVec4::new((0, 0, 0, 0))), "0,0,0,0");
  assert_eq!(format!("{}", UVec4::new((1, 0, 0, 0))), "1,0,0,0");
  assert_eq!(format!("{}", UVec4::new((2, 0, 0, 0))), "2,0,0,0");
  assert_eq!(format!("{}", UVec4::new((2, 1, 0, 0))), "2,1,0,0");
  assert_eq!(format!("{}", UVec4::new((2, 2, 0, 0))), "2,2,0,0");
  assert_eq!(format!("{}", UVec4::new((2, 2, 1, 0))), "2,2,1,0");
  assert_eq!(format!("{}", UVec4::new((2, 2, 2, 0))), "2,2,2,0");
  assert_eq!(format!("{}", UVec4::new((2, 2, 2, 1))), "2,2,2,1");
  assert_eq!(format!("{}", UVec4::new((2, 2, 2, 2))), "2,2,2,2");
}

#[test]
fn test_into_dvec4() {
  assert_eq!(
    DVec4::from(UVec4::new((0, 0, 0, 0))),
    DVec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec4::from(UVec4::new((1, 0, 0, 0))),
    DVec4::new((1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec4::from(UVec4::new((2, 0, 0, 0))),
    DVec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec4::from(UVec4::new((2, 1, 0, 0))),
    DVec4::new((2.0, 1.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec4::from(UVec4::new((2, 2, 0, 0))),
    DVec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    DVec4::from(UVec4::new((2, 2, 1, 0))),
    DVec4::new((2.0, 2.0, 1.0, 0.0))
  );
  assert_eq!(
    DVec4::from(UVec4::new((2, 2, 2, 0))),
    DVec4::new((2.0, 2.0, 2.0, 0.0))
  );
}

#[test]
fn test_into_fvec4() {
  assert_eq!(
    FVec4::from(UVec4::new((0, 0, 0, 0))),
    FVec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    FVec4::from(UVec4::new((1, 0, 0, 0))),
    FVec4::new((1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    FVec4::from(UVec4::new((2, 0, 0, 0))),
    FVec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    FVec4::from(UVec4::new((2, 1, 0, 0))),
    FVec4::new((2.0, 1.0, 0.0, 0.0))
  );
  assert_eq!(
    FVec4::from(UVec4::new((2, 2, 0, 0))),
    FVec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    FVec4::from(UVec4::new((2, 2, 1, 0))),
    FVec4::new((2.0, 2.0, 1.0, 0.0))
  );
  assert_eq!(
    FVec4::from(UVec4::new((2, 2, 2, 0))),
    FVec4::new((2.0, 2.0, 2.0, 0.0))
  );
}

#[test]
fn test_into_ivec4() {
  assert_eq!(
    IVec4::from(UVec4::new((0, 0, 0, 0))),
    IVec4::new((0, 0, 0, 0))
  );
  assert_eq!(
    IVec4::from(UVec4::new((1, 0, 0, 0))),
    IVec4::new((1, 0, 0, 0))
  );
  assert_eq!(
    IVec4::from(UVec4::new((2, 0, 0, 0))),
    IVec4::new((2, 0, 0, 0))
  );
  assert_eq!(
    IVec4::from(UVec4::new((2, 1, 0, 0))),
    IVec4::new((2, 1, 0, 0))
  );
  assert_eq!(
    IVec4::from(UVec4::new((2, 2, 0, 0))),
    IVec4::new((2, 2, 0, 0))
  );
  assert_eq!(
    IVec4::from(UVec4::new((2, 2, 1, 0))),
    IVec4::new((2, 2, 1, 0))
  );
  assert_eq!(
    IVec4::from(UVec4::new((2, 2, 2, 0))),
    IVec4::new((2, 2, 2, 0))
  );
}
