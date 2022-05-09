use iron::ApproxEq;
use iron::Vec3;

#[test]
fn test_from_f64() {
  assert_eq!(Vec3::new(0.0), Vec3::new((0.0, 0.0, 0.0)));
  assert_eq!(Vec3::new(1.0), Vec3::new((1.0, 1.0, 1.0)));
  assert_eq!(Vec3::new(2.0), Vec3::new((2.0, 2.0, 2.0)));
}

#[test]
fn test_with_x() {
  assert_eq!(
    Vec3::new((0.0, 0.0, 0.0)).with_x(0.0),
    Vec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((1.0, 0.0, 0.0)).with_x(0.0),
    Vec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 0.0, 0.0)).with_x(0.0),
    Vec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 1.0, 0.0)).with_x(0.0),
    Vec3::new((0.0, 1.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)).with_x(0.0),
    Vec3::new((0.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)).with_x(1.0),
    Vec3::new((1.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)).with_x(2.0),
    Vec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 1.0)).with_x(2.0),
    Vec3::new((2.0, 2.0, 1.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 2.0)).with_x(2.0),
    Vec3::new((2.0, 2.0, 2.0))
  );
}

#[test]
fn test_with_y() {
  assert_eq!(
    Vec3::new((0.0, 0.0, 0.0)).with_y(0.0),
    Vec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((1.0, 0.0, 0.0)).with_y(0.0),
    Vec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 0.0, 0.0)).with_y(0.0),
    Vec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 1.0, 0.0)).with_y(0.0),
    Vec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)).with_y(0.0),
    Vec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)).with_y(1.0),
    Vec3::new((2.0, 1.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)).with_y(2.0),
    Vec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 1.0)).with_y(2.0),
    Vec3::new((2.0, 2.0, 1.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 2.0)).with_y(2.0),
    Vec3::new((2.0, 2.0, 2.0))
  );
}

#[test]
fn test_with_z() {
  assert_eq!(
    Vec3::new((0.0, 0.0, 0.0)).with_z(0.0),
    Vec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((1.0, 0.0, 0.0)).with_z(0.0),
    Vec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 0.0, 0.0)).with_z(0.0),
    Vec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 1.0, 0.0)).with_z(0.0),
    Vec3::new((2.0, 1.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)).with_z(0.0),
    Vec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)).with_z(1.0),
    Vec3::new((2.0, 2.0, 1.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)).with_z(2.0),
    Vec3::new((2.0, 2.0, 2.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 1.0)).with_z(2.0),
    Vec3::new((2.0, 2.0, 2.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 2.0)).with_z(2.0),
    Vec3::new((2.0, 2.0, 2.0))
  );
}

#[test]
fn test_add() {
  assert_eq!(
    Vec3::new((0.0, 0.0, 0.0)) + Vec3::new((0.0, 0.0, 0.0)),
    Vec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((1.0, 0.0, 0.0)) + Vec3::new((0.0, 0.0, 0.0)),
    Vec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 0.0, 0.0)) + Vec3::new((0.0, 0.0, 0.0)),
    Vec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 1.0, 0.0)) + Vec3::new((0.0, 0.0, 0.0)),
    Vec3::new((2.0, 1.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)) + Vec3::new((0.0, 0.0, 0.0)),
    Vec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)) + Vec3::new((1.0, 0.0, 0.0)),
    Vec3::new((3.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)) + Vec3::new((2.0, 0.0, 0.0)),
    Vec3::new((4.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)) + Vec3::new((2.0, 1.0, 0.0)),
    Vec3::new((4.0, 3.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)) + Vec3::new((2.0, 2.0, 0.0)),
    Vec3::new((4.0, 4.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 1.0)) + Vec3::new((2.0, 2.0, 0.0)),
    Vec3::new((4.0, 4.0, 1.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 2.0)) + Vec3::new((2.0, 2.0, 0.0)),
    Vec3::new((4.0, 4.0, 2.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 2.0)) + Vec3::new((2.0, 2.0, 1.0)),
    Vec3::new((4.0, 4.0, 3.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 2.0)) + Vec3::new((2.0, 2.0, 2.0)),
    Vec3::new((4.0, 4.0, 4.0))
  );
}

