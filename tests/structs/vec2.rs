use iron::ApproxEq;
use iron::Vec2;

#[test]
fn test_from_f64() {
  assert_eq!(Vec2::new(0.0), Vec2::new((0.0, 0.0)));
  assert_eq!(Vec2::new(1.0), Vec2::new((1.0, 1.0)));
  assert_eq!(Vec2::new(2.0), Vec2::new((2.0, 2.0)));
}

#[test]
fn test_with_x() {
  assert_eq!(Vec2::new((0.0, 0.0)).with_x(0.0), Vec2::new((0.0, 0.0)));
  assert_eq!(Vec2::new((1.0, 0.0)).with_x(0.0), Vec2::new((0.0, 0.0)));
  assert_eq!(Vec2::new((2.0, 0.0)).with_x(0.0), Vec2::new((0.0, 0.0)));
  assert_eq!(Vec2::new((2.0, 1.0)).with_x(0.0), Vec2::new((0.0, 1.0)));
  assert_eq!(Vec2::new((2.0, 2.0)).with_x(0.0), Vec2::new((0.0, 2.0)));
  assert_eq!(Vec2::new((2.0, 2.0)).with_x(1.0), Vec2::new((1.0, 2.0)));
  assert_eq!(Vec2::new((2.0, 2.0)).with_x(2.0), Vec2::new((2.0, 2.0)));
}

#[test]
fn test_with_y() {
  assert_eq!(Vec2::new((0.0, 0.0)).with_y(0.0), Vec2::new((0.0, 0.0)));
  assert_eq!(Vec2::new((1.0, 0.0)).with_y(0.0), Vec2::new((1.0, 0.0)));
  assert_eq!(Vec2::new((2.0, 0.0)).with_y(0.0), Vec2::new((2.0, 0.0)));
  assert_eq!(Vec2::new((2.0, 1.0)).with_y(0.0), Vec2::new((2.0, 0.0)));
  assert_eq!(Vec2::new((2.0, 2.0)).with_y(0.0), Vec2::new((2.0, 0.0)));
  assert_eq!(Vec2::new((2.0, 2.0)).with_y(1.0), Vec2::new((2.0, 1.0)));
  assert_eq!(Vec2::new((2.0, 2.0)).with_y(2.0), Vec2::new((2.0, 2.0)));
}

#[test]
fn test_add() {
  assert_eq!(
    Vec2::new((0.0, 0.0)) + Vec2::new((0.0, 0.0)),
    Vec2::new((0.0, 0.0))
  );
  assert_eq!(
    Vec2::new((1.0, 0.0)) + Vec2::new((0.0, 0.0)),
    Vec2::new((1.0, 0.0))
  );
  assert_eq!(
    Vec2::new((2.0, 0.0)) + Vec2::new((0.0, 0.0)),
    Vec2::new((2.0, 0.0))
  );
  assert_eq!(
    Vec2::new((2.0, 1.0)) + Vec2::new((0.0, 0.0)),
    Vec2::new((2.0, 1.0))
  );
  assert_eq!(
    Vec2::new((2.0, 2.0)) + Vec2::new((0.0, 0.0)),
    Vec2::new((2.0, 2.0))
  );
  assert_eq!(
    Vec2::new((2.0, 2.0)) + Vec2::new((1.0, 0.0)),
    Vec2::new((3.0, 2.0))
  );
  assert_eq!(
    Vec2::new((2.0, 2.0)) + Vec2::new((2.0, 0.0)),
    Vec2::new((4.0, 2.0))
  );
  assert_eq!(
    Vec2::new((2.0, 2.0)) + Vec2::new((2.0, 1.0)),
    Vec2::new((4.0, 3.0))
  );
  assert_eq!(
    Vec2::new((2.0, 2.0)) + Vec2::new((2.0, 2.0)),
    Vec2::new((4.0, 4.0))
  );
}

