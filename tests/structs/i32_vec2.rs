use iron_ingot::{F64Vec2, I32Vec2};

#[test]
fn test_from_i32() {
  assert_eq!(I32Vec2::new(0), I32Vec2::new((0, 0)));
  assert_eq!(I32Vec2::new(1), I32Vec2::new((1, 1)));
  assert_eq!(I32Vec2::new(2), I32Vec2::new((2, 2)));
}

#[test]
fn test_with_x() {
  assert_eq!(I32Vec2::new((0, 0)).with_x(0), I32Vec2::new((0, 0)));
  assert_eq!(I32Vec2::new((1, 0)).with_x(0), I32Vec2::new((0, 0)));
  assert_eq!(I32Vec2::new((2, 0)).with_x(0), I32Vec2::new((0, 0)));
  assert_eq!(I32Vec2::new((2, 1)).with_x(0), I32Vec2::new((0, 1)));
  assert_eq!(I32Vec2::new((2, 2)).with_x(0), I32Vec2::new((0, 2)));
  assert_eq!(I32Vec2::new((2, 2)).with_x(1), I32Vec2::new((1, 2)));
  assert_eq!(I32Vec2::new((2, 2)).with_x(2), I32Vec2::new((2, 2)));
}

#[test]
fn test_with_y() {
  assert_eq!(I32Vec2::new((0, 0)).with_y(0), I32Vec2::new((0, 0)));
  assert_eq!(I32Vec2::new((1, 0)).with_y(0), I32Vec2::new((1, 0)));
  assert_eq!(I32Vec2::new((2, 0)).with_y(0), I32Vec2::new((2, 0)));
  assert_eq!(I32Vec2::new((2, 1)).with_y(0), I32Vec2::new((2, 0)));
  assert_eq!(I32Vec2::new((2, 2)).with_y(0), I32Vec2::new((2, 0)));
  assert_eq!(I32Vec2::new((2, 2)).with_y(1), I32Vec2::new((2, 1)));
  assert_eq!(I32Vec2::new((2, 2)).with_y(2), I32Vec2::new((2, 2)));
}

#[test]
fn test_add() {
  assert_eq!(
    I32Vec2::new((0, 0)) + I32Vec2::new((0, 0)),
    I32Vec2::new((0, 0))
  );
  assert_eq!(
    I32Vec2::new((1, 0)) + I32Vec2::new((0, 0)),
    I32Vec2::new((1, 0))
  );
  assert_eq!(
    I32Vec2::new((2, 0)) + I32Vec2::new((0, 0)),
    I32Vec2::new((2, 0))
  );
  assert_eq!(
    I32Vec2::new((2, 1)) + I32Vec2::new((0, 0)),
    I32Vec2::new((2, 1))
  );
  assert_eq!(
    I32Vec2::new((2, 2)) + I32Vec2::new((0, 0)),
    I32Vec2::new((2, 2))
  );
  assert_eq!(
    I32Vec2::new((2, 2)) + I32Vec2::new((1, 0)),
    I32Vec2::new((3, 2))
  );
  assert_eq!(
    I32Vec2::new((2, 2)) + I32Vec2::new((2, 0)),
    I32Vec2::new((4, 2))
  );
  assert_eq!(
    I32Vec2::new((2, 2)) + I32Vec2::new((2, 1)),
    I32Vec2::new((4, 3))
  );
  assert_eq!(
    I32Vec2::new((2, 2)) + I32Vec2::new((2, 2)),
    I32Vec2::new((4, 4))
  );
}

