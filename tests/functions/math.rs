use iron::{math::*, Seq, Vec2, Vec3, Vec4};

#[test]
fn test_approx_eq() {
  assert!(0.000001f64.approx_eq(0.000001f64));
  assert!(!0.000001f64.approx_eq(0.0f64));
  assert!(!0.0f64.approx_eq(0.000001f64));
  assert!(!0.000001f64.approx_eq(0.0000009f64));
  assert!(!0.0000009f64.approx_eq(0.000001f64));
  assert!(!0.000001f64.approx_eq(0.0000011f64));
  assert!(!0.0000011f64.approx_eq(0.000001f64));
  assert!(0.00001f64.approx_eq(0.00001f64));
  assert!(!0.00001f64.approx_eq(0.0f64));
  assert!(!0.0f64.approx_eq(0.00001f64));
  assert!(!0.00001f64.approx_eq(0.0000099f64));
  assert!(!0.0000099f64.approx_eq(0.00001f64));
  assert!(!0.00001f64.approx_eq(0.0000101f64));
  assert!(!0.0000101f64.approx_eq(0.00001f64));
  assert!(0.0001f64.approx_eq(0.0001f64));
  assert!(!0.0001f64.approx_eq(0.0f64));
  assert!(!0.0f64.approx_eq(0.0001f64));
  assert!(!0.0001f64.approx_eq(0.0000999f64));
  assert!(!0.0000999f64.approx_eq(0.0001f64));
  assert!(!0.0001f64.approx_eq(0.0001001f64));
  assert!(!0.0001001f64.approx_eq(0.0001f64));
  assert!(0.001f64.approx_eq(0.001f64));
  assert!(!0.001f64.approx_eq(0.0f64));
  assert!(!0.0f64.approx_eq(0.001f64));
  assert!(!0.001f64.approx_eq(0.0009999f64));
  assert!(!0.0009999f64.approx_eq(0.001f64));
  assert!(!0.001f64.approx_eq(0.0010001f64));
  assert!(!0.0010001f64.approx_eq(0.001f64));
  assert!(0.01f64.approx_eq(0.01f64));
  assert!(!0.01f64.approx_eq(0.0f64));
  assert!(!0.0f64.approx_eq(0.01f64));
  assert!(!0.01f64.approx_eq(0.0099999f64));
  assert!(!0.0099999f64.approx_eq(0.01f64));
  assert!(!0.01f64.approx_eq(0.0100001f64));
  assert!(!0.0100001f64.approx_eq(0.01f64));
  assert!(0.1f64.approx_eq(0.1f64));
  assert!(!0.1f64.approx_eq(0.0f64));
  assert!(!0.0f64.approx_eq(0.1f64));
  assert!(!0.1f64.approx_eq(0.0999999f64));
  assert!(!0.0999999f64.approx_eq(0.1f64));
  assert!(!0.1f64.approx_eq(0.1000001f64));
  assert!(!0.1000001f64.approx_eq(0.1f64));
  assert!(1.0f64.approx_eq(1.0f64));
  assert!(!1.0f64.approx_eq(0.0f64));
  assert!(!0.0f64.approx_eq(1.0f64));
  assert!(!1.0f64.approx_eq(0.9999999f64));
  assert!(!0.9999999f64.approx_eq(1.0f64));
  assert!(!1.0f64.approx_eq(1.0000001f64));
  assert!(!1.0000001f64.approx_eq(1.0f64));
  assert!(10.0f64.approx_eq(10.0f64));
  assert!(!10.0f64.approx_eq(0.0f64));
  assert!(!0.0f64.approx_eq(10.0f64));
  assert!(!10.0f64.approx_eq(9.9999999f64));
  assert!(!9.9999999f64.approx_eq(10.0f64));
  assert!(!10.0f64.approx_eq(10.000001f64));
  assert!(!10.000001f64.approx_eq(10.0f64));
  assert!(100.0f64.approx_eq(100.0f64));
  assert!(!100.0f64.approx_eq(0.0f64));
  assert!(!0.0f64.approx_eq(100.0f64));
  assert!(!100.0f64.approx_eq(99.9999999f64));
  assert!(!99.9999999f64.approx_eq(100.0f64));
  assert!(!100.0f64.approx_eq(100.000001f64));
  assert!(!100.000001f64.approx_eq(100.0f64));
  assert!(1000.0f64.approx_eq(1000.0f64));
  assert!(!1000.0f64.approx_eq(0.0f64));
  assert!(!0.0f64.approx_eq(1000.0f64));
  assert!(!1000.0f64.approx_eq(999.999999f64));
  assert!(!999.999999f64.approx_eq(1000.0f64));
  assert!(!1000.0f64.approx_eq(1000.00001f64));
  assert!(!1000.00001f64.approx_eq(1000.0f64));
  assert!(10000.0f64.approx_eq(10000.0f64));
  assert!(!10000.0f64.approx_eq(0.0f64));
  assert!(!0.0f64.approx_eq(10000.0f64));
  assert!(!10000.0f64.approx_eq(9999.99999f64));
  assert!(!9999.99999f64.approx_eq(10000.0f64));
  assert!(!10000.0f64.approx_eq(10000.0001f64));
  assert!(!10000.0001f64.approx_eq(10000.0f64));
  assert!(100000.0f64.approx_eq(100000.0f64));
  assert!(!100000.0f64.approx_eq(0.0f64));
  assert!(!0.0f64.approx_eq(100000.0f64));
  assert!(!100000.0f64.approx_eq(99999.9999f64));
  assert!(!99999.9999f64.approx_eq(100000.0f64));
  assert!(!100000.0f64.approx_eq(100000.001f64));
  assert!(!100000.001f64.approx_eq(100000.0f64));
  assert!(1000000.0f64.approx_eq(1000000.0f64));
  assert!(!1000000.0f64.approx_eq(0.0f64));
  assert!(!0.0f64.approx_eq(1000000.0f64));
  assert!(!1000000.0f64.approx_eq(999999.999f64));
  assert!(!999999.999f64.approx_eq(1000000.0f64));
  assert!(!1000000.0f64.approx_eq(1000000.01f64));
  assert!(!1000000.01f64.approx_eq(1000000.0f64));
  assert!(!1000000.0f64.approx_eq(999999.9999999f64));
  assert!(!999999.9999999f64.approx_eq(1000000.0f64));
  assert!(!1000000.0f64.approx_eq(1000000.0000001f64));
  assert!(!1000000.0000001f64.approx_eq(1000000.0f64));
}

