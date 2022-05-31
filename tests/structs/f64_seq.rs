use iron_ingot::{F64Bound, F64Seq, I32Bound, I32Seq};

#[test]
fn test_normalize() {
  assert_eq!(F64Seq::new((0.0, 1.0)).normalize(0.0), 0.0);
  assert_eq!(F64Seq::new((0.0, 2.0)).normalize(0.0), 0.0);
  assert_eq!(F64Seq::new((0.0, 5.0)).normalize(0.0), 0.0);
  assert_eq!(F64Seq::new((0.0, 5.0)).normalize(1.0), 0.2);
  assert_eq!(F64Seq::new((0.0, 5.0)).normalize(2.0), 0.4);
  assert_eq!(F64Seq::new((1.0, 5.0)).normalize(2.0), 0.25);
  assert_eq!(F64Seq::new((2.0, 5.0)).normalize(2.0), 0.0);
}

#[test]
fn test_unnormalize() {
  assert_eq!(F64Seq::new((0.0, 1.0)).unnormalize(0.0), 0.0);
  assert_eq!(F64Seq::new((0.0, 2.0)).unnormalize(0.0), 0.0);
  assert_eq!(F64Seq::new((0.0, 5.0)).unnormalize(0.0), 0.0);
  assert_eq!(F64Seq::new((0.0, 5.0)).unnormalize(0.2), 1.0);
  assert_eq!(F64Seq::new((0.0, 5.0)).unnormalize(0.4), 2.0);
  assert_eq!(F64Seq::new((1.0, 5.0)).unnormalize(0.25), 2.0);
  assert_eq!(F64Seq::new((2.0, 5.0)).unnormalize(0.0), 2.0);
}

#[test]
fn test_into_f64_bound() {
  assert_eq!(
    F64Bound::from(F64Seq::new((0.0, 1.0))),
    F64Bound::new((0.0, 1.0))
  );
  assert_eq!(
    F64Bound::from(F64Seq::new((0.0, 2.0))),
    F64Bound::new((0.0, 2.0))
  );
  assert_eq!(
    F64Bound::from(F64Seq::new((0.0, 3.0))),
    F64Bound::new((0.0, 3.0))
  );
  assert_eq!(
    F64Bound::from(F64Seq::new((1.0, 3.0))),
    F64Bound::new((1.0, 3.0))
  );
  assert_eq!(
    F64Bound::from(F64Seq::new((2.0, 3.0))),
    F64Bound::new((2.0, 3.0))
  );
}

#[test]
fn test_into_u32_seq() {
  assert_eq!(I32Seq::from(F64Seq::new((0.0, 1.0))), I32Seq::new((0, 1)));
  assert_eq!(I32Seq::from(F64Seq::new((0.0, 2.0))), I32Seq::new((0, 2)));
  assert_eq!(I32Seq::from(F64Seq::new((0.0, 3.0))), I32Seq::new((0, 3)));
  assert_eq!(I32Seq::from(F64Seq::new((1.0, 3.0))), I32Seq::new((1, 3)));
  assert_eq!(I32Seq::from(F64Seq::new((2.0, 3.0))), I32Seq::new((2, 3)));
}

#[test]
fn test_into_u32_bound() {
  assert_eq!(
    I32Bound::from(F64Seq::new((0.0, 1.0))),
    I32Bound::new((0, 1))
  );
  assert_eq!(
    I32Bound::from(F64Seq::new((0.0, 2.0))),
    I32Bound::new((0, 2))
  );
  assert_eq!(
    I32Bound::from(F64Seq::new((0.0, 3.0))),
    I32Bound::new((0, 3))
  );
  assert_eq!(
    I32Bound::from(F64Seq::new((1.0, 3.0))),
    I32Bound::new((1, 3))
  );
  assert_eq!(
    I32Bound::from(F64Seq::new((2.0, 3.0))),
    I32Bound::new((2, 3))
  );
}
