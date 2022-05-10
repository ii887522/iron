use iron::{math::ApproxEq, Vec4};

#[test]
fn test_from_f64() {
  assert_eq!(Vec4::new(0.0), Vec4::new((0.0, 0.0, 0.0, 0.0)));
  assert_eq!(Vec4::new(1.0), Vec4::new((1.0, 1.0, 1.0, 1.0)));
  assert_eq!(Vec4::new(2.0), Vec4::new((2.0, 2.0, 2.0, 2.0)));
}

#[test]
fn test_with_x() {
  assert_eq!(
    Vec4::new((0.0, 0.0, 0.0, 0.0)).with_x(0.0),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((1.0, 0.0, 0.0, 0.0)).with_x(0.0),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 0.0, 0.0, 0.0)).with_x(0.0),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 1.0, 0.0, 0.0)).with_x(0.0),
    Vec4::new((0.0, 1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)).with_x(0.0),
    Vec4::new((0.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)).with_x(1.0),
    Vec4::new((1.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)).with_x(2.0),
    Vec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 1.0, 0.0)).with_x(2.0),
    Vec4::new((2.0, 2.0, 1.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 0.0)).with_x(2.0),
    Vec4::new((2.0, 2.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 1.0)).with_x(2.0),
    Vec4::new((2.0, 2.0, 2.0, 1.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 2.0)).with_x(2.0),
    Vec4::new((2.0, 2.0, 2.0, 2.0))
  );
}

#[test]
fn test_with_y() {
  assert_eq!(
    Vec4::new((0.0, 0.0, 0.0, 0.0)).with_y(0.0),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((1.0, 0.0, 0.0, 0.0)).with_y(0.0),
    Vec4::new((1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 0.0, 0.0, 0.0)).with_y(0.0),
    Vec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 1.0, 0.0, 0.0)).with_y(0.0),
    Vec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)).with_y(0.0),
    Vec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)).with_y(1.0),
    Vec4::new((2.0, 1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)).with_y(2.0),
    Vec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 1.0, 0.0)).with_y(2.0),
    Vec4::new((2.0, 2.0, 1.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 0.0)).with_y(2.0),
    Vec4::new((2.0, 2.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 1.0)).with_y(2.0),
    Vec4::new((2.0, 2.0, 2.0, 1.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 2.0)).with_y(2.0),
    Vec4::new((2.0, 2.0, 2.0, 2.0))
  );
}

#[test]
fn test_with_z() {
  assert_eq!(
    Vec4::new((0.0, 0.0, 0.0, 0.0)).with_z(0.0),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((1.0, 0.0, 0.0, 0.0)).with_z(0.0),
    Vec4::new((1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 0.0, 0.0, 0.0)).with_z(0.0),
    Vec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 1.0, 0.0, 0.0)).with_z(0.0),
    Vec4::new((2.0, 1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)).with_z(0.0),
    Vec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)).with_z(1.0),
    Vec4::new((2.0, 2.0, 1.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)).with_z(2.0),
    Vec4::new((2.0, 2.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 1.0, 0.0)).with_z(2.0),
    Vec4::new((2.0, 2.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 0.0)).with_z(2.0),
    Vec4::new((2.0, 2.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 1.0)).with_z(2.0),
    Vec4::new((2.0, 2.0, 2.0, 1.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 2.0)).with_z(2.0),
    Vec4::new((2.0, 2.0, 2.0, 2.0))
  );
}

#[test]
fn test_with_w() {
  assert_eq!(
    Vec4::new((0.0, 0.0, 0.0, 0.0)).with_w(0.0),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((1.0, 0.0, 0.0, 0.0)).with_w(0.0),
    Vec4::new((1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 0.0, 0.0, 0.0)).with_w(0.0),
    Vec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 1.0, 0.0, 0.0)).with_w(0.0),
    Vec4::new((2.0, 1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)).with_w(0.0),
    Vec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)).with_w(1.0),
    Vec4::new((2.0, 2.0, 0.0, 1.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)).with_w(2.0),
    Vec4::new((2.0, 2.0, 0.0, 2.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 1.0, 0.0)).with_w(2.0),
    Vec4::new((2.0, 2.0, 1.0, 2.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 0.0)).with_w(2.0),
    Vec4::new((2.0, 2.0, 2.0, 2.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 1.0)).with_w(2.0),
    Vec4::new((2.0, 2.0, 2.0, 2.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 2.0)).with_w(2.0),
    Vec4::new((2.0, 2.0, 2.0, 2.0))
  );
}