#[test]
fn test_sub() {
  assert_eq!(
    Vec3::new((0.0, 0.0, 0.0)) - Vec3::new((0.0, 0.0, 0.0)),
    Vec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((1.0, 0.0, 0.0)) - Vec3::new((0.0, 0.0, 0.0)),
    Vec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 0.0, 0.0)) - Vec3::new((0.0, 0.0, 0.0)),
    Vec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 1.0, 0.0)) - Vec3::new((0.0, 0.0, 0.0)),
    Vec3::new((2.0, 1.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)) - Vec3::new((0.0, 0.0, 0.0)),
    Vec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)) - Vec3::new((1.0, 0.0, 0.0)),
    Vec3::new((1.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)) - Vec3::new((2.0, 0.0, 0.0)),
    Vec3::new((0.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)) - Vec3::new((2.0, 1.0, 0.0)),
    Vec3::new((0.0, 1.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)) - Vec3::new((2.0, 2.0, 0.0)),
    Vec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 1.0)) - Vec3::new((2.0, 2.0, 0.0)),
    Vec3::new((0.0, 0.0, 1.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 2.0)) - Vec3::new((2.0, 2.0, 0.0)),
    Vec3::new((0.0, 0.0, 2.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 2.0)) - Vec3::new((2.0, 2.0, 1.0)),
    Vec3::new((0.0, 0.0, 1.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 2.0)) - Vec3::new((2.0, 2.0, 2.0)),
    Vec3::new((0.0, 0.0, 0.0))
  );
}

#[test]
fn test_mul() {
  assert_eq!(Vec3::new((0.0, 0.0, 0.0)) * 0.0, Vec3::new((0.0, 0.0, 0.0)));
  assert_eq!(Vec3::new((1.0, 0.0, 0.0)) * 0.0, Vec3::new((0.0, 0.0, 0.0)));
  assert_eq!(Vec3::new((2.0, 0.0, 0.0)) * 0.0, Vec3::new((0.0, 0.0, 0.0)));
  assert_eq!(Vec3::new((2.0, 1.0, 0.0)) * 0.0, Vec3::new((0.0, 0.0, 0.0)));
  assert_eq!(Vec3::new((2.0, 2.0, 0.0)) * 0.0, Vec3::new((0.0, 0.0, 0.0)));
  assert_eq!(Vec3::new((2.0, 2.0, 0.0)) * 1.0, Vec3::new((2.0, 2.0, 0.0)));
  assert_eq!(Vec3::new((2.0, 2.0, 0.0)) * 2.0, Vec3::new((4.0, 4.0, 0.0)));
  assert_eq!(Vec3::new((2.0, 2.0, 1.0)) * 2.0, Vec3::new((4.0, 4.0, 2.0)));
  assert_eq!(Vec3::new((2.0, 2.0, 2.0)) * 2.0, Vec3::new((4.0, 4.0, 4.0)));
}

#[test]
fn test_div() {
  assert_eq!(Vec3::new((0.0, 0.0, 0.0)) / 1.0, Vec3::new((0.0, 0.0, 0.0)));
  assert_eq!(Vec3::new((1.0, 0.0, 0.0)) / 1.0, Vec3::new((1.0, 0.0, 0.0)));
  assert_eq!(Vec3::new((2.0, 0.0, 0.0)) / 1.0, Vec3::new((2.0, 0.0, 0.0)));
  assert_eq!(Vec3::new((2.0, 1.0, 0.0)) / 1.0, Vec3::new((2.0, 1.0, 0.0)));
  assert_eq!(Vec3::new((2.0, 2.0, 0.0)) / 1.0, Vec3::new((2.0, 2.0, 0.0)));
  assert_eq!(Vec3::new((2.0, 2.0, 0.0)) / 2.0, Vec3::new((1.0, 1.0, 0.0)));
  assert_eq!(Vec3::new((2.0, 2.0, 0.0)) / 4.0, Vec3::new((0.5, 0.5, 0.0)));
  assert_eq!(
    Vec3::new((2.0, 2.0, 1.0)) / 4.0,
    Vec3::new((0.5, 0.5, 0.25))
  );
  assert_eq!(Vec3::new((2.0, 2.0, 2.0)) / 4.0, Vec3::new((0.5, 0.5, 0.5)));
}