#[test]
fn test_sub() {
  assert_eq!(
    I32Vec2::new((0, 0)) - I32Vec2::new((0, 0)),
    I32Vec2::new((0, 0))
  );
  assert_eq!(
    I32Vec2::new((1, 0)) - I32Vec2::new((0, 0)),
    I32Vec2::new((1, 0))
  );
  assert_eq!(
    I32Vec2::new((2, 0)) - I32Vec2::new((0, 0)),
    I32Vec2::new((2, 0))
  );
  assert_eq!(
    I32Vec2::new((2, 1)) - I32Vec2::new((0, 0)),
    I32Vec2::new((2, 1))
  );
  assert_eq!(
    I32Vec2::new((2, 2)) - I32Vec2::new((0, 0)),
    I32Vec2::new((2, 2))
  );
  assert_eq!(
    I32Vec2::new((2, 2)) - I32Vec2::new((1, 0)),
    I32Vec2::new((1, 2))
  );
  assert_eq!(
    I32Vec2::new((2, 2)) - I32Vec2::new((2, 0)),
    I32Vec2::new((0, 2))
  );
  assert_eq!(
    I32Vec2::new((2, 2)) - I32Vec2::new((2, 1)),
    I32Vec2::new((0, 1))
  );
  assert_eq!(
    I32Vec2::new((2, 2)) - I32Vec2::new((2, 2)),
    I32Vec2::new((0, 0))
  );
}

#[test]
fn test_mul() {
  assert_eq!(I32Vec2::new((0, 0)) * 0, I32Vec2::new((0, 0)));
  assert_eq!(I32Vec2::new((1, 0)) * 0, I32Vec2::new((0, 0)));
  assert_eq!(I32Vec2::new((2, 0)) * 0, I32Vec2::new((0, 0)));
  assert_eq!(I32Vec2::new((2, 1)) * 0, I32Vec2::new((0, 0)));
  assert_eq!(I32Vec2::new((2, 2)) * 0, I32Vec2::new((0, 0)));
  assert_eq!(I32Vec2::new((2, 2)) * 1, I32Vec2::new((2, 2)));
  assert_eq!(I32Vec2::new((2, 2)) * 2, I32Vec2::new((4, 4)));
}

#[test]
fn test_div() {
  assert_eq!(I32Vec2::new((0, 0)) / 1, I32Vec2::new((0, 0)));
  assert_eq!(I32Vec2::new((1, 0)) / 1, I32Vec2::new((1, 0)));
  assert_eq!(I32Vec2::new((2, 0)) / 1, I32Vec2::new((2, 0)));
  assert_eq!(I32Vec2::new((2, 1)) / 1, I32Vec2::new((2, 1)));
  assert_eq!(I32Vec2::new((2, 2)) / 1, I32Vec2::new((2, 2)));
  assert_eq!(I32Vec2::new((2, 2)) / 2, I32Vec2::new((1, 1)));
  assert_eq!(I32Vec2::new((2, 2)) / 4, I32Vec2::new((0, 0)));
}

#[test]
fn test_set() {
  let mut vec = I32Vec2::new(());
  vec.set(I32Vec2::new((0, 0)));
  assert_eq!(vec, I32Vec2::new((0, 0)));
  vec.set(I32Vec2::new((1, 0)));
  assert_eq!(vec, I32Vec2::new((1, 0)));
  vec.set(I32Vec2::new((2, 0)));
  assert_eq!(vec, I32Vec2::new((2, 0)));
  vec.set(I32Vec2::new((2, 1)));
  assert_eq!(vec, I32Vec2::new((2, 1)));
  vec.set(I32Vec2::new((2, 2)));
  assert_eq!(vec, I32Vec2::new((2, 2)));
}

#[test]
fn test_add_assign() {
  let mut vec = I32Vec2::new(());
  vec += I32Vec2::new((0, 0));
  assert_eq!(vec, I32Vec2::new((0, 0)));
  vec += I32Vec2::new((1, 0));
  assert_eq!(vec, I32Vec2::new((1, 0)));
  vec += I32Vec2::new((2, 0));
  assert_eq!(vec, I32Vec2::new((3, 0)));
  vec += I32Vec2::new((2, 1));
  assert_eq!(vec, I32Vec2::new((5, 1)));
  vec += I32Vec2::new((2, 2));
  assert_eq!(vec, I32Vec2::new((7, 3)));
}

#[test]
fn test_sub_assign() {
  let mut vec = I32Vec2::new(());
  vec -= I32Vec2::new((0, 0));
  assert_eq!(vec, I32Vec2::new((0, 0)));
  vec -= I32Vec2::new((1, 0));
  assert_eq!(vec, I32Vec2::new((-1, 0)));
  vec -= I32Vec2::new((2, 0));
  assert_eq!(vec, I32Vec2::new((-3, 0)));
  vec -= I32Vec2::new((2, 1));
  assert_eq!(vec, I32Vec2::new((-5, -1)));
  vec -= I32Vec2::new((2, 2));
  assert_eq!(vec, I32Vec2::new((-7, -3)));
}