#[test]
fn test_sub() {
  assert_eq!(
    Vec2::new((0.0, 0.0)) - Vec2::new((0.0, 0.0)),
    Vec2::new((0.0, 0.0))
  );
  assert_eq!(
    Vec2::new((1.0, 0.0)) - Vec2::new((0.0, 0.0)),
    Vec2::new((1.0, 0.0))
  );
  assert_eq!(
    Vec2::new((2.0, 0.0)) - Vec2::new((0.0, 0.0)),
    Vec2::new((2.0, 0.0))
  );
  assert_eq!(
    Vec2::new((2.0, 1.0)) - Vec2::new((0.0, 0.0)),
    Vec2::new((2.0, 1.0))
  );
  assert_eq!(
    Vec2::new((2.0, 2.0)) - Vec2::new((0.0, 0.0)),
    Vec2::new((2.0, 2.0))
  );
  assert_eq!(
    Vec2::new((2.0, 2.0)) - Vec2::new((1.0, 0.0)),
    Vec2::new((1.0, 2.0))
  );
  assert_eq!(
    Vec2::new((2.0, 2.0)) - Vec2::new((2.0, 0.0)),
    Vec2::new((0.0, 2.0))
  );
  assert_eq!(
    Vec2::new((2.0, 2.0)) - Vec2::new((2.0, 1.0)),
    Vec2::new((0.0, 1.0))
  );
  assert_eq!(
    Vec2::new((2.0, 2.0)) - Vec2::new((2.0, 2.0)),
    Vec2::new((0.0, 0.0))
  );
}

#[test]
fn test_mul() {
  assert_eq!(Vec2::new((0.0, 0.0)) * 0.0, Vec2::new((0.0, 0.0)));
  assert_eq!(Vec2::new((1.0, 0.0)) * 0.0, Vec2::new((0.0, 0.0)));
  assert_eq!(Vec2::new((2.0, 0.0)) * 0.0, Vec2::new((0.0, 0.0)));
  assert_eq!(Vec2::new((2.0, 1.0)) * 0.0, Vec2::new((0.0, 0.0)));
  assert_eq!(Vec2::new((2.0, 2.0)) * 0.0, Vec2::new((0.0, 0.0)));
  assert_eq!(Vec2::new((2.0, 2.0)) * 1.0, Vec2::new((2.0, 2.0)));
  assert_eq!(Vec2::new((2.0, 2.0)) * 2.0, Vec2::new((4.0, 4.0)));
}

#[test]
fn test_div() {
  assert_eq!(Vec2::new((0.0, 0.0)) / 1.0, Vec2::new((0.0, 0.0)));
  assert_eq!(Vec2::new((1.0, 0.0)) / 1.0, Vec2::new((1.0, 0.0)));
  assert_eq!(Vec2::new((2.0, 0.0)) / 1.0, Vec2::new((2.0, 0.0)));
  assert_eq!(Vec2::new((2.0, 1.0)) / 1.0, Vec2::new((2.0, 1.0)));
  assert_eq!(Vec2::new((2.0, 2.0)) / 1.0, Vec2::new((2.0, 2.0)));
  assert_eq!(Vec2::new((2.0, 2.0)) / 2.0, Vec2::new((1.0, 1.0)));
  assert_eq!(Vec2::new((2.0, 2.0)) / 4.0, Vec2::new((0.5, 0.5)));
}

#[test]
fn test_set() {
  let mut vec = Vec2::new(());
  vec.set(Vec2::new((0.0, 0.0)));
  assert_eq!(vec, Vec2::new((0.0, 0.0)));
  vec.set(Vec2::new((1.0, 0.0)));
  assert_eq!(vec, Vec2::new((1.0, 0.0)));
  vec.set(Vec2::new((2.0, 0.0)));
  assert_eq!(vec, Vec2::new((2.0, 0.0)));
  vec.set(Vec2::new((2.0, 1.0)));
  assert_eq!(vec, Vec2::new((2.0, 1.0)));
  vec.set(Vec2::new((2.0, 2.0)));
  assert_eq!(vec, Vec2::new((2.0, 2.0)));
}