#[test]
fn test_is_odd() {
  assert!(!0.is_odd());
  assert!(1.is_odd());
  assert!(!2.is_odd());
  assert!(3.is_odd());
}

#[test]
fn test_is_even() {
  assert!(0.is_even());
  assert!(!1.is_even());
  assert!(2.is_even());
  assert!(!3.is_even());
}

#[test]
fn test_is_prime() {
  assert!(!0.is_prime());
  assert!(!1.is_prime());
  assert!(2.is_prime());
  assert!(3.is_prime());
  assert!(!4.is_prime());
  assert!(5.is_prime());
  assert!(!6.is_prime());
  assert!(7.is_prime());
  assert!(!8.is_prime());
  assert!(!9.is_prime());
  assert!(!10.is_prime());
  assert!(11.is_prime());
  assert!(!12.is_prime());
  assert!(13.is_prime());
  assert!(!14.is_prime());
  assert!(!15.is_prime());
  assert!(!16.is_prime());
  assert!(17.is_prime());
  assert!(!18.is_prime());
  assert!(19.is_prime());
  assert!(!20.is_prime());
  assert!(!21.is_prime());
  assert!(!22.is_prime());
  assert!(23.is_prime());
}

#[test]
fn test_is_pow_of_2() {
  assert!(1.is_pow_of_2());
  assert!(2.is_pow_of_2());
  assert!(!3.is_pow_of_2());
  assert!(4.is_pow_of_2());
  assert!(!5.is_pow_of_2());
}

#[test]
fn test_lerp() {
  assert_eq!(lerp(0.0, 0.0, 0.0), 0.0);
  assert_eq!(lerp(1.0, 0.0, 0.0), 0.0);
  assert_eq!(lerp(2.0, 0.0, 0.0), 0.0);
  assert_eq!(lerp(2.0, 1.0, 0.0), -1.0);
  assert_eq!(lerp(2.0, 2.0, 0.0), -2.0);
  assert_eq!(lerp(2.0, 2.0, 1.0), 0.0);
  assert_eq!(lerp(2.0, 2.0, 2.0), 2.0);
}

