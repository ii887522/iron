use iron_ingot::{F64Bound, F64Seq, I32Bound, I32Seq};

#[test]
fn test_into_f64_bound() {
  assert_eq!(
    F64Bound::from(I32Seq::new((0, 1))),
    F64Bound::new((0.0, 1.0))
  );
  assert_eq!(
    F64Bound::from(I32Seq::new((0, 2))),
    F64Bound::new((0.0, 2.0))
  );
  assert_eq!(
    F64Bound::from(I32Seq::new((0, 3))),
    F64Bound::new((0.0, 3.0))
  );
  assert_eq!(
    F64Bound::from(I32Seq::new((1, 3))),
    F64Bound::new((1.0, 3.0))
  );
  assert_eq!(
    F64Bound::from(I32Seq::new((2, 3))),
    F64Bound::new((2.0, 3.0))
  );
}

#[test]
fn test_into_i32_bound() {
  assert_eq!(I32Bound::from(I32Seq::new((0, 1))), I32Bound::new((0, 1)));
  assert_eq!(I32Bound::from(I32Seq::new((0, 2))), I32Bound::new((0, 2)));
  assert_eq!(I32Bound::from(I32Seq::new((0, 3))), I32Bound::new((0, 3)));
  assert_eq!(I32Bound::from(I32Seq::new((1, 3))), I32Bound::new((1, 3)));
  assert_eq!(I32Bound::from(I32Seq::new((2, 3))), I32Bound::new((2, 3)));
}

#[test]
fn test_into_f64_seq() {
  assert_eq!(F64Seq::from(I32Seq::new((0, 1))), F64Seq::new((0.0, 1.0)));
  assert_eq!(F64Seq::from(I32Seq::new((0, 2))), F64Seq::new((0.0, 2.0)));
  assert_eq!(F64Seq::from(I32Seq::new((0, 3))), F64Seq::new((0.0, 3.0)));
  assert_eq!(F64Seq::from(I32Seq::new((1, 3))), F64Seq::new((1.0, 3.0)));
  assert_eq!(F64Seq::from(I32Seq::new((2, 3))), F64Seq::new((2.0, 3.0)));
}