#[test]
fn test_set() {
  let mut vec = Vec3::new(());
  vec.set(Vec3::new((0.0, 0.0, 0.0)));
  assert_eq!(vec, Vec3::new((0.0, 0.0, 0.0)));
  vec.set(Vec3::new((1.0, 0.0, 0.0)));
  assert_eq!(vec, Vec3::new((1.0, 0.0, 0.0)));
  vec.set(Vec3::new((2.0, 0.0, 0.0)));
  assert_eq!(vec, Vec3::new((2.0, 0.0, 0.0)));
  vec.set(Vec3::new((2.0, 1.0, 0.0)));
  assert_eq!(vec, Vec3::new((2.0, 1.0, 0.0)));
  vec.set(Vec3::new((2.0, 2.0, 0.0)));
  assert_eq!(vec, Vec3::new((2.0, 2.0, 0.0)));
  vec.set(Vec3::new((2.0, 2.0, 1.0)));
  assert_eq!(vec, Vec3::new((2.0, 2.0, 1.0)));
  vec.set(Vec3::new((2.0, 2.0, 2.0)));
  assert_eq!(vec, Vec3::new((2.0, 2.0, 2.0)));
}

#[test]
fn test_add_assign() {
  let mut vec = Vec3::new(());
  vec += Vec3::new((0.0, 0.0, 0.0));
  assert_eq!(vec, Vec3::new((0.0, 0.0, 0.0)));
  vec += Vec3::new((1.0, 0.0, 0.0));
  assert_eq!(vec, Vec3::new((1.0, 0.0, 0.0)));
  vec += Vec3::new((2.0, 0.0, 0.0));
  assert_eq!(vec, Vec3::new((3.0, 0.0, 0.0)));
  vec += Vec3::new((2.0, 1.0, 0.0));
  assert_eq!(vec, Vec3::new((5.0, 1.0, 0.0)));
  vec += Vec3::new((2.0, 2.0, 0.0));
  assert_eq!(vec, Vec3::new((7.0, 3.0, 0.0)));
  vec += Vec3::new((2.0, 2.0, 1.0));
  assert_eq!(vec, Vec3::new((9.0, 5.0, 1.0)));
  vec += Vec3::new((2.0, 2.0, 2.0));
  assert_eq!(vec, Vec3::new((11.0, 7.0, 3.0)));
}

#[test]
fn test_sub_assign() {
  let mut vec = Vec3::new(());
  vec -= Vec3::new((0.0, 0.0, 0.0));
  assert_eq!(vec, Vec3::new((0.0, 0.0, 0.0)));
  vec -= Vec3::new((1.0, 0.0, 0.0));
  assert_eq!(vec, Vec3::new((-1.0, 0.0, 0.0)));
  vec -= Vec3::new((2.0, 0.0, 0.0));
  assert_eq!(vec, Vec3::new((-3.0, 0.0, 0.0)));
  vec -= Vec3::new((2.0, 1.0, 0.0));
  assert_eq!(vec, Vec3::new((-5.0, -1.0, 0.0)));
  vec -= Vec3::new((2.0, 2.0, 0.0));
  assert_eq!(vec, Vec3::new((-7.0, -3.0, 0.0)));
  vec -= Vec3::new((2.0, 2.0, 1.0));
  assert_eq!(vec, Vec3::new((-9.0, -5.0, -1.0)));
  vec -= Vec3::new((2.0, 2.0, 2.0));
  assert_eq!(vec, Vec3::new((-11.0, -7.0, -3.0)));
}

#[test]
fn test_mul_assign() {
  let mut vec = Vec3::new((1.0, 1.0, 1.0));
  vec *= 1.0;
  assert_eq!(vec, Vec3::new((1.0, 1.0, 1.0)));
  vec *= 2.0;
  assert_eq!(vec, Vec3::new((2.0, 2.0, 2.0)));
  vec *= 3.0;
  assert_eq!(vec, Vec3::new((6.0, 6.0, 6.0)));
}