#[test]
fn test_add() {
  assert_eq!(
    Vec4::new((0.0, 0.0, 0.0, 0.0)) + Vec4::new((0.0, 0.0, 0.0, 0.0)),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((1.0, 0.0, 0.0, 0.0)) + Vec4::new((0.0, 0.0, 0.0, 0.0)),
    Vec4::new((1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 0.0, 0.0, 0.0)) + Vec4::new((0.0, 0.0, 0.0, 0.0)),
    Vec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 1.0, 0.0, 0.0)) + Vec4::new((0.0, 0.0, 0.0, 0.0)),
    Vec4::new((2.0, 1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)) + Vec4::new((0.0, 0.0, 0.0, 0.0)),
    Vec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)) + Vec4::new((1.0, 0.0, 0.0, 0.0)),
    Vec4::new((3.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)) + Vec4::new((2.0, 0.0, 0.0, 0.0)),
    Vec4::new((4.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)) + Vec4::new((2.0, 1.0, 0.0, 0.0)),
    Vec4::new((4.0, 3.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)) + Vec4::new((2.0, 2.0, 0.0, 0.0)),
    Vec4::new((4.0, 4.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 1.0, 0.0)) + Vec4::new((2.0, 2.0, 0.0, 0.0)),
    Vec4::new((4.0, 4.0, 1.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 0.0)) + Vec4::new((2.0, 2.0, 0.0, 0.0)),
    Vec4::new((4.0, 4.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 0.0)) + Vec4::new((2.0, 2.0, 1.0, 0.0)),
    Vec4::new((4.0, 4.0, 3.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 0.0)) + Vec4::new((2.0, 2.0, 2.0, 0.0)),
    Vec4::new((4.0, 4.0, 4.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 1.0)) + Vec4::new((2.0, 2.0, 2.0, 0.0)),
    Vec4::new((4.0, 4.0, 4.0, 1.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 2.0)) + Vec4::new((2.0, 2.0, 2.0, 0.0)),
    Vec4::new((4.0, 4.0, 4.0, 2.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 2.0)) + Vec4::new((2.0, 2.0, 2.0, 1.0)),
    Vec4::new((4.0, 4.0, 4.0, 3.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 2.0)) + Vec4::new((2.0, 2.0, 2.0, 2.0)),
    Vec4::new((4.0, 4.0, 4.0, 4.0))
  );
}

#[test]
fn test_sub() {
  assert_eq!(
    Vec4::new((0.0, 0.0, 0.0, 0.0)) - Vec4::new((0.0, 0.0, 0.0, 0.0)),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((1.0, 0.0, 0.0, 0.0)) - Vec4::new((0.0, 0.0, 0.0, 0.0)),
    Vec4::new((1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 0.0, 0.0, 0.0)) - Vec4::new((0.0, 0.0, 0.0, 0.0)),
    Vec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 1.0, 0.0, 0.0)) - Vec4::new((0.0, 0.0, 0.0, 0.0)),
    Vec4::new((2.0, 1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)) - Vec4::new((0.0, 0.0, 0.0, 0.0)),
    Vec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)) - Vec4::new((1.0, 0.0, 0.0, 0.0)),
    Vec4::new((1.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)) - Vec4::new((2.0, 0.0, 0.0, 0.0)),
    Vec4::new((0.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)) - Vec4::new((2.0, 1.0, 0.0, 0.0)),
    Vec4::new((0.0, 1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)) - Vec4::new((2.0, 2.0, 0.0, 0.0)),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 1.0, 0.0)) - Vec4::new((2.0, 2.0, 0.0, 0.0)),
    Vec4::new((0.0, 0.0, 1.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 0.0)) - Vec4::new((2.0, 2.0, 0.0, 0.0)),
    Vec4::new((0.0, 0.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 0.0)) - Vec4::new((2.0, 2.0, 1.0, 0.0)),
    Vec4::new((0.0, 0.0, 1.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 0.0)) - Vec4::new((2.0, 2.0, 2.0, 0.0)),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 1.0)) - Vec4::new((2.0, 2.0, 2.0, 0.0)),
    Vec4::new((0.0, 0.0, 0.0, 1.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 2.0)) - Vec4::new((2.0, 2.0, 2.0, 0.0)),
    Vec4::new((0.0, 0.0, 0.0, 2.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 2.0)) - Vec4::new((2.0, 2.0, 2.0, 1.0)),
    Vec4::new((0.0, 0.0, 0.0, 1.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 2.0)) - Vec4::new((2.0, 2.0, 2.0, 2.0)),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
}

