use iron_ingot::{math::*, DSeq, DVec2, DVec3, DVec4, FVec2, FVec3, FVec4};

#[test]
fn test_approx_eq_f32() {
  assert!(0.000001f32.approx_eq(0.000001f32));
  assert!(!0.000001f32.approx_eq(0.0f32));
  assert!(!0.0f32.approx_eq(0.000001f32));
  assert!(!0.000001f32.approx_eq(0.0000009f32));
  assert!(!0.0000009f32.approx_eq(0.000001f32));
  assert!(!0.000001f32.approx_eq(0.0000011f32));
  assert!(!0.0000011f32.approx_eq(0.000001f32));
  assert!(0.00001f32.approx_eq(0.00001f32));
  assert!(!0.00001f32.approx_eq(0.0f32));
  assert!(!0.0f32.approx_eq(0.00001f32));
  assert!(!0.00001f32.approx_eq(0.0000099f32));
  assert!(!0.0000099f32.approx_eq(0.00001f32));
  assert!(!0.00001f32.approx_eq(0.0000101f32));
  assert!(!0.0000101f32.approx_eq(0.00001f32));
  assert!(0.0001f32.approx_eq(0.0001f32));
  assert!(!0.0001f32.approx_eq(0.0f32));
  assert!(!0.0f32.approx_eq(0.0001f32));
  assert!(!0.0001f32.approx_eq(0.0000999f32));
  assert!(!0.0000999f32.approx_eq(0.0001f32));
  assert!(!0.0001f32.approx_eq(0.0001001f32));
  assert!(!0.0001001f32.approx_eq(0.0001f32));
  assert!(0.001f32.approx_eq(0.001f32));
  assert!(!0.001f32.approx_eq(0.0f32));
  assert!(!0.0f32.approx_eq(0.001f32));
  assert!(!0.001f32.approx_eq(0.0009999f32));
  assert!(!0.0009999f32.approx_eq(0.001f32));
  assert!(!0.001f32.approx_eq(0.0010001f32));
  assert!(!0.0010001f32.approx_eq(0.001f32));
  assert!(0.01f32.approx_eq(0.01f32));
  assert!(!0.01f32.approx_eq(0.0f32));
  assert!(!0.0f32.approx_eq(0.01f32));
  assert!(!0.01f32.approx_eq(0.0099999f32));
  assert!(!0.0099999f32.approx_eq(0.01f32));
  assert!(!0.01f32.approx_eq(0.0100001f32));
  assert!(!0.0100001f32.approx_eq(0.01f32));
  assert!(0.1f32.approx_eq(0.1f32));
  assert!(!0.1f32.approx_eq(0.0f32));
  assert!(!0.0f32.approx_eq(0.1f32));
  assert!(!0.1f32.approx_eq(0.0999999f32));
  assert!(!0.0999999f32.approx_eq(0.1f32));
  assert!(!0.1f32.approx_eq(0.1000001f32));
  assert!(!0.1000001f32.approx_eq(0.1f32));
  assert!(1.0f32.approx_eq(1.0f32));
  assert!(!1.0f32.approx_eq(0.0f32));
  assert!(!0.0f32.approx_eq(1.0f32));
  assert!(1.0f32.approx_eq(0.9999999f32));
  assert!(0.9999999f32.approx_eq(1.0f32));
  assert!(1.0f32.approx_eq(1.0000001f32));
  assert!(1.0000001f32.approx_eq(1.0f32));
}