#[test]
fn test_lerp_vec2() {
  assert_eq!(
    lerp_vec2(0.0, Vec2::new((0.0, 0.0)), Vec2::new((0.0, 0.0))),
    Vec2::new((0.0, 0.0))
  );
  assert_eq!(
    lerp_vec2(1.0, Vec2::new((0.0, 0.0)), Vec2::new((0.0, 0.0))),
    Vec2::new((0.0, 0.0))
  );
  assert_eq!(
    lerp_vec2(2.0, Vec2::new((0.0, 0.0)), Vec2::new((0.0, 0.0))),
    Vec2::new((0.0, 0.0))
  );
  assert_eq!(
    lerp_vec2(2.0, Vec2::new((1.0, 0.0)), Vec2::new((0.0, 0.0))),
    Vec2::new((-1.0, 0.0))
  );
  assert_eq!(
    lerp_vec2(2.0, Vec2::new((2.0, 0.0)), Vec2::new((0.0, 0.0))),
    Vec2::new((-2.0, 0.0))
  );
  assert_eq!(
    lerp_vec2(2.0, Vec2::new((2.0, 0.0)), Vec2::new((1.0, 0.0))),
    Vec2::new((0.0, 0.0))
  );
  assert_eq!(
    lerp_vec2(2.0, Vec2::new((2.0, 0.0)), Vec2::new((2.0, 0.0))),
    Vec2::new((2.0, 0.0))
  );
  assert_eq!(
    lerp_vec2(2.0, Vec2::new((2.0, 1.0)), Vec2::new((2.0, 0.0))),
    Vec2::new((2.0, -1.0))
  );
  assert_eq!(
    lerp_vec2(2.0, Vec2::new((2.0, 2.0)), Vec2::new((2.0, 0.0))),
    Vec2::new((2.0, -2.0))
  );
  assert_eq!(
    lerp_vec2(2.0, Vec2::new((2.0, 2.0)), Vec2::new((2.0, 1.0))),
    Vec2::new((2.0, 0.0))
  );
  assert_eq!(
    lerp_vec2(2.0, Vec2::new((2.0, 2.0)), Vec2::new((2.0, 2.0))),
    Vec2::new((2.0, 2.0))
  );
}