#[test]
fn test_mul() {
  assert_eq!(
    Vec4::new((0.0, 0.0, 0.0, 0.0)) * 0.0,
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((1.0, 0.0, 0.0, 0.0)) * 0.0,
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 0.0, 0.0, 0.0)) * 0.0,
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 1.0, 0.0, 0.0)) * 0.0,
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)) * 0.0,
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)) * 1.0,
    Vec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)) * 2.0,
    Vec4::new((4.0, 4.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 1.0, 0.0)) * 2.0,
    Vec4::new((4.0, 4.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 0.0)) * 2.0,
    Vec4::new((4.0, 4.0, 4.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 1.0)) * 2.0,
    Vec4::new((4.0, 4.0, 4.0, 2.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 2.0)) * 2.0,
    Vec4::new((4.0, 4.0, 4.0, 4.0))
  );
}

#[test]
fn test_div() {
  assert_eq!(
    Vec4::new((0.0, 0.0, 0.0, 0.0)) / 1.0,
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((1.0, 0.0, 0.0, 0.0)) / 1.0,
    Vec4::new((1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 0.0, 0.0, 0.0)) / 1.0,
    Vec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 1.0, 0.0, 0.0)) / 1.0,
    Vec4::new((2.0, 1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)) / 1.0,
    Vec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)) / 2.0,
    Vec4::new((1.0, 1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)) / 4.0,
    Vec4::new((0.5, 0.5, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 1.0, 0.0)) / 4.0,
    Vec4::new((0.5, 0.5, 0.25, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 0.0)) / 4.0,
    Vec4::new((0.5, 0.5, 0.5, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 1.0)) / 4.0,
    Vec4::new((0.5, 0.5, 0.5, 0.25))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 2.0)) / 4.0,
    Vec4::new((0.5, 0.5, 0.5, 0.5))
  );
}

#[test]
fn test_set() {
  let mut vec = Vec4::new(());
  vec.set(Vec4::new((0.0, 0.0, 0.0, 0.0)));
  assert_eq!(vec, Vec4::new((0.0, 0.0, 0.0, 0.0)));
  vec.set(Vec4::new((1.0, 0.0, 0.0, 0.0)));
  assert_eq!(vec, Vec4::new((1.0, 0.0, 0.0, 0.0)));
  vec.set(Vec4::new((2.0, 0.0, 0.0, 0.0)));
  assert_eq!(vec, Vec4::new((2.0, 0.0, 0.0, 0.0)));
  vec.set(Vec4::new((2.0, 1.0, 0.0, 0.0)));
  assert_eq!(vec, Vec4::new((2.0, 1.0, 0.0, 0.0)));
  vec.set(Vec4::new((2.0, 2.0, 0.0, 0.0)));
  assert_eq!(vec, Vec4::new((2.0, 2.0, 0.0, 0.0)));
  vec.set(Vec4::new((2.0, 2.0, 1.0, 0.0)));
  assert_eq!(vec, Vec4::new((2.0, 2.0, 1.0, 0.0)));
  vec.set(Vec4::new((2.0, 2.0, 2.0, 0.0)));
  assert_eq!(vec, Vec4::new((2.0, 2.0, 2.0, 0.0)));
  vec.set(Vec4::new((2.0, 2.0, 2.0, 1.0)));
  assert_eq!(vec, Vec4::new((2.0, 2.0, 2.0, 1.0)));
  vec.set(Vec4::new((2.0, 2.0, 2.0, 2.0)));
  assert_eq!(vec, Vec4::new((2.0, 2.0, 2.0, 2.0)));
}