#[test]
fn test_add_assign() {
  let mut vec = Vec2::new(());
  vec += Vec2::new((0.0, 0.0));
  assert_eq!(vec, Vec2::new((0.0, 0.0)));
  vec += Vec2::new((1.0, 0.0));
  assert_eq!(vec, Vec2::new((1.0, 0.0)));
  vec += Vec2::new((2.0, 0.0));
  assert_eq!(vec, Vec2::new((3.0, 0.0)));
  vec += Vec2::new((2.0, 1.0));
  assert_eq!(vec, Vec2::new((5.0, 1.0)));
  vec += Vec2::new((2.0, 2.0));
  assert_eq!(vec, Vec2::new((7.0, 3.0)));
}

#[test]
fn test_sub_assign() {
  let mut vec = Vec2::new(());
  vec -= Vec2::new((0.0, 0.0));
  assert_eq!(vec, Vec2::new((0.0, 0.0)));
  vec -= Vec2::new((1.0, 0.0));
  assert_eq!(vec, Vec2::new((-1.0, 0.0)));
  vec -= Vec2::new((2.0, 0.0));
  assert_eq!(vec, Vec2::new((-3.0, 0.0)));
  vec -= Vec2::new((2.0, 1.0));
  assert_eq!(vec, Vec2::new((-5.0, -1.0)));
  vec -= Vec2::new((2.0, 2.0));
  assert_eq!(vec, Vec2::new((-7.0, -3.0)));
}

#[test]
fn test_mul_assign() {
  let mut vec = Vec2::new((1.0, 1.0));
  vec *= 1.0;
  assert_eq!(vec, Vec2::new((1.0, 1.0)));
  vec *= 2.0;
  assert_eq!(vec, Vec2::new((2.0, 2.0)));
  vec *= 3.0;
  assert_eq!(vec, Vec2::new((6.0, 6.0)));
}

#[test]
fn test_div_assign() {
  let mut vec = Vec2::new((1.0, 1.0));
  vec /= 1.0;
  assert_eq!(vec, Vec2::new((1.0, 1.0)));
  vec /= 2.0;
  assert_eq!(vec, Vec2::new((0.5, 0.5)));
  vec /= 5.0;
  assert_eq!(vec, Vec2::new((0.1, 0.1)));
}

#[test]
fn test_get_squared_len() {
  assert_eq!(Vec2::new((0.0, 0.0)).get_squared_len(), 0.0);
  assert_eq!(Vec2::new((1.0, 0.0)).get_squared_len(), 1.0);
  assert_eq!(Vec2::new((2.0, 0.0)).get_squared_len(), 4.0);
  assert_eq!(Vec2::new((2.0, 1.0)).get_squared_len(), 5.0);
  assert_eq!(Vec2::new((2.0, 2.0)).get_squared_len(), 8.0);
}

#[test]
fn test_get_len() {
  assert_eq!(Vec2::new((0.0, 0.0)).get_len(), 0.0);
  assert_eq!(Vec2::new((1.0, 0.0)).get_len(), 1.0);
  assert_eq!(Vec2::new((2.0, 0.0)).get_len(), 2.0);
  assert_eq!(Vec2::new((2.0, 1.0)).get_len(), 5.0f64.sqrt());
  assert_eq!(Vec2::new((2.0, 2.0)).get_len(), 8.0f64.sqrt());
}

#[test]
fn test_get_normalized() {
  assert_eq!(
    Vec2::new((1.0, 0.0)).get_normalized(),
    Vec2::new((1.0, 0.0))
  );
  assert_eq!(
    Vec2::new((2.0, 0.0)).get_normalized(),
    Vec2::new((1.0, 0.0))
  );
  assert_eq!(
    Vec2::new((3.0, 0.0)).get_normalized(),
    Vec2::new((1.0, 0.0))
  );
  assert_eq!(
    Vec2::new((3.0, 4.0)).get_normalized(),
    Vec2::new((0.6, 0.8))
  );
  assert_eq!(
    Vec2::new((6.0, 8.0)).get_normalized(),
    Vec2::new((0.6, 0.8))
  );
}