#[test]
fn test_lerp_vec3() {
  assert_eq!(
    lerp_vec3(0.0, Vec3::new((0.0, 0.0, 0.0)), Vec3::new((0.0, 0.0, 0.0))),
    Vec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec3(1.0, Vec3::new((0.0, 0.0, 0.0)), Vec3::new((0.0, 0.0, 0.0))),
    Vec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec3(2.0, Vec3::new((0.0, 0.0, 0.0)), Vec3::new((0.0, 0.0, 0.0))),
    Vec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec3(2.0, Vec3::new((1.0, 0.0, 0.0)), Vec3::new((0.0, 0.0, 0.0))),
    Vec3::new((-1.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec3(2.0, Vec3::new((2.0, 0.0, 0.0)), Vec3::new((0.0, 0.0, 0.0))),
    Vec3::new((-2.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec3(2.0, Vec3::new((2.0, 0.0, 0.0)), Vec3::new((1.0, 0.0, 0.0))),
    Vec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec3(2.0, Vec3::new((2.0, 0.0, 0.0)), Vec3::new((2.0, 0.0, 0.0))),
    Vec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec3(2.0, Vec3::new((2.0, 1.0, 0.0)), Vec3::new((2.0, 0.0, 0.0))),
    Vec3::new((2.0, -1.0, 0.0))
  );
  assert_eq!(
    lerp_vec3(2.0, Vec3::new((2.0, 2.0, 0.0)), Vec3::new((2.0, 0.0, 0.0))),
    Vec3::new((2.0, -2.0, 0.0))
  );
  assert_eq!(
    lerp_vec3(2.0, Vec3::new((2.0, 2.0, 0.0)), Vec3::new((2.0, 1.0, 0.0))),
    Vec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec3(2.0, Vec3::new((2.0, 2.0, 0.0)), Vec3::new((2.0, 2.0, 0.0))),
    Vec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    lerp_vec3(2.0, Vec3::new((2.0, 2.0, 1.0)), Vec3::new((2.0, 2.0, 0.0))),
    Vec3::new((2.0, 2.0, -1.0))
  );
  assert_eq!(
    lerp_vec3(2.0, Vec3::new((2.0, 2.0, 2.0)), Vec3::new((2.0, 2.0, 0.0))),
    Vec3::new((2.0, 2.0, -2.0))
  );
  assert_eq!(
    lerp_vec3(2.0, Vec3::new((2.0, 2.0, 2.0)), Vec3::new((2.0, 2.0, 1.0))),
    Vec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    lerp_vec3(2.0, Vec3::new((2.0, 2.0, 2.0)), Vec3::new((2.0, 2.0, 2.0))),
    Vec3::new((2.0, 2.0, 2.0))
  );
}

#[test]
fn test_lerp_vec4() {
  assert_eq!(
    lerp_vec4(
      0.0,
      Vec4::new((0.0, 0.0, 0.0, 0.0)),
      Vec4::new((0.0, 0.0, 0.0, 0.0))
    ),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec4(
      1.0,
      Vec4::new((0.0, 0.0, 0.0, 0.0)),
      Vec4::new((0.0, 0.0, 0.0, 0.0))
    ),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec4(
      2.0,
      Vec4::new((0.0, 0.0, 0.0, 0.0)),
      Vec4::new((0.0, 0.0, 0.0, 0.0))
    ),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec4(
      2.0,
      Vec4::new((1.0, 0.0, 0.0, 0.0)),
      Vec4::new((0.0, 0.0, 0.0, 0.0))
    ),
    Vec4::new((-1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec4(
      2.0,
      Vec4::new((2.0, 0.0, 0.0, 0.0)),
      Vec4::new((0.0, 0.0, 0.0, 0.0))
    ),
    Vec4::new((-2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec4(
      2.0,
      Vec4::new((2.0, 0.0, 0.0, 0.0)),
      Vec4::new((1.0, 0.0, 0.0, 0.0))
    ),
    Vec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec4(
      2.0,
      Vec4::new((2.0, 0.0, 0.0, 0.0)),
      Vec4::new((2.0, 0.0, 0.0, 0.0))
    ),
    Vec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec4(
      2.0,
      Vec4::new((2.0, 1.0, 0.0, 0.0)),
      Vec4::new((2.0, 0.0, 0.0, 0.0))
    ),
    Vec4::new((2.0, -1.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec4(
      2.0,
      Vec4::new((2.0, 2.0, 0.0, 0.0)),
      Vec4::new((2.0, 0.0, 0.0, 0.0))
    ),
    Vec4::new((2.0, -2.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec4(
      2.0,
      Vec4::new((2.0, 2.0, 0.0, 0.0)),
      Vec4::new((2.0, 1.0, 0.0, 0.0))
    ),
    Vec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec4(
      2.0,
      Vec4::new((2.0, 2.0, 0.0, 0.0)),
      Vec4::new((2.0, 2.0, 0.0, 0.0))
    ),
    Vec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec4(
      2.0,
      Vec4::new((2.0, 2.0, 1.0, 0.0)),
      Vec4::new((2.0, 2.0, 0.0, 0.0))
    ),
    Vec4::new((2.0, 2.0, -1.0, 0.0))
  );
  assert_eq!(
    lerp_vec4(
      2.0,
      Vec4::new((2.0, 2.0, 2.0, 0.0)),
      Vec4::new((2.0, 2.0, 0.0, 0.0))
    ),
    Vec4::new((2.0, 2.0, -2.0, 0.0))
  );
  assert_eq!(
    lerp_vec4(
      2.0,
      Vec4::new((2.0, 2.0, 2.0, 0.0)),
      Vec4::new((2.0, 2.0, 1.0, 0.0))
    ),
    Vec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_vec4(
      2.0,
      Vec4::new((2.0, 2.0, 2.0, 0.0)),
      Vec4::new((2.0, 2.0, 2.0, 0.0))
    ),
    Vec4::new((2.0, 2.0, 2.0, 0.0))
  );
  assert_eq!(
    lerp_vec4(
      2.0,
      Vec4::new((2.0, 2.0, 2.0, 1.0)),
      Vec4::new((2.0, 2.0, 2.0, 0.0))
    ),
    Vec4::new((2.0, 2.0, 2.0, -1.0))
  );
  assert_eq!(
    lerp_vec4(
      2.0,
      Vec4::new((2.0, 2.0, 2.0, 2.0)),
      Vec4::new((2.0, 2.0, 2.0, 0.0))
    ),
    Vec4::new((2.0, 2.0, 2.0, -2.0))
  );
  assert_eq!(
    lerp_vec4(
      2.0,
      Vec4::new((2.0, 2.0, 2.0, 2.0)),
      Vec4::new((2.0, 2.0, 2.0, 1.0))
    ),
    Vec4::new((2.0, 2.0, 2.0, 0.0))
  );
  assert_eq!(
    lerp_vec4(
      2.0,
      Vec4::new((2.0, 2.0, 2.0, 2.0)),
      Vec4::new((2.0, 2.0, 2.0, 2.0))
    ),
    Vec4::new((2.0, 2.0, 2.0, 2.0))
  );
}

#[test]
fn test_linear_map() {
  assert_eq!(
    linear_map(0.0, Seq::new((0.0, 1.0)), Seq::new((0.0, 1.0))),
    0.0
  );
  assert_eq!(
    linear_map(1.0, Seq::new((0.0, 1.0)), Seq::new((0.0, 1.0))),
    1.0
  );
  assert_eq!(
    linear_map(2.0, Seq::new((0.0, 1.0)), Seq::new((0.0, 1.0))),
    2.0
  );
  assert_eq!(
    linear_map(2.0, Seq::new((0.0, 2.0)), Seq::new((0.0, 1.0))),
    1.0
  );
  assert_eq!(
    linear_map(2.0, Seq::new((0.0, 4.0)), Seq::new((0.0, 1.0))),
    0.5
  );
  assert_eq!(
    linear_map(2.0, Seq::new((2.0, 4.0)), Seq::new((0.0, 1.0))),
    0.0
  );
  assert_eq!(
    linear_map(2.0, Seq::new((3.0, 4.0)), Seq::new((0.0, 1.0))),
    -1.0
  );
  assert_eq!(
    linear_map(2.0, Seq::new((3.0, 4.0)), Seq::new((0.0, 2.0))),
    -2.0
  );
  assert_eq!(
    linear_map(2.0, Seq::new((3.0, 4.0)), Seq::new((0.0, 3.0))),
    -3.0
  );
  assert_eq!(
    linear_map(2.0, Seq::new((3.0, 4.0)), Seq::new((1.0, 3.0))),
    -1.0
  );
  assert_eq!(
    linear_map(2.0, Seq::new((3.0, 4.0)), Seq::new((2.0, 3.0))),
    1.0
  );
}

#[test]
fn test_linear_map_vec2() {
  assert_eq!(
    linear_map_vec2(
      Vec2::new((0.0, 0.0)),
      Vec2::new((0.0, 0.0)),
      Vec2::new((1.0, 1.0)),
      Vec2::new((0.0, 0.0)),
      Vec2::new((1.0, 1.0))
    ),
    Vec2::new((0.0, 0.0))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((1.0, 0.0)),
      Vec2::new((0.0, 0.0)),
      Vec2::new((1.0, 1.0)),
      Vec2::new((0.0, 0.0)),
      Vec2::new((1.0, 1.0))
    ),
    Vec2::new((1.0, 0.0))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((2.0, 0.0)),
      Vec2::new((0.0, 0.0)),
      Vec2::new((1.0, 1.0)),
      Vec2::new((0.0, 0.0)),
      Vec2::new((1.0, 1.0))
    ),
    Vec2::new((2.0, 0.0))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((2.0, 1.0)),
      Vec2::new((0.0, 0.0)),
      Vec2::new((1.0, 1.0)),
      Vec2::new((0.0, 0.0)),
      Vec2::new((1.0, 1.0))
    ),
    Vec2::new((2.0, 1.0))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((2.0, 2.0)),
      Vec2::new((0.0, 0.0)),
      Vec2::new((1.0, 1.0)),
      Vec2::new((0.0, 0.0)),
      Vec2::new((1.0, 1.0))
    ),
    Vec2::new((2.0, 2.0))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((2.0, 2.0)),
      Vec2::new((1.0, 0.0)),
      Vec2::new((1.0, 1.0)),
      Vec2::new((0.0, 0.0)),
      Vec2::new((1.0, 1.0))
    ),
    Vec2::new((1.0, 2.0))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((2.0, 2.0)),
      Vec2::new((2.0, 0.0)),
      Vec2::new((1.0, 1.0)),
      Vec2::new((0.0, 0.0)),
      Vec2::new((1.0, 1.0))
    ),
    Vec2::new((0.0, 2.0))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((2.0, 2.0)),
      Vec2::new((2.0, 1.0)),
      Vec2::new((1.0, 1.0)),
      Vec2::new((0.0, 0.0)),
      Vec2::new((1.0, 1.0))
    ),
    Vec2::new((0.0, 1.0))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((2.0, 2.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((1.0, 1.0)),
      Vec2::new((0.0, 0.0)),
      Vec2::new((1.0, 1.0))
    ),
    Vec2::new((0.0, 0.0))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((2.0, 2.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((1.0, 1.0)),
      Vec2::new((1.0, 0.0)),
      Vec2::new((1.0, 1.0))
    ),
    Vec2::new((1.0, 0.0))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((2.0, 2.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((1.0, 1.0)),
      Vec2::new((2.0, 0.0)),
      Vec2::new((1.0, 1.0))
    ),
    Vec2::new((2.0, 0.0))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((2.0, 2.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((1.0, 1.0)),
      Vec2::new((2.0, 1.0)),
      Vec2::new((1.0, 1.0))
    ),
    Vec2::new((2.0, 1.0))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((2.0, 2.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((1.0, 1.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((1.0, 1.0))
    ),
    Vec2::new((2.0, 2.0))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((3.0, 3.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((1.0, 1.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((1.0, 1.0))
    ),
    Vec2::new((3.0, 3.0))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((3.0, 3.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((2.0, 1.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((1.0, 1.0))
    ),
    Vec2::new((2.5, 3.0))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((3.0, 3.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((4.0, 1.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((1.0, 1.0))
    ),
    Vec2::new((2.25, 3.0))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((3.0, 3.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((4.0, 2.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((1.0, 1.0))
    ),
    Vec2::new((2.25, 2.5))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((3.0, 3.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((4.0, 4.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((1.0, 1.0))
    ),
    Vec2::new((2.25, 2.25))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((3.0, 3.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((4.0, 4.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((2.0, 1.0))
    ),
    Vec2::new((2.5, 2.25))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((3.0, 3.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((4.0, 4.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((4.0, 1.0))
    ),
    Vec2::new((3.0, 2.25))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((3.0, 3.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((4.0, 4.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((4.0, 2.0))
    ),
    Vec2::new((3.0, 2.5))
  );
  assert_eq!(
    linear_map_vec2(
      Vec2::new((3.0, 3.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((4.0, 4.0)),
      Vec2::new((2.0, 2.0)),
      Vec2::new((4.0, 4.0))
    ),
    Vec2::new((3.0, 3.0))
  );
}

#[test]
fn test_min_pos() {
  assert_eq!([].min_pos(), None);
  assert_eq!([-2.0].min_pos(), None);
  assert_eq!([-1.0].min_pos(), None);
  assert_eq!([0.0].min_pos(), Some((0, 0.0)));
  assert_eq!([1.0].min_pos(), Some((0, 1.0)));
  assert_eq!([2.0].min_pos(), Some((0, 2.0)));
  assert_eq!([2.0, -2.0].min_pos(), Some((0, 2.0)));
  assert_eq!([2.0, -1.0].min_pos(), Some((0, 2.0)));
  assert_eq!([2.0, 0.0].min_pos(), Some((1, 0.0)));
  assert_eq!([2.0, 1.0].min_pos(), Some((1, 1.0)));
  assert_eq!([2.0, 2.0].min_pos(), Some((0, 2.0)));
  assert_eq!([2.0, 2.0, -2.0].min_pos(), Some((0, 2.0)));
  assert_eq!([2.0, 2.0, -1.0].min_pos(), Some((0, 2.0)));
  assert_eq!([2.0, 2.0, 0.0].min_pos(), Some((2, 0.0)));
  assert_eq!([2.0, 2.0, 1.0].min_pos(), Some((2, 1.0)));
  assert_eq!([2.0, 2.0, 2.0].min_pos(), Some((0, 2.0)));
}

#[test]
fn test_max_pos() {
  assert_eq!([].max_pos(), None);
  assert_eq!([-2.0].max_pos(), None);
  assert_eq!([-1.0].max_pos(), None);
  assert_eq!([0.0].max_pos(), Some((0, 0.0)));
  assert_eq!([1.0].max_pos(), Some((0, 1.0)));
  assert_eq!([2.0].max_pos(), Some((0, 2.0)));
  assert_eq!([0.0, -2.0].max_pos(), Some((0, 0.0)));
  assert_eq!([0.0, -1.0].max_pos(), Some((0, 0.0)));
  assert_eq!([0.0, 0.0].max_pos(), Some((0, 0.0)));
  assert_eq!([0.0, 1.0].max_pos(), Some((1, 1.0)));
  assert_eq!([0.0, 2.0].max_pos(), Some((1, 2.0)));
  assert_eq!([0.0, 0.0, -2.0].max_pos(), Some((0, 0.0)));
  assert_eq!([0.0, 0.0, -1.0].max_pos(), Some((0, 0.0)));
  assert_eq!([0.0, 0.0, 0.0].max_pos(), Some((0, 0.0)));
  assert_eq!([0.0, 0.0, 1.0].max_pos(), Some((2, 1.0)));
  assert_eq!([0.0, 0.0, 2.0].max_pos(), Some((2, 2.0)));
}

#[test]
fn test_min_neg() {
  assert_eq!([].min_neg(), None);
  assert_eq!([-2.0].min_neg(), Some((0, -2.0)));
  assert_eq!([-1.0].min_neg(), Some((0, -1.0)));
  assert_eq!([0.0].min_neg(), None);
  assert_eq!([1.0].min_neg(), None);
  assert_eq!([2.0].min_neg(), None);
  assert_eq!([-1.0, -2.0].min_neg(), Some((1, -2.0)));
  assert_eq!([-1.0, -1.0].min_neg(), Some((0, -1.0)));
  assert_eq!([-1.0, 0.0].min_neg(), Some((0, -1.0)));
  assert_eq!([-1.0, 1.0].min_neg(), Some((0, -1.0)));
  assert_eq!([-1.0, 2.0].min_neg(), Some((0, -1.0)));
  assert_eq!([-1.0, -1.0, -2.0].min_neg(), Some((2, -2.0)));
  assert_eq!([-1.0, -1.0, -1.0].min_neg(), Some((0, -1.0)));
  assert_eq!([-1.0, -1.0, 0.0].min_neg(), Some((0, -1.0)));
  assert_eq!([-1.0, -1.0, 1.0].min_neg(), Some((0, -1.0)));
  assert_eq!([-1.0, -1.0, 2.0].min_neg(), Some((0, -1.0)));
}

#[test]
fn test_max_neg() {
  assert_eq!([].max_neg(), None);
  assert_eq!([-2.0].max_neg(), Some((0, -2.0)));
  assert_eq!([-1.0].max_neg(), Some((0, -1.0)));
  assert_eq!([0.0].max_neg(), None);
  assert_eq!([1.0].max_neg(), None);
  assert_eq!([2.0].max_neg(), None);
  assert_eq!([-3.0, -2.0].max_neg(), Some((1, -2.0)));
  assert_eq!([-3.0, -1.0].max_neg(), Some((1, -1.0)));
  assert_eq!([-3.0, 0.0].max_neg(), Some((0, -3.0)));
  assert_eq!([-3.0, 1.0].max_neg(), Some((0, -3.0)));
  assert_eq!([-3.0, 2.0].max_neg(), Some((0, -3.0)));
  assert_eq!([-3.0, -3.0, -2.0].max_neg(), Some((2, -2.0)));
  assert_eq!([-3.0, -3.0, -1.0].max_neg(), Some((2, -1.0)));
  assert_eq!([-3.0, -3.0, 0.0].max_neg(), Some((0, -3.0)));
  assert_eq!([-3.0, -3.0, 1.0].max_neg(), Some((0, -3.0)));
  assert_eq!([-3.0, -3.0, 2.0].max_neg(), Some((0, -3.0)));
}