#[test]
fn test_add_assign() {
  let mut vec = Vec4::new(());
  vec += Vec4::new((0.0, 0.0, 0.0, 0.0));
  assert_eq!(vec, Vec4::new((0.0, 0.0, 0.0, 0.0)));
  vec += Vec4::new((1.0, 0.0, 0.0, 0.0));
  assert_eq!(vec, Vec4::new((1.0, 0.0, 0.0, 0.0)));
  vec += Vec4::new((2.0, 0.0, 0.0, 0.0));
  assert_eq!(vec, Vec4::new((3.0, 0.0, 0.0, 0.0)));
  vec += Vec4::new((2.0, 1.0, 0.0, 0.0));
  assert_eq!(vec, Vec4::new((5.0, 1.0, 0.0, 0.0)));
  vec += Vec4::new((2.0, 2.0, 0.0, 0.0));
  assert_eq!(vec, Vec4::new((7.0, 3.0, 0.0, 0.0)));
  vec += Vec4::new((2.0, 2.0, 1.0, 0.0));
  assert_eq!(vec, Vec4::new((9.0, 5.0, 1.0, 0.0)));
  vec += Vec4::new((2.0, 2.0, 2.0, 0.0));
  assert_eq!(vec, Vec4::new((11.0, 7.0, 3.0, 0.0)));
  vec += Vec4::new((2.0, 2.0, 2.0, 1.0));
  assert_eq!(vec, Vec4::new((13.0, 9.0, 5.0, 1.0)));
  vec += Vec4::new((2.0, 2.0, 2.0, 2.0));
  assert_eq!(vec, Vec4::new((15.0, 11.0, 7.0, 3.0)));
}

#[test]
fn test_sub_assign() {
  let mut vec = Vec4::new(());
  vec -= Vec4::new((0.0, 0.0, 0.0, 0.0));
  assert_eq!(vec, Vec4::new((0.0, 0.0, 0.0, 0.0)));
  vec -= Vec4::new((1.0, 0.0, 0.0, 0.0));
  assert_eq!(vec, Vec4::new((-1.0, 0.0, 0.0, 0.0)));
  vec -= Vec4::new((2.0, 0.0, 0.0, 0.0));
  assert_eq!(vec, Vec4::new((-3.0, 0.0, 0.0, 0.0)));
  vec -= Vec4::new((2.0, 1.0, 0.0, 0.0));
  assert_eq!(vec, Vec4::new((-5.0, -1.0, 0.0, 0.0)));
  vec -= Vec4::new((2.0, 2.0, 0.0, 0.0));
  assert_eq!(vec, Vec4::new((-7.0, -3.0, 0.0, 0.0)));
  vec -= Vec4::new((2.0, 2.0, 1.0, 0.0));
  assert_eq!(vec, Vec4::new((-9.0, -5.0, -1.0, 0.0)));
  vec -= Vec4::new((2.0, 2.0, 2.0, 0.0));
  assert_eq!(vec, Vec4::new((-11.0, -7.0, -3.0, 0.0)));
  vec -= Vec4::new((2.0, 2.0, 2.0, 1.0));
  assert_eq!(vec, Vec4::new((-13.0, -9.0, -5.0, -1.0)));
  vec -= Vec4::new((2.0, 2.0, 2.0, 2.0));
  assert_eq!(vec, Vec4::new((-15.0, -11.0, -7.0, -3.0)));
}

#[test]
fn test_mul_assign() {
  let mut vec = Vec4::new((1.0, 1.0, 1.0, 1.0));
  vec *= 1.0;
  assert_eq!(vec, Vec4::new((1.0, 1.0, 1.0, 1.0)));
  vec *= 2.0;
  assert_eq!(vec, Vec4::new((2.0, 2.0, 2.0, 2.0)));
  vec *= 3.0;
  assert_eq!(vec, Vec4::new((6.0, 6.0, 6.0, 6.0)));
}