#[test]
fn test_get_x_fliped() {
  assert_eq!(Vec2::new((0.0, 0.0)).get_x_fliped(), Vec2::new((0.0, 0.0)));
  assert_eq!(Vec2::new((1.0, 0.0)).get_x_fliped(), Vec2::new((-1.0, 0.0)));
  assert_eq!(Vec2::new((2.0, 0.0)).get_x_fliped(), Vec2::new((-2.0, 0.0)));
  assert_eq!(Vec2::new((2.0, 1.0)).get_x_fliped(), Vec2::new((-2.0, 1.0)));
  assert_eq!(Vec2::new((2.0, 2.0)).get_x_fliped(), Vec2::new((-2.0, 2.0)));
}

#[test]
fn test_get_y_fliped() {
  assert_eq!(Vec2::new((0.0, 0.0)).get_y_fliped(), Vec2::new((0.0, 0.0)));
  assert_eq!(Vec2::new((1.0, 0.0)).get_y_fliped(), Vec2::new((1.0, 0.0)));
  assert_eq!(Vec2::new((2.0, 0.0)).get_y_fliped(), Vec2::new((2.0, 0.0)));
  assert_eq!(Vec2::new((2.0, 1.0)).get_y_fliped(), Vec2::new((2.0, -1.0)));
  assert_eq!(Vec2::new((2.0, 2.0)).get_y_fliped(), Vec2::new((2.0, -2.0)));
}

#[test]
fn test_get_fliped() {
  assert_eq!(Vec2::new((0.0, 0.0)).get_fliped(), Vec2::new((0.0, 0.0)));
  assert_eq!(Vec2::new((1.0, 0.0)).get_fliped(), Vec2::new((-1.0, 0.0)));
  assert_eq!(Vec2::new((2.0, 0.0)).get_fliped(), Vec2::new((-2.0, 0.0)));
  assert_eq!(Vec2::new((2.0, 1.0)).get_fliped(), Vec2::new((-2.0, -1.0)));
  assert_eq!(Vec2::new((2.0, 2.0)).get_fliped(), Vec2::new((-2.0, -2.0)));
}

#[test]
fn test_display() {
  assert_eq!(format!("{}", Vec2::new((0.0, 0.0))), "0,0");
  assert_eq!(format!("{}", Vec2::new((1.0, 0.0))), "1,0");
  assert_eq!(format!("{}", Vec2::new((2.0, 0.0))), "2,0");
  assert_eq!(format!("{}", Vec2::new((2.0, 1.0))), "2,1");
  assert_eq!(format!("{}", Vec2::new((2.0, 2.0))), "2,2");
}

#[test]
fn test_approx_eq() {
  assert!(Vec2::new((0.0, 0.0)).approx_eq(Vec2::new((0.0, 0.0))));
  assert!(!Vec2::new((1.0, 0.0)).approx_eq(Vec2::new((0.0, 0.0))));
  assert!(!Vec2::new((2.0, 0.0)).approx_eq(Vec2::new((0.0, 0.0))));
  assert!(!Vec2::new((2.0, 1.0)).approx_eq(Vec2::new((0.0, 0.0))));
  assert!(!Vec2::new((2.0, 2.0)).approx_eq(Vec2::new((0.0, 0.0))));
  assert!(!Vec2::new((2.0, 2.0)).approx_eq(Vec2::new((1.0, 0.0))));
  assert!(!Vec2::new((2.0, 2.0)).approx_eq(Vec2::new((2.0, 0.0))));
  assert!(!Vec2::new((2.0, 2.0)).approx_eq(Vec2::new((2.0, 1.0))));
  assert!(Vec2::new((2.0, 2.0)).approx_eq(Vec2::new((2.0, 2.0))));
}