#[test]
fn test_div_assign() {
  let mut vec = Vec3::new((1.0, 1.0, 1.0));
  vec /= 1.0;
  assert_eq!(vec, Vec3::new((1.0, 1.0, 1.0)));
  vec /= 2.0;
  assert_eq!(vec, Vec3::new((0.5, 0.5, 0.5)));
  vec /= 5.0;
  assert_eq!(vec, Vec3::new((0.1, 0.1, 0.1)));
}

#[test]
fn test_get_squared_len() {
  assert_eq!(Vec3::new((0.0, 0.0, 0.0)).get_squared_len(), 0.0);
  assert_eq!(Vec3::new((1.0, 0.0, 0.0)).get_squared_len(), 1.0);
  assert_eq!(Vec3::new((2.0, 0.0, 0.0)).get_squared_len(), 4.0);
  assert_eq!(Vec3::new((2.0, 1.0, 0.0)).get_squared_len(), 5.0);
  assert_eq!(Vec3::new((2.0, 2.0, 0.0)).get_squared_len(), 8.0);
  assert_eq!(Vec3::new((2.0, 2.0, 1.0)).get_squared_len(), 9.0);
  assert_eq!(Vec3::new((2.0, 2.0, 2.0)).get_squared_len(), 12.0);
}

#[test]
fn test_get_len() {
  assert_eq!(Vec3::new((0.0, 0.0, 0.0)).get_len(), 0.0);
  assert_eq!(Vec3::new((1.0, 0.0, 0.0)).get_len(), 1.0);
  assert_eq!(Vec3::new((2.0, 0.0, 0.0)).get_len(), 2.0);
  assert_eq!(Vec3::new((2.0, 1.0, 0.0)).get_len(), 5.0f64.sqrt());
  assert_eq!(Vec3::new((2.0, 2.0, 0.0)).get_len(), 8.0f64.sqrt());
  assert!(Vec3::new((2.0, 2.0, 1.0)).get_len().approx_eq(3.0));
  assert!(Vec3::new((2.0, 2.0, 2.0))
    .get_len()
    .approx_eq(12.0f64.sqrt()));
}

#[test]
fn test_get_normalized() {
  assert_eq!(
    Vec3::new((1.0, 0.0, 0.0)).get_normalized(),
    Vec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 0.0, 0.0)).get_normalized(),
    Vec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((3.0, 0.0, 0.0)).get_normalized(),
    Vec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((3.0, 4.0, 0.0)).get_normalized(),
    Vec3::new((0.6, 0.8, 0.0))
  );
  assert_eq!(
    Vec3::new((6.0, 8.0, 0.0)).get_normalized(),
    Vec3::new((0.6, 0.8, 0.0))
  );
  assert!(Vec3::new((2.0, 2.0, 1.0))
    .get_normalized()
    .approx_eq(Vec3::new((2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0))));
}

#[test]
fn test_get_x_fliped() {
  assert_eq!(
    Vec3::new((0.0, 0.0, 0.0)).get_x_fliped(),
    Vec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((1.0, 0.0, 0.0)).get_x_fliped(),
    Vec3::new((-1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 0.0, 0.0)).get_x_fliped(),
    Vec3::new((-2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 1.0, 0.0)).get_x_fliped(),
    Vec3::new((-2.0, 1.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)).get_x_fliped(),
    Vec3::new((-2.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 1.0)).get_x_fliped(),
    Vec3::new((-2.0, 2.0, 1.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 2.0)).get_x_fliped(),
    Vec3::new((-2.0, 2.0, 2.0))
  );
}

#[test]
fn test_get_y_fliped() {
  assert_eq!(
    Vec3::new((0.0, 0.0, 0.0)).get_y_fliped(),
    Vec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((1.0, 0.0, 0.0)).get_y_fliped(),
    Vec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 0.0, 0.0)).get_y_fliped(),
    Vec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 1.0, 0.0)).get_y_fliped(),
    Vec3::new((2.0, -1.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)).get_y_fliped(),
    Vec3::new((2.0, -2.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 1.0)).get_y_fliped(),
    Vec3::new((2.0, -2.0, 1.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 2.0)).get_y_fliped(),
    Vec3::new((2.0, -2.0, 2.0))
  );
}