#[test]
fn test_div_assign() {
  let mut vec = Vec4::new((1.0, 1.0, 1.0, 1.0));
  vec /= 1.0;
  assert_eq!(vec, Vec4::new((1.0, 1.0, 1.0, 1.0)));
  vec /= 2.0;
  assert_eq!(vec, Vec4::new((0.5, 0.5, 0.5, 0.5)));
  vec /= 5.0;
  assert_eq!(vec, Vec4::new((0.1, 0.1, 0.1, 0.1)));
}

#[test]
fn test_get_squared_len() {
  assert_eq!(Vec4::new((0.0, 0.0, 0.0, 0.0)).get_squared_len(), 0.0);
  assert_eq!(Vec4::new((1.0, 0.0, 0.0, 0.0)).get_squared_len(), 1.0);
  assert_eq!(Vec4::new((2.0, 0.0, 0.0, 0.0)).get_squared_len(), 4.0);
  assert_eq!(Vec4::new((2.0, 1.0, 0.0, 0.0)).get_squared_len(), 5.0);
  assert_eq!(Vec4::new((2.0, 2.0, 0.0, 0.0)).get_squared_len(), 8.0);
  assert_eq!(Vec4::new((2.0, 2.0, 1.0, 0.0)).get_squared_len(), 9.0);
  assert_eq!(Vec4::new((2.0, 2.0, 2.0, 0.0)).get_squared_len(), 12.0);
  assert_eq!(Vec4::new((2.0, 2.0, 2.0, 1.0)).get_squared_len(), 13.0);
  assert_eq!(Vec4::new((2.0, 2.0, 2.0, 2.0)).get_squared_len(), 16.0);
}

#[test]
fn test_get_len() {
  assert_eq!(Vec4::new((0.0, 0.0, 0.0, 0.0)).get_len(), 0.0);
  assert_eq!(Vec4::new((1.0, 0.0, 0.0, 0.0)).get_len(), 1.0);
  assert_eq!(Vec4::new((2.0, 0.0, 0.0, 0.0)).get_len(), 2.0);
  assert_eq!(Vec4::new((2.0, 1.0, 0.0, 0.0)).get_len(), 5.0f64.sqrt());
  assert_eq!(Vec4::new((2.0, 2.0, 0.0, 0.0)).get_len(), 8.0f64.sqrt());
  assert!(Vec4::new((2.0, 2.0, 1.0, 0.0)).get_len().approx_eq(3.0));
  assert!(Vec4::new((2.0, 2.0, 2.0, 0.0))
    .get_len()
    .approx_eq(12.0f64.sqrt()));
  assert!(Vec4::new((2.0, 2.0, 2.0, 1.0))
    .get_len()
    .approx_eq(13.0f64.sqrt()));
  assert_eq!(Vec4::new((2.0, 2.0, 2.0, 2.0)).get_len(), 4.0);
}