#[test]
fn test_approx_eq_f64() {
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
fn test_lerp_f32() {
  assert_eq!(lerp_f32(0.0, 0.0, 0.0), 0.0);
  assert_eq!(lerp_f32(1.0, 0.0, 0.0), 0.0);
  assert_eq!(lerp_f32(2.0, 0.0, 0.0), 0.0);
  assert_eq!(lerp_f32(2.0, 1.0, 0.0), -1.0);
  assert_eq!(lerp_f32(2.0, 2.0, 0.0), -2.0);
  assert_eq!(lerp_f32(2.0, 2.0, 1.0), 0.0);
  assert_eq!(lerp_f32(2.0, 2.0, 2.0), 2.0);
}

#[test]
fn test_lerp_f64() {
  assert_eq!(lerp_f64(0.0, 0.0, 0.0), 0.0);
  assert_eq!(lerp_f64(1.0, 0.0, 0.0), 0.0);
  assert_eq!(lerp_f64(2.0, 0.0, 0.0), 0.0);
  assert_eq!(lerp_f64(2.0, 1.0, 0.0), -1.0);
  assert_eq!(lerp_f64(2.0, 2.0, 0.0), -2.0);
  assert_eq!(lerp_f64(2.0, 2.0, 1.0), 0.0);
  assert_eq!(lerp_f64(2.0, 2.0, 2.0), 2.0);
}

#[test]
fn test_lerp_fvec2() {
  assert_eq!(
    lerp_fvec2(0.0, FVec2::new((0.0, 0.0)), FVec2::new((0.0, 0.0))),
    FVec2::new((0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec2(1.0, FVec2::new((0.0, 0.0)), FVec2::new((0.0, 0.0))),
    FVec2::new((0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec2(2.0, FVec2::new((0.0, 0.0)), FVec2::new((0.0, 0.0))),
    FVec2::new((0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec2(2.0, FVec2::new((1.0, 0.0)), FVec2::new((0.0, 0.0))),
    FVec2::new((-1.0, 0.0))
  );
  assert_eq!(
    lerp_fvec2(2.0, FVec2::new((2.0, 0.0)), FVec2::new((0.0, 0.0))),
    FVec2::new((-2.0, 0.0))
  );
  assert_eq!(
    lerp_fvec2(2.0, FVec2::new((2.0, 0.0)), FVec2::new((1.0, 0.0))),
    FVec2::new((0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec2(2.0, FVec2::new((2.0, 0.0)), FVec2::new((2.0, 0.0))),
    FVec2::new((2.0, 0.0))
  );
  assert_eq!(
    lerp_fvec2(2.0, FVec2::new((2.0, 1.0)), FVec2::new((2.0, 0.0))),
    FVec2::new((2.0, -1.0))
  );
  assert_eq!(
    lerp_fvec2(2.0, FVec2::new((2.0, 2.0)), FVec2::new((2.0, 0.0))),
    FVec2::new((2.0, -2.0))
  );
  assert_eq!(
    lerp_fvec2(2.0, FVec2::new((2.0, 2.0)), FVec2::new((2.0, 1.0))),
    FVec2::new((2.0, 0.0))
  );
  assert_eq!(
    lerp_fvec2(2.0, FVec2::new((2.0, 2.0)), FVec2::new((2.0, 2.0))),
    FVec2::new((2.0, 2.0))
  );
}

#[test]
fn test_lerp_dvec2() {
  assert_eq!(
    lerp_dvec2(0.0, DVec2::new((0.0, 0.0)), DVec2::new((0.0, 0.0))),
    DVec2::new((0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec2(1.0, DVec2::new((0.0, 0.0)), DVec2::new((0.0, 0.0))),
    DVec2::new((0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec2(2.0, DVec2::new((0.0, 0.0)), DVec2::new((0.0, 0.0))),
    DVec2::new((0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec2(2.0, DVec2::new((1.0, 0.0)), DVec2::new((0.0, 0.0))),
    DVec2::new((-1.0, 0.0))
  );
  assert_eq!(
    lerp_dvec2(2.0, DVec2::new((2.0, 0.0)), DVec2::new((0.0, 0.0))),
    DVec2::new((-2.0, 0.0))
  );
  assert_eq!(
    lerp_dvec2(2.0, DVec2::new((2.0, 0.0)), DVec2::new((1.0, 0.0))),
    DVec2::new((0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec2(2.0, DVec2::new((2.0, 0.0)), DVec2::new((2.0, 0.0))),
    DVec2::new((2.0, 0.0))
  );
  assert_eq!(
    lerp_dvec2(2.0, DVec2::new((2.0, 1.0)), DVec2::new((2.0, 0.0))),
    DVec2::new((2.0, -1.0))
  );
  assert_eq!(
    lerp_dvec2(2.0, DVec2::new((2.0, 2.0)), DVec2::new((2.0, 0.0))),
    DVec2::new((2.0, -2.0))
  );
  assert_eq!(
    lerp_dvec2(2.0, DVec2::new((2.0, 2.0)), DVec2::new((2.0, 1.0))),
    DVec2::new((2.0, 0.0))
  );
  assert_eq!(
    lerp_dvec2(2.0, DVec2::new((2.0, 2.0)), DVec2::new((2.0, 2.0))),
    DVec2::new((2.0, 2.0))
  );
}

#[test]
fn test_lerp_fvec3() {
  assert_eq!(
    lerp_fvec3(
      0.0,
      FVec3::new((0.0, 0.0, 0.0)),
      FVec3::new((0.0, 0.0, 0.0))
    ),
    FVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec3(
      1.0,
      FVec3::new((0.0, 0.0, 0.0)),
      FVec3::new((0.0, 0.0, 0.0))
    ),
    FVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec3(
      2.0,
      FVec3::new((0.0, 0.0, 0.0)),
      FVec3::new((0.0, 0.0, 0.0))
    ),
    FVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec3(
      2.0,
      FVec3::new((1.0, 0.0, 0.0)),
      FVec3::new((0.0, 0.0, 0.0))
    ),
    FVec3::new((-1.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec3(
      2.0,
      FVec3::new((2.0, 0.0, 0.0)),
      FVec3::new((0.0, 0.0, 0.0))
    ),
    FVec3::new((-2.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec3(
      2.0,
      FVec3::new((2.0, 0.0, 0.0)),
      FVec3::new((1.0, 0.0, 0.0))
    ),
    FVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec3(
      2.0,
      FVec3::new((2.0, 0.0, 0.0)),
      FVec3::new((2.0, 0.0, 0.0))
    ),
    FVec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec3(
      2.0,
      FVec3::new((2.0, 1.0, 0.0)),
      FVec3::new((2.0, 0.0, 0.0))
    ),
    FVec3::new((2.0, -1.0, 0.0))
  );
  assert_eq!(
    lerp_fvec3(
      2.0,
      FVec3::new((2.0, 2.0, 0.0)),
      FVec3::new((2.0, 0.0, 0.0))
    ),
    FVec3::new((2.0, -2.0, 0.0))
  );
  assert_eq!(
    lerp_fvec3(
      2.0,
      FVec3::new((2.0, 2.0, 0.0)),
      FVec3::new((2.0, 1.0, 0.0))
    ),
    FVec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec3(
      2.0,
      FVec3::new((2.0, 2.0, 0.0)),
      FVec3::new((2.0, 2.0, 0.0))
    ),
    FVec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    lerp_fvec3(
      2.0,
      FVec3::new((2.0, 2.0, 1.0)),
      FVec3::new((2.0, 2.0, 0.0))
    ),
    FVec3::new((2.0, 2.0, -1.0))
  );
  assert_eq!(
    lerp_fvec3(
      2.0,
      FVec3::new((2.0, 2.0, 2.0)),
      FVec3::new((2.0, 2.0, 0.0))
    ),
    FVec3::new((2.0, 2.0, -2.0))
  );
  assert_eq!(
    lerp_fvec3(
      2.0,
      FVec3::new((2.0, 2.0, 2.0)),
      FVec3::new((2.0, 2.0, 1.0))
    ),
    FVec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    lerp_fvec3(
      2.0,
      FVec3::new((2.0, 2.0, 2.0)),
      FVec3::new((2.0, 2.0, 2.0))
    ),
    FVec3::new((2.0, 2.0, 2.0))
  );
}

#[test]
fn test_lerp_dvec3() {
  assert_eq!(
    lerp_dvec3(
      0.0,
      DVec3::new((0.0, 0.0, 0.0)),
      DVec3::new((0.0, 0.0, 0.0))
    ),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec3(
      1.0,
      DVec3::new((0.0, 0.0, 0.0)),
      DVec3::new((0.0, 0.0, 0.0))
    ),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec3(
      2.0,
      DVec3::new((0.0, 0.0, 0.0)),
      DVec3::new((0.0, 0.0, 0.0))
    ),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec3(
      2.0,
      DVec3::new((1.0, 0.0, 0.0)),
      DVec3::new((0.0, 0.0, 0.0))
    ),
    DVec3::new((-1.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec3(
      2.0,
      DVec3::new((2.0, 0.0, 0.0)),
      DVec3::new((0.0, 0.0, 0.0))
    ),
    DVec3::new((-2.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec3(
      2.0,
      DVec3::new((2.0, 0.0, 0.0)),
      DVec3::new((1.0, 0.0, 0.0))
    ),
    DVec3::new((0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec3(
      2.0,
      DVec3::new((2.0, 0.0, 0.0)),
      DVec3::new((2.0, 0.0, 0.0))
    ),
    DVec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec3(
      2.0,
      DVec3::new((2.0, 1.0, 0.0)),
      DVec3::new((2.0, 0.0, 0.0))
    ),
    DVec3::new((2.0, -1.0, 0.0))
  );
  assert_eq!(
    lerp_dvec3(
      2.0,
      DVec3::new((2.0, 2.0, 0.0)),
      DVec3::new((2.0, 0.0, 0.0))
    ),
    DVec3::new((2.0, -2.0, 0.0))
  );
  assert_eq!(
    lerp_dvec3(
      2.0,
      DVec3::new((2.0, 2.0, 0.0)),
      DVec3::new((2.0, 1.0, 0.0))
    ),
    DVec3::new((2.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec3(
      2.0,
      DVec3::new((2.0, 2.0, 0.0)),
      DVec3::new((2.0, 2.0, 0.0))
    ),
    DVec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    lerp_dvec3(
      2.0,
      DVec3::new((2.0, 2.0, 1.0)),
      DVec3::new((2.0, 2.0, 0.0))
    ),
    DVec3::new((2.0, 2.0, -1.0))
  );
  assert_eq!(
    lerp_dvec3(
      2.0,
      DVec3::new((2.0, 2.0, 2.0)),
      DVec3::new((2.0, 2.0, 0.0))
    ),
    DVec3::new((2.0, 2.0, -2.0))
  );
  assert_eq!(
    lerp_dvec3(
      2.0,
      DVec3::new((2.0, 2.0, 2.0)),
      DVec3::new((2.0, 2.0, 1.0))
    ),
    DVec3::new((2.0, 2.0, 0.0))
  );
  assert_eq!(
    lerp_dvec3(
      2.0,
      DVec3::new((2.0, 2.0, 2.0)),
      DVec3::new((2.0, 2.0, 2.0))
    ),
    DVec3::new((2.0, 2.0, 2.0))
  );
}

#[test]
fn test_lerp_fvec4() {
  assert_eq!(
    lerp_fvec4(
      0.0,
      FVec4::new((0.0, 0.0, 0.0, 0.0)),
      FVec4::new((0.0, 0.0, 0.0, 0.0))
    ),
    FVec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec4(
      1.0,
      FVec4::new((0.0, 0.0, 0.0, 0.0)),
      FVec4::new((0.0, 0.0, 0.0, 0.0))
    ),
    FVec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec4(
      2.0,
      FVec4::new((0.0, 0.0, 0.0, 0.0)),
      FVec4::new((0.0, 0.0, 0.0, 0.0))
    ),
    FVec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec4(
      2.0,
      FVec4::new((1.0, 0.0, 0.0, 0.0)),
      FVec4::new((0.0, 0.0, 0.0, 0.0))
    ),
    FVec4::new((-1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec4(
      2.0,
      FVec4::new((2.0, 0.0, 0.0, 0.0)),
      FVec4::new((0.0, 0.0, 0.0, 0.0))
    ),
    FVec4::new((-2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec4(
      2.0,
      FVec4::new((2.0, 0.0, 0.0, 0.0)),
      FVec4::new((1.0, 0.0, 0.0, 0.0))
    ),
    FVec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec4(
      2.0,
      FVec4::new((2.0, 0.0, 0.0, 0.0)),
      FVec4::new((2.0, 0.0, 0.0, 0.0))
    ),
    FVec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec4(
      2.0,
      FVec4::new((2.0, 1.0, 0.0, 0.0)),
      FVec4::new((2.0, 0.0, 0.0, 0.0))
    ),
    FVec4::new((2.0, -1.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec4(
      2.0,
      FVec4::new((2.0, 2.0, 0.0, 0.0)),
      FVec4::new((2.0, 0.0, 0.0, 0.0))
    ),
    FVec4::new((2.0, -2.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec4(
      2.0,
      FVec4::new((2.0, 2.0, 0.0, 0.0)),
      FVec4::new((2.0, 1.0, 0.0, 0.0))
    ),
    FVec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec4(
      2.0,
      FVec4::new((2.0, 2.0, 0.0, 0.0)),
      FVec4::new((2.0, 2.0, 0.0, 0.0))
    ),
    FVec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec4(
      2.0,
      FVec4::new((2.0, 2.0, 1.0, 0.0)),
      FVec4::new((2.0, 2.0, 0.0, 0.0))
    ),
    FVec4::new((2.0, 2.0, -1.0, 0.0))
  );
  assert_eq!(
    lerp_fvec4(
      2.0,
      FVec4::new((2.0, 2.0, 2.0, 0.0)),
      FVec4::new((2.0, 2.0, 0.0, 0.0))
    ),
    FVec4::new((2.0, 2.0, -2.0, 0.0))
  );
  assert_eq!(
    lerp_fvec4(
      2.0,
      FVec4::new((2.0, 2.0, 2.0, 0.0)),
      FVec4::new((2.0, 2.0, 1.0, 0.0))
    ),
    FVec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_fvec4(
      2.0,
      FVec4::new((2.0, 2.0, 2.0, 0.0)),
      FVec4::new((2.0, 2.0, 2.0, 0.0))
    ),
    FVec4::new((2.0, 2.0, 2.0, 0.0))
  );
  assert_eq!(
    lerp_fvec4(
      2.0,
      FVec4::new((2.0, 2.0, 2.0, 1.0)),
      FVec4::new((2.0, 2.0, 2.0, 0.0))
    ),
    FVec4::new((2.0, 2.0, 2.0, -1.0))
  );
  assert_eq!(
    lerp_fvec4(
      2.0,
      FVec4::new((2.0, 2.0, 2.0, 2.0)),
      FVec4::new((2.0, 2.0, 2.0, 0.0))
    ),
    FVec4::new((2.0, 2.0, 2.0, -2.0))
  );
  assert_eq!(
    lerp_fvec4(
      2.0,
      FVec4::new((2.0, 2.0, 2.0, 2.0)),
      FVec4::new((2.0, 2.0, 2.0, 1.0))
    ),
    FVec4::new((2.0, 2.0, 2.0, 0.0))
  );
  assert_eq!(
    lerp_fvec4(
      2.0,
      FVec4::new((2.0, 2.0, 2.0, 2.0)),
      FVec4::new((2.0, 2.0, 2.0, 2.0))
    ),
    FVec4::new((2.0, 2.0, 2.0, 2.0))
  );
}

#[test]
fn test_lerp_dvec4() {
  assert_eq!(
    lerp_dvec4(
      0.0,
      DVec4::new((0.0, 0.0, 0.0, 0.0)),
      DVec4::new((0.0, 0.0, 0.0, 0.0))
    ),
    DVec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec4(
      1.0,
      DVec4::new((0.0, 0.0, 0.0, 0.0)),
      DVec4::new((0.0, 0.0, 0.0, 0.0))
    ),
    DVec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec4(
      2.0,
      DVec4::new((0.0, 0.0, 0.0, 0.0)),
      DVec4::new((0.0, 0.0, 0.0, 0.0))
    ),
    DVec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec4(
      2.0,
      DVec4::new((1.0, 0.0, 0.0, 0.0)),
      DVec4::new((0.0, 0.0, 0.0, 0.0))
    ),
    DVec4::new((-1.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec4(
      2.0,
      DVec4::new((2.0, 0.0, 0.0, 0.0)),
      DVec4::new((0.0, 0.0, 0.0, 0.0))
    ),
    DVec4::new((-2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec4(
      2.0,
      DVec4::new((2.0, 0.0, 0.0, 0.0)),
      DVec4::new((1.0, 0.0, 0.0, 0.0))
    ),
    DVec4::new((0.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec4(
      2.0,
      DVec4::new((2.0, 0.0, 0.0, 0.0)),
      DVec4::new((2.0, 0.0, 0.0, 0.0))
    ),
    DVec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec4(
      2.0,
      DVec4::new((2.0, 1.0, 0.0, 0.0)),
      DVec4::new((2.0, 0.0, 0.0, 0.0))
    ),
    DVec4::new((2.0, -1.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec4(
      2.0,
      DVec4::new((2.0, 2.0, 0.0, 0.0)),
      DVec4::new((2.0, 0.0, 0.0, 0.0))
    ),
    DVec4::new((2.0, -2.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec4(
      2.0,
      DVec4::new((2.0, 2.0, 0.0, 0.0)),
      DVec4::new((2.0, 1.0, 0.0, 0.0))
    ),
    DVec4::new((2.0, 0.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec4(
      2.0,
      DVec4::new((2.0, 2.0, 0.0, 0.0)),
      DVec4::new((2.0, 2.0, 0.0, 0.0))
    ),
    DVec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec4(
      2.0,
      DVec4::new((2.0, 2.0, 1.0, 0.0)),
      DVec4::new((2.0, 2.0, 0.0, 0.0))
    ),
    DVec4::new((2.0, 2.0, -1.0, 0.0))
  );
  assert_eq!(
    lerp_dvec4(
      2.0,
      DVec4::new((2.0, 2.0, 2.0, 0.0)),
      DVec4::new((2.0, 2.0, 0.0, 0.0))
    ),
    DVec4::new((2.0, 2.0, -2.0, 0.0))
  );
  assert_eq!(
    lerp_dvec4(
      2.0,
      DVec4::new((2.0, 2.0, 2.0, 0.0)),
      DVec4::new((2.0, 2.0, 1.0, 0.0))
    ),
    DVec4::new((2.0, 2.0, 0.0, 0.0))
  );
  assert_eq!(
    lerp_dvec4(
      2.0,
      DVec4::new((2.0, 2.0, 2.0, 0.0)),
      DVec4::new((2.0, 2.0, 2.0, 0.0))
    ),
    DVec4::new((2.0, 2.0, 2.0, 0.0))
  );
  assert_eq!(
    lerp_dvec4(
      2.0,
      DVec4::new((2.0, 2.0, 2.0, 1.0)),
      DVec4::new((2.0, 2.0, 2.0, 0.0))
    ),
    DVec4::new((2.0, 2.0, 2.0, -1.0))
  );
  assert_eq!(
    lerp_dvec4(
      2.0,
      DVec4::new((2.0, 2.0, 2.0, 2.0)),
      DVec4::new((2.0, 2.0, 2.0, 0.0))
    ),
    DVec4::new((2.0, 2.0, 2.0, -2.0))
  );
  assert_eq!(
    lerp_dvec4(
      2.0,
      DVec4::new((2.0, 2.0, 2.0, 2.0)),
      DVec4::new((2.0, 2.0, 2.0, 1.0))
    ),
    DVec4::new((2.0, 2.0, 2.0, 0.0))
  );
  assert_eq!(
    lerp_dvec4(
      2.0,
      DVec4::new((2.0, 2.0, 2.0, 2.0)),
      DVec4::new((2.0, 2.0, 2.0, 2.0))
    ),
    DVec4::new((2.0, 2.0, 2.0, 2.0))
  );
}

#[test]
fn test_linear_map_f64() {
  assert_eq!(
    linear_map_f64(0.0, DSeq::new(0.0, 1.0), DSeq::new(0.0, 1.0)),
    0.0
  );
  assert_eq!(
    linear_map_f64(1.0, DSeq::new(0.0, 1.0), DSeq::new(0.0, 1.0)),
    1.0
  );
  assert_eq!(
    linear_map_f64(2.0, DSeq::new(0.0, 1.0), DSeq::new(0.0, 1.0)),
    2.0
  );
  assert_eq!(
    linear_map_f64(2.0, DSeq::new(0.0, 2.0), DSeq::new(0.0, 1.0)),
    1.0
  );
  assert_eq!(
    linear_map_f64(2.0, DSeq::new(0.0, 4.0), DSeq::new(0.0, 1.0)),
    0.5
  );
  assert_eq!(
    linear_map_f64(2.0, DSeq::new(2.0, 4.0), DSeq::new(0.0, 1.0)),
    0.0
  );
  assert_eq!(
    linear_map_f64(2.0, DSeq::new(3.0, 4.0), DSeq::new(0.0, 1.0)),
    -1.0
  );
  assert_eq!(
    linear_map_f64(2.0, DSeq::new(3.0, 4.0), DSeq::new(0.0, 2.0)),
    -2.0
  );
  assert_eq!(
    linear_map_f64(2.0, DSeq::new(3.0, 4.0), DSeq::new(0.0, 3.0)),
    -3.0
  );
  assert_eq!(
    linear_map_f64(2.0, DSeq::new(3.0, 4.0), DSeq::new(1.0, 3.0)),
    -1.0
  );
  assert_eq!(
    linear_map_f64(2.0, DSeq::new(3.0, 4.0), DSeq::new(2.0, 3.0)),
    1.0
  );
}

#[test]
fn test_linear_map_dvec2() {
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((0.0, 0.0)),
      DVec2::new((0.0, 0.0)),
      DVec2::new((1.0, 1.0)),
      DVec2::new((0.0, 0.0)),
      DVec2::new((1.0, 1.0))
    ),
    DVec2::new((0.0, 0.0))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((1.0, 0.0)),
      DVec2::new((0.0, 0.0)),
      DVec2::new((1.0, 1.0)),
      DVec2::new((0.0, 0.0)),
      DVec2::new((1.0, 1.0))
    ),
    DVec2::new((1.0, 0.0))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((2.0, 0.0)),
      DVec2::new((0.0, 0.0)),
      DVec2::new((1.0, 1.0)),
      DVec2::new((0.0, 0.0)),
      DVec2::new((1.0, 1.0))
    ),
    DVec2::new((2.0, 0.0))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((2.0, 1.0)),
      DVec2::new((0.0, 0.0)),
      DVec2::new((1.0, 1.0)),
      DVec2::new((0.0, 0.0)),
      DVec2::new((1.0, 1.0))
    ),
    DVec2::new((2.0, 1.0))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((2.0, 2.0)),
      DVec2::new((0.0, 0.0)),
      DVec2::new((1.0, 1.0)),
      DVec2::new((0.0, 0.0)),
      DVec2::new((1.0, 1.0))
    ),
    DVec2::new((2.0, 2.0))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((2.0, 2.0)),
      DVec2::new((1.0, 0.0)),
      DVec2::new((1.0, 1.0)),
      DVec2::new((0.0, 0.0)),
      DVec2::new((1.0, 1.0))
    ),
    DVec2::new((1.0, 2.0))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((2.0, 2.0)),
      DVec2::new((2.0, 0.0)),
      DVec2::new((1.0, 1.0)),
      DVec2::new((0.0, 0.0)),
      DVec2::new((1.0, 1.0))
    ),
    DVec2::new((0.0, 2.0))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((2.0, 2.0)),
      DVec2::new((2.0, 1.0)),
      DVec2::new((1.0, 1.0)),
      DVec2::new((0.0, 0.0)),
      DVec2::new((1.0, 1.0))
    ),
    DVec2::new((0.0, 1.0))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((2.0, 2.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((1.0, 1.0)),
      DVec2::new((0.0, 0.0)),
      DVec2::new((1.0, 1.0))
    ),
    DVec2::new((0.0, 0.0))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((2.0, 2.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((1.0, 1.0)),
      DVec2::new((1.0, 0.0)),
      DVec2::new((1.0, 1.0))
    ),
    DVec2::new((1.0, 0.0))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((2.0, 2.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((1.0, 1.0)),
      DVec2::new((2.0, 0.0)),
      DVec2::new((1.0, 1.0))
    ),
    DVec2::new((2.0, 0.0))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((2.0, 2.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((1.0, 1.0)),
      DVec2::new((2.0, 1.0)),
      DVec2::new((1.0, 1.0))
    ),
    DVec2::new((2.0, 1.0))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((2.0, 2.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((1.0, 1.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((1.0, 1.0))
    ),
    DVec2::new((2.0, 2.0))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((3.0, 3.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((1.0, 1.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((1.0, 1.0))
    ),
    DVec2::new((3.0, 3.0))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((3.0, 3.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((2.0, 1.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((1.0, 1.0))
    ),
    DVec2::new((2.5, 3.0))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((3.0, 3.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((4.0, 1.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((1.0, 1.0))
    ),
    DVec2::new((2.25, 3.0))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((3.0, 3.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((4.0, 2.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((1.0, 1.0))
    ),
    DVec2::new((2.25, 2.5))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((3.0, 3.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((4.0, 4.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((1.0, 1.0))
    ),
    DVec2::new((2.25, 2.25))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((3.0, 3.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((4.0, 4.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((2.0, 1.0))
    ),
    DVec2::new((2.5, 2.25))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((3.0, 3.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((4.0, 4.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((4.0, 1.0))
    ),
    DVec2::new((3.0, 2.25))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((3.0, 3.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((4.0, 4.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((4.0, 2.0))
    ),
    DVec2::new((3.0, 2.5))
  );
  assert_eq!(
    linear_map_dvec2(
      DVec2::new((3.0, 3.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((4.0, 4.0)),
      DVec2::new((2.0, 2.0)),
      DVec2::new((4.0, 4.0))
    ),
    DVec2::new((3.0, 3.0))
  );
}

#[test]
fn test_min_pos_f32() {
  assert_eq!(([] as [f32; 0]).min_pos(), None);
  assert_eq!([-2.0f32].min_pos(), None);
  assert_eq!([-1.0f32].min_pos(), None);
  assert_eq!([0.0f32].min_pos(), Some((0, 0.0f32)));
  assert_eq!([1.0f32].min_pos(), Some((0, 1.0f32)));
  assert_eq!([2.0f32].min_pos(), Some((0, 2.0f32)));
  assert_eq!([2.0f32, -2.0f32].min_pos(), Some((0, 2.0f32)));
  assert_eq!([2.0f32, -1.0f32].min_pos(), Some((0, 2.0f32)));
  assert_eq!([2.0f32, 0.0f32].min_pos(), Some((1, 0.0f32)));
  assert_eq!([2.0f32, 1.0f32].min_pos(), Some((1, 1.0f32)));
  assert_eq!([2.0f32, 2.0f32].min_pos(), Some((0, 2.0f32)));
  assert_eq!([2.0f32, 2.0f32, -2.0f32].min_pos(), Some((0, 2.0f32)));
  assert_eq!([2.0f32, 2.0f32, -1.0f32].min_pos(), Some((0, 2.0f32)));
  assert_eq!([2.0f32, 2.0f32, 0.0f32].min_pos(), Some((2, 0.0f32)));
  assert_eq!([2.0f32, 2.0f32, 1.0f32].min_pos(), Some((2, 1.0f32)));
  assert_eq!([2.0f32, 2.0f32, 2.0f32].min_pos(), Some((0, 2.0f32)));
}

#[test]
fn test_max_pos_f32() {
  assert_eq!(([] as [f32; 0]).max_pos(), None);
  assert_eq!([-2.0f32].max_pos(), None);
  assert_eq!([-1.0f32].max_pos(), None);
  assert_eq!([0.0f32].max_pos(), Some((0, 0.0f32)));
  assert_eq!([1.0f32].max_pos(), Some((0, 1.0f32)));
  assert_eq!([2.0f32].max_pos(), Some((0, 2.0f32)));
  assert_eq!([0.0f32, -2.0f32].max_pos(), Some((0, 0.0f32)));
  assert_eq!([0.0f32, -1.0f32].max_pos(), Some((0, 0.0f32)));
  assert_eq!([0.0f32, 0.0f32].max_pos(), Some((0, 0.0f32)));
  assert_eq!([0.0f32, 1.0f32].max_pos(), Some((1, 1.0f32)));
  assert_eq!([0.0f32, 2.0f32].max_pos(), Some((1, 2.0f32)));
  assert_eq!([0.0f32, 0.0f32, -2.0f32].max_pos(), Some((0, 0.0f32)));
  assert_eq!([0.0f32, 0.0f32, -1.0f32].max_pos(), Some((0, 0.0f32)));
  assert_eq!([0.0f32, 0.0f32, 0.0f32].max_pos(), Some((0, 0.0f32)));
  assert_eq!([0.0f32, 0.0f32, 1.0f32].max_pos(), Some((2, 1.0f32)));
  assert_eq!([0.0f32, 0.0f32, 2.0f32].max_pos(), Some((2, 2.0f32)));
}

#[test]
fn test_min_neg_f32() {
  assert_eq!(([] as [f32; 0]).min_neg(), None);
  assert_eq!([-2.0f32].min_neg(), Some((0, -2.0f32)));
  assert_eq!([-1.0f32].min_neg(), Some((0, -1.0f32)));
  assert_eq!([0.0f32].min_neg(), None);
  assert_eq!([1.0f32].min_neg(), None);
  assert_eq!([2.0f32].min_neg(), None);
  assert_eq!([-1.0f32, -2.0f32].min_neg(), Some((1, -2.0f32)));
  assert_eq!([-1.0f32, -1.0f32].min_neg(), Some((0, -1.0f32)));
  assert_eq!([-1.0f32, 0.0f32].min_neg(), Some((0, -1.0f32)));
  assert_eq!([-1.0f32, 1.0f32].min_neg(), Some((0, -1.0f32)));
  assert_eq!([-1.0f32, 2.0f32].min_neg(), Some((0, -1.0f32)));
  assert_eq!([-1.0f32, -1.0f32, -2.0f32].min_neg(), Some((2, -2.0f32)));
  assert_eq!([-1.0f32, -1.0f32, -1.0f32].min_neg(), Some((0, -1.0f32)));
  assert_eq!([-1.0f32, -1.0f32, 0.0f32].min_neg(), Some((0, -1.0f32)));
  assert_eq!([-1.0f32, -1.0f32, 1.0f32].min_neg(), Some((0, -1.0f32)));
  assert_eq!([-1.0f32, -1.0f32, 2.0f32].min_neg(), Some((0, -1.0f32)));
}

#[test]
fn test_max_neg_f32() {
  assert_eq!(([] as [f32; 0]).max_neg(), None);
  assert_eq!([-2.0f32].max_neg(), Some((0, -2.0f32)));
  assert_eq!([-1.0f32].max_neg(), Some((0, -1.0f32)));
  assert_eq!([0.0f32].max_neg(), None);
  assert_eq!([1.0f32].max_neg(), None);
  assert_eq!([2.0f32].max_neg(), None);
  assert_eq!([-3.0f32, -2.0f32].max_neg(), Some((1, -2.0f32)));
  assert_eq!([-3.0f32, -1.0f32].max_neg(), Some((1, -1.0f32)));
  assert_eq!([-3.0f32, 0.0f32].max_neg(), Some((0, -3.0f32)));
  assert_eq!([-3.0f32, 1.0f32].max_neg(), Some((0, -3.0f32)));
  assert_eq!([-3.0f32, 2.0f32].max_neg(), Some((0, -3.0f32)));
  assert_eq!([-3.0f32, -3.0f32, -2.0f32].max_neg(), Some((2, -2.0f32)));
  assert_eq!([-3.0f32, -3.0f32, -1.0f32].max_neg(), Some((2, -1.0f32)));
  assert_eq!([-3.0f32, -3.0f32, 0.0f32].max_neg(), Some((0, -3.0f32)));
  assert_eq!([-3.0f32, -3.0f32, 1.0f32].max_neg(), Some((0, -3.0f32)));
  assert_eq!([-3.0f32, -3.0f32, 2.0f32].max_neg(), Some((0, -3.0f32)));
}

#[test]
fn test_min_pos_f64() {
  assert_eq!(([] as [f64; 0]).min_pos(), None);
  assert_eq!([-2.0f64].min_pos(), None);
  assert_eq!([-1.0f64].min_pos(), None);
  assert_eq!([0.0f64].min_pos(), Some((0, 0.0f64)));
  assert_eq!([1.0f64].min_pos(), Some((0, 1.0f64)));
  assert_eq!([2.0f64].min_pos(), Some((0, 2.0f64)));
  assert_eq!([2.0f64, -2.0f64].min_pos(), Some((0, 2.0f64)));
  assert_eq!([2.0f64, -1.0f64].min_pos(), Some((0, 2.0f64)));
  assert_eq!([2.0f64, 0.0f64].min_pos(), Some((1, 0.0f64)));
  assert_eq!([2.0f64, 1.0f64].min_pos(), Some((1, 1.0f64)));
  assert_eq!([2.0f64, 2.0f64].min_pos(), Some((0, 2.0f64)));
  assert_eq!([2.0f64, 2.0f64, -2.0f64].min_pos(), Some((0, 2.0f64)));
  assert_eq!([2.0f64, 2.0f64, -1.0f64].min_pos(), Some((0, 2.0f64)));
  assert_eq!([2.0f64, 2.0f64, 0.0f64].min_pos(), Some((2, 0.0f64)));
  assert_eq!([2.0f64, 2.0f64, 1.0f64].min_pos(), Some((2, 1.0f64)));
  assert_eq!([2.0f64, 2.0f64, 2.0f64].min_pos(), Some((0, 2.0f64)));
}

#[test]
fn test_max_pos_f64() {
  assert_eq!(([] as [f64; 0]).max_pos(), None);
  assert_eq!([-2.0f64].max_pos(), None);
  assert_eq!([-1.0f64].max_pos(), None);
  assert_eq!([0.0f64].max_pos(), Some((0, 0.0f64)));
  assert_eq!([1.0f64].max_pos(), Some((0, 1.0f64)));
  assert_eq!([2.0f64].max_pos(), Some((0, 2.0f64)));
  assert_eq!([0.0f64, -2.0f64].max_pos(), Some((0, 0.0f64)));
  assert_eq!([0.0f64, -1.0f64].max_pos(), Some((0, 0.0f64)));
  assert_eq!([0.0f64, 0.0f64].max_pos(), Some((0, 0.0f64)));
  assert_eq!([0.0f64, 1.0f64].max_pos(), Some((1, 1.0f64)));
  assert_eq!([0.0f64, 2.0f64].max_pos(), Some((1, 2.0f64)));
  assert_eq!([0.0f64, 0.0f64, -2.0f64].max_pos(), Some((0, 0.0f64)));
  assert_eq!([0.0f64, 0.0f64, -1.0f64].max_pos(), Some((0, 0.0f64)));
  assert_eq!([0.0f64, 0.0f64, 0.0f64].max_pos(), Some((0, 0.0f64)));
  assert_eq!([0.0f64, 0.0f64, 1.0f64].max_pos(), Some((2, 1.0f64)));
  assert_eq!([0.0f64, 0.0f64, 2.0f64].max_pos(), Some((2, 2.0f64)));
}

#[test]
fn test_min_neg_f64() {
  assert_eq!(([] as [f64; 0]).min_neg(), None);
  assert_eq!([-2.0f64].min_neg(), Some((0, -2.0f64)));
  assert_eq!([-1.0f64].min_neg(), Some((0, -1.0f64)));
  assert_eq!([0.0f64].min_neg(), None);
  assert_eq!([1.0f64].min_neg(), None);
  assert_eq!([2.0f64].min_neg(), None);
  assert_eq!([-1.0f64, -2.0f64].min_neg(), Some((1, -2.0f64)));
  assert_eq!([-1.0f64, -1.0f64].min_neg(), Some((0, -1.0f64)));
  assert_eq!([-1.0f64, 0.0f64].min_neg(), Some((0, -1.0f64)));
  assert_eq!([-1.0f64, 1.0f64].min_neg(), Some((0, -1.0f64)));
  assert_eq!([-1.0f64, 2.0f64].min_neg(), Some((0, -1.0f64)));
  assert_eq!([-1.0f64, -1.0f64, -2.0f64].min_neg(), Some((2, -2.0f64)));
  assert_eq!([-1.0f64, -1.0f64, -1.0f64].min_neg(), Some((0, -1.0f64)));
  assert_eq!([-1.0f64, -1.0f64, 0.0f64].min_neg(), Some((0, -1.0f64)));
  assert_eq!([-1.0f64, -1.0f64, 1.0f64].min_neg(), Some((0, -1.0f64)));
  assert_eq!([-1.0f64, -1.0f64, 2.0f64].min_neg(), Some((0, -1.0f64)));
}

#[test]
fn test_max_neg_f64() {
  assert_eq!(([] as [f64; 0]).max_neg(), None);
  assert_eq!([-2.0f64].max_neg(), Some((0, -2.0f64)));
  assert_eq!([-1.0f64].max_neg(), Some((0, -1.0f64)));
  assert_eq!([0.0f64].max_neg(), None);
  assert_eq!([1.0f64].max_neg(), None);
  assert_eq!([2.0f64].max_neg(), None);
  assert_eq!([-3.0f64, -2.0f64].max_neg(), Some((1, -2.0f64)));
  assert_eq!([-3.0f64, -1.0f64].max_neg(), Some((1, -1.0f64)));
  assert_eq!([-3.0f64, 0.0f64].max_neg(), Some((0, -3.0f64)));
  assert_eq!([-3.0f64, 1.0f64].max_neg(), Some((0, -3.0f64)));
  assert_eq!([-3.0f64, 2.0f64].max_neg(), Some((0, -3.0f64)));
  assert_eq!([-3.0f64, -3.0f64, -2.0f64].max_neg(), Some((2, -2.0f64)));
  assert_eq!([-3.0f64, -3.0f64, -1.0f64].max_neg(), Some((2, -1.0f64)));
  assert_eq!([-3.0f64, -3.0f64, 0.0f64].max_neg(), Some((0, -3.0f64)));
  assert_eq!([-3.0f64, -3.0f64, 1.0f64].max_neg(), Some((0, -3.0f64)));
  assert_eq!([-3.0f64, -3.0f64, 2.0f64].max_neg(), Some((0, -3.0f64)));
}