#[test]
fn test_get_z_fliped() {
  assert_eq!(
    Vec3::new((0.0, 0.0, 0.0)).get_z_fliped(),
    Vec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((1.0, 0.0, 0.0)).get_z_fliped(),
    Vec3::new((1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 0.0, 0.0)).get_z_fliped(),
    Vec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 1.0, 0.0)).get_z_fliped(),
    Vec3::new((2.0, 1.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)).get_z_fliped(),
    Vec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 1.0)).get_z_fliped(),
    Vec3::new((2.0, 2.0, -1.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 2.0)).get_z_fliped(),
    Vec3::new((2.0, 2.0, -2.0))
  );
}

#[test]
fn test_get_fliped() {
  assert_eq!(
    Vec3::new((0.0, 0.0, 0.0)).get_fliped(),
    Vec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((1.0, 0.0, 0.0)).get_fliped(),
    Vec3::new((-1.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 0.0, 0.0)).get_fliped(),
    Vec3::new((-2.0, 0.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 1.0, 0.0)).get_fliped(),
    Vec3::new((-2.0, -1.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 0.0)).get_fliped(),
    Vec3::new((-2.0, -2.0, 0.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 1.0)).get_fliped(),
    Vec3::new((-2.0, -2.0, -1.0))
  );
  assert_eq!(
    Vec3::new((2.0, 2.0, 2.0)).get_fliped(),
    Vec3::new((-2.0, -2.0, -2.0))
  );
}

#[test]
fn test_display() {
  assert_eq!(format!("{}", Vec3::new((0.0, 0.0, 0.0))), "0,0,0");
  assert_eq!(format!("{}", Vec3::new((1.0, 0.0, 0.0))), "1,0,0");
  assert_eq!(format!("{}", Vec3::new((2.0, 0.0, 0.0))), "2,0,0");
  assert_eq!(format!("{}", Vec3::new((2.0, 1.0, 0.0))), "2,1,0");
  assert_eq!(format!("{}", Vec3::new((2.0, 2.0, 0.0))), "2,2,0");
  assert_eq!(format!("{}", Vec3::new((2.0, 2.0, 1.0))), "2,2,1");
  assert_eq!(format!("{}", Vec3::new((2.0, 2.0, 2.0))), "2,2,2");
}

#[test]
fn test_approx_eq() {
  assert!(Vec3::new((0.0, 0.0, 0.0)).approx_eq(Vec3::new((0.0, 0.0, 0.0))));
  assert!(!Vec3::new((1.0, 0.0, 0.0)).approx_eq(Vec3::new((0.0, 0.0, 0.0))));
  assert!(!Vec3::new((2.0, 0.0, 0.0)).approx_eq(Vec3::new((0.0, 0.0, 0.0))));
  assert!(!Vec3::new((2.0, 1.0, 0.0)).approx_eq(Vec3::new((0.0, 0.0, 0.0))));
  assert!(!Vec3::new((2.0, 2.0, 0.0)).approx_eq(Vec3::new((0.0, 0.0, 0.0))));
  assert!(!Vec3::new((2.0, 2.0, 0.0)).approx_eq(Vec3::new((1.0, 0.0, 0.0))));
  assert!(!Vec3::new((2.0, 2.0, 0.0)).approx_eq(Vec3::new((2.0, 0.0, 0.0))));
  assert!(!Vec3::new((2.0, 2.0, 0.0)).approx_eq(Vec3::new((2.0, 1.0, 0.0))));
  assert!(Vec3::new((2.0, 2.0, 0.0)).approx_eq(Vec3::new((2.0, 2.0, 0.0))));
  assert!(!Vec3::new((2.0, 2.0, 1.0)).approx_eq(Vec3::new((2.0, 2.0, 0.0))));
  assert!(!Vec3::new((2.0, 2.0, 2.0)).approx_eq(Vec3::new((2.0, 2.0, 0.0))));
  assert!(!Vec3::new((2.0, 2.0, 2.0)).approx_eq(Vec3::new((2.0, 2.0, 1.0))));
  assert!(Vec3::new((2.0, 2.0, 2.0)).approx_eq(Vec3::new((2.0, 2.0, 2.0))));
}