#[test]
fn test_mul_assign() {
  let mut vec = I32Vec2::new((1, 1));
  vec *= 1;
  assert_eq!(vec, I32Vec2::new((1, 1)));
  vec *= 2;
  assert_eq!(vec, I32Vec2::new((2, 2)));
  vec *= 3;
  assert_eq!(vec, I32Vec2::new((6, 6)));
}

#[test]
fn test_div_assign() {
  let mut vec = I32Vec2::new((1, 1));
  vec /= 1;
  assert_eq!(vec, I32Vec2::new((1, 1)));
  vec /= 2;
  assert_eq!(vec, I32Vec2::new((0, 0)));
  vec /= 5;
  assert_eq!(vec, I32Vec2::new((0, 0)));
}

#[test]
fn test_get_x_fliped() {
  assert_eq!(I32Vec2::new((0, 0)).get_x_fliped(), I32Vec2::new((0, 0)));
  assert_eq!(I32Vec2::new((1, 0)).get_x_fliped(), I32Vec2::new((-1, 0)));
  assert_eq!(I32Vec2::new((2, 0)).get_x_fliped(), I32Vec2::new((-2, 0)));
  assert_eq!(I32Vec2::new((2, 1)).get_x_fliped(), I32Vec2::new((-2, 1)));
  assert_eq!(I32Vec2::new((2, 2)).get_x_fliped(), I32Vec2::new((-2, 2)));
}

#[test]
fn test_get_y_fliped() {
  assert_eq!(I32Vec2::new((0, 0)).get_y_fliped(), I32Vec2::new((0, 0)));
  assert_eq!(I32Vec2::new((1, 0)).get_y_fliped(), I32Vec2::new((1, 0)));
  assert_eq!(I32Vec2::new((2, 0)).get_y_fliped(), I32Vec2::new((2, 0)));
  assert_eq!(I32Vec2::new((2, 1)).get_y_fliped(), I32Vec2::new((2, -1)));
  assert_eq!(I32Vec2::new((2, 2)).get_y_fliped(), I32Vec2::new((2, -2)));
}

#[test]
fn test_get_fliped() {
  assert_eq!(I32Vec2::new((0, 0)).get_fliped(), I32Vec2::new((0, 0)));
  assert_eq!(I32Vec2::new((1, 0)).get_fliped(), I32Vec2::new((-1, 0)));
  assert_eq!(I32Vec2::new((2, 0)).get_fliped(), I32Vec2::new((-2, 0)));
  assert_eq!(I32Vec2::new((2, 1)).get_fliped(), I32Vec2::new((-2, -1)));
  assert_eq!(I32Vec2::new((2, 2)).get_fliped(), I32Vec2::new((-2, -2)));
}

#[test]
fn test_display() {
  assert_eq!(format!("{}", I32Vec2::new((0, 0))), "0,0");
  assert_eq!(format!("{}", I32Vec2::new((1, 0))), "1,0");
  assert_eq!(format!("{}", I32Vec2::new((2, 0))), "2,0");
  assert_eq!(format!("{}", I32Vec2::new((2, 1))), "2,1");
  assert_eq!(format!("{}", I32Vec2::new((2, 2))), "2,2");
}

#[test]
fn test_into_f64_vec2() {
  assert_eq!(
    F64Vec2::from(I32Vec2::new((0, 0))),
    F64Vec2::new((0.0, 0.0))
  );
  assert_eq!(
    F64Vec2::from(I32Vec2::new((1, 0))),
    F64Vec2::new((1.0, 0.0))
  );
  assert_eq!(
    F64Vec2::from(I32Vec2::new((2, 0))),
    F64Vec2::new((2.0, 0.0))
  );
  assert_eq!(
    F64Vec2::from(I32Vec2::new((2, 1))),
    F64Vec2::new((2.0, 1.0))
  );
  assert_eq!(
    F64Vec2::from(I32Vec2::new((2, 2))),
    F64Vec2::new((2.0, 2.0))
  );
}