#[test]
fn test_get_normalized() {
  assert_eq!(
    Vec4::new((1.0, 0.0, 0.0, 0.0)).get_normalized(),
    Vec4::new((1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 0.0, 0.0, 0.0)).get_normalized(),
    Vec4::new((1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((3.0, 0.0, 0.0, 0.0)).get_normalized(),
    Vec4::new((1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((3.0, 4.0, 0.0, 0.0)).get_normalized(),
    Vec4::new((0.6, 0.8, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((6.0, 8.0, 0.0, 0.0)).get_normalized(),
    Vec4::new((0.6, 0.8, 0.0, 0.0))
  );
  assert!(Vec4::new((2.0, 2.0, 1.0, 0.0))
    .get_normalized()
    .approx_eq(Vec4::new((2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0, 0.0))));
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 2.0)).get_normalized(),
    Vec4::new((0.5, 0.5, 0.5, 0.5))
  );
}

#[test]
fn test_get_x_fliped() {
  assert_eq!(
    Vec4::new((0.0, 0.0, 0.0, 0.0)).get_x_fliped(),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((1.0, 0.0, 0.0, 0.0)).get_x_fliped(),
    Vec4::new((-1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 0.0, 0.0, 0.0)).get_x_fliped(),
    Vec4::new((-2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 1.0, 0.0, 0.0)).get_x_fliped(),
    Vec4::new((-2.0, 1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)).get_x_fliped(),
    Vec4::new((-2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 1.0, 0.0)).get_x_fliped(),
    Vec4::new((-2.0, 2.0, 1.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 0.0)).get_x_fliped(),
    Vec4::new((-2.0, 2.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 1.0)).get_x_fliped(),
    Vec4::new((-2.0, 2.0, 2.0, 1.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 2.0)).get_x_fliped(),
    Vec4::new((-2.0, 2.0, 2.0, 2.0))
  );
}

#[test]
fn test_get_y_fliped() {
  assert_eq!(
    Vec4::new((0.0, 0.0, 0.0, 0.0)).get_y_fliped(),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((1.0, 0.0, 0.0, 0.0)).get_y_fliped(),
    Vec4::new((1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 0.0, 0.0, 0.0)).get_y_fliped(),
    Vec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 1.0, 0.0, 0.0)).get_y_fliped(),
    Vec4::new((2.0, -1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)).get_y_fliped(),
    Vec4::new((2.0, -2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 1.0, 0.0)).get_y_fliped(),
    Vec4::new((2.0, -2.0, 1.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 0.0)).get_y_fliped(),
    Vec4::new((2.0, -2.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 1.0)).get_y_fliped(),
    Vec4::new((2.0, -2.0, 2.0, 1.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 2.0)).get_y_fliped(),
    Vec4::new((2.0, -2.0, 2.0, 2.0))
  );
}

#[test]
fn test_get_z_fliped() {
  assert_eq!(
    Vec4::new((0.0, 0.0, 0.0, 0.0)).get_z_fliped(),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((1.0, 0.0, 0.0, 0.0)).get_z_fliped(),
    Vec4::new((1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 0.0, 0.0, 0.0)).get_z_fliped(),
    Vec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 1.0, 0.0, 0.0)).get_z_fliped(),
    Vec4::new((2.0, 1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)).get_z_fliped(),
    Vec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 1.0, 0.0)).get_z_fliped(),
    Vec4::new((2.0, 2.0, -1.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 0.0)).get_z_fliped(),
    Vec4::new((2.0, 2.0, -2.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 1.0)).get_z_fliped(),
    Vec4::new((2.0, 2.0, -2.0, 1.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 2.0)).get_z_fliped(),
    Vec4::new((2.0, 2.0, -2.0, 2.0))
  );
}

#[test]
fn test_get_w_fliped() {
  assert_eq!(
    Vec4::new((0.0, 0.0, 0.0, 0.0)).get_w_fliped(),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((1.0, 0.0, 0.0, 0.0)).get_w_fliped(),
    Vec4::new((1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 0.0, 0.0, 0.0)).get_w_fliped(),
    Vec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 1.0, 0.0, 0.0)).get_w_fliped(),
    Vec4::new((2.0, 1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)).get_w_fliped(),
    Vec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 1.0, 0.0)).get_w_fliped(),
    Vec4::new((2.0, 2.0, 1.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 0.0)).get_w_fliped(),
    Vec4::new((2.0, 2.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 1.0)).get_w_fliped(),
    Vec4::new((2.0, 2.0, 2.0, -1.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 2.0)).get_w_fliped(),
    Vec4::new((2.0, 2.0, 2.0, -2.0))
  );
}

#[test]
fn test_get_fliped() {
  assert_eq!(
    Vec4::new((0.0, 0.0, 0.0, 0.0)).get_fliped(),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((1.0, 0.0, 0.0, 0.0)).get_fliped(),
    Vec4::new((-1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 0.0, 0.0, 0.0)).get_fliped(),
    Vec4::new((-2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 1.0, 0.0, 0.0)).get_fliped(),
    Vec4::new((-2.0, -1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 0.0, 0.0)).get_fliped(),
    Vec4::new((-2.0, -2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 1.0, 0.0)).get_fliped(),
    Vec4::new((-2.0, -2.0, -1.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 0.0)).get_fliped(),
    Vec4::new((-2.0, -2.0, -2.0, 0.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 1.0)).get_fliped(),
    Vec4::new((-2.0, -2.0, -2.0, -1.0))
  );
  assert_eq!(
    Vec4::new((2.0, 2.0, 2.0, 2.0)).get_fliped(),
    Vec4::new((-2.0, -2.0, -2.0, -2.0))
  );
}

#[test]
fn test_display() {
  assert_eq!(format!("{}", Vec4::new((0.0, 0.0, 0.0, 0.0))), "0,0,0,0");
  assert_eq!(format!("{}", Vec4::new((1.0, 0.0, 0.0, 0.0))), "1,0,0,0");
  assert_eq!(format!("{}", Vec4::new((2.0, 0.0, 0.0, 0.0))), "2,0,0,0");
  assert_eq!(format!("{}", Vec4::new((2.0, 1.0, 0.0, 0.0))), "2,1,0,0");
  assert_eq!(format!("{}", Vec4::new((2.0, 2.0, 0.0, 0.0))), "2,2,0,0");
  assert_eq!(format!("{}", Vec4::new((2.0, 2.0, 1.0, 0.0))), "2,2,1,0");
  assert_eq!(format!("{}", Vec4::new((2.0, 2.0, 2.0, 0.0))), "2,2,2,0");
  assert_eq!(format!("{}", Vec4::new((2.0, 2.0, 2.0, 1.0))), "2,2,2,1");
  assert_eq!(format!("{}", Vec4::new((2.0, 2.0, 2.0, 2.0))), "2,2,2,2");
}

#[test]
fn test_approx_eq() {
  assert!(Vec4::new((0.0, 0.0, 0.0, 0.0)).approx_eq(Vec4::new((0.0, 0.0, 0.0, 0.0))));
  assert!(!Vec4::new((1.0, 0.0, 0.0, 0.0)).approx_eq(Vec4::new((0.0, 0.0, 0.0, 0.0))));
  assert!(!Vec4::new((2.0, 0.0, 0.0, 0.0)).approx_eq(Vec4::new((0.0, 0.0, 0.0, 0.0))));
  assert!(!Vec4::new((2.0, 1.0, 0.0, 0.0)).approx_eq(Vec4::new((0.0, 0.0, 0.0, 0.0))));
  assert!(!Vec4::new((2.0, 2.0, 0.0, 0.0)).approx_eq(Vec4::new((0.0, 0.0, 0.0, 0.0))));
  assert!(!Vec4::new((2.0, 2.0, 0.0, 0.0)).approx_eq(Vec4::new((1.0, 0.0, 0.0, 0.0))));
  assert!(!Vec4::new((2.0, 2.0, 0.0, 0.0)).approx_eq(Vec4::new((2.0, 0.0, 0.0, 0.0))));
  assert!(!Vec4::new((2.0, 2.0, 0.0, 0.0)).approx_eq(Vec4::new((2.0, 1.0, 0.0, 0.0))));
  assert!(Vec4::new((2.0, 2.0, 0.0, 0.0)).approx_eq(Vec4::new((2.0, 2.0, 0.0, 0.0))));
  assert!(!Vec4::new((2.0, 2.0, 1.0, 0.0)).approx_eq(Vec4::new((2.0, 2.0, 0.0, 0.0))));
  assert!(!Vec4::new((2.0, 2.0, 2.0, 0.0)).approx_eq(Vec4::new((2.0, 2.0, 0.0, 0.0))));
  assert!(!Vec4::new((2.0, 2.0, 2.0, 0.0)).approx_eq(Vec4::new((2.0, 2.0, 1.0, 0.0))));
  assert!(Vec4::new((2.0, 2.0, 2.0, 0.0)).approx_eq(Vec4::new((2.0, 2.0, 2.0, 0.0))));
  assert!(!Vec4::new((2.0, 2.0, 2.0, 1.0)).approx_eq(Vec4::new((2.0, 2.0, 2.0, 0.0))));
  assert!(!Vec4::new((2.0, 2.0, 2.0, 2.0)).approx_eq(Vec4::new((2.0, 2.0, 2.0, 0.0))));
  assert!(!Vec4::new((2.0, 2.0, 2.0, 2.0)).approx_eq(Vec4::new((2.0, 2.0, 2.0, 1.0))));
  assert!(Vec4::new((2.0, 2.0, 2.0, 2.0)).approx_eq(Vec4::new((2.0, 2.0, 2.0, 2.0))));
}
