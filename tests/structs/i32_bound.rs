use iron_ingot::{F64Bound, F64Seq, I32Bound, I32Seq};

#[test]
fn test_get_middle() {
  assert_eq!(I32Bound::new((0, 0)).get_middle(), 0);
  assert_eq!(I32Bound::new((0, 1)).get_middle(), 0);
  assert_eq!(I32Bound::new((0, 2)).get_middle(), 1);
  assert_eq!(I32Bound::new((1, 2)).get_middle(), 1);
  assert_eq!(I32Bound::new((2, 2)).get_middle(), 2);
}

#[test]
fn test_is_intersect() {
  assert!(I32Bound::new((0, 0)).is_intersect(I32Bound::new((0, 0))));
  assert!(I32Bound::new((0, 1)).is_intersect(I32Bound::new((0, 0))));
  assert!(I32Bound::new((0, 2)).is_intersect(I32Bound::new((0, 0))));
  assert!(!I32Bound::new((1, 2)).is_intersect(I32Bound::new((0, 0))));
  assert!(!I32Bound::new((2, 2)).is_intersect(I32Bound::new((0, 0))));
  assert!(!I32Bound::new((2, 2)).is_intersect(I32Bound::new((0, 1))));
  assert!(I32Bound::new((2, 2)).is_intersect(I32Bound::new((0, 2))));
  assert!(I32Bound::new((2, 2)).is_intersect(I32Bound::new((1, 2))));
  assert!(I32Bound::new((2, 2)).is_intersect(I32Bound::new((2, 2))));
}

#[test]
fn test_intersect() {
  assert_eq!(
    I32Bound::new((0, 0)).intersect(I32Bound::new((0, 0))),
    Some(I32Bound::new((0, 0)))
  );
  assert_eq!(
    I32Bound::new((0, 1)).intersect(I32Bound::new((0, 0))),
    Some(I32Bound::new((0, 0)))
  );
  assert_eq!(
    I32Bound::new((0, 2)).intersect(I32Bound::new((0, 0))),
    Some(I32Bound::new((0, 0)))
  );
  assert_eq!(I32Bound::new((1, 2)).intersect(I32Bound::new((0, 0))), None);
  assert_eq!(I32Bound::new((2, 2)).intersect(I32Bound::new((0, 0))), None);
  assert_eq!(I32Bound::new((2, 2)).intersect(I32Bound::new((0, 1))), None);
  assert_eq!(
    I32Bound::new((2, 2)).intersect(I32Bound::new((0, 2))),
    Some(I32Bound::new((2, 2)))
  );
  assert_eq!(
    I32Bound::new((2, 2)).intersect(I32Bound::new((1, 2))),
    Some(I32Bound::new((2, 2)))
  );
  assert_eq!(
    I32Bound::new((2, 2)).intersect(I32Bound::new((2, 2))),
    Some(I32Bound::new((2, 2)))
  );
}

#[test]
fn test_has() {
  assert!(I32Bound::new((0, 0)).has(0));
  assert!(I32Bound::new((0, 1)).has(0));
  assert!(I32Bound::new((0, 2)).has(0));
  assert!(!I32Bound::new((1, 2)).has(0));
  assert!(!I32Bound::new((2, 2)).has(0));
  assert!(!I32Bound::new((2, 2)).has(1));
  assert!(I32Bound::new((2, 2)).has(2));
}

#[test]
fn test_rand() {
  assert_eq!(I32Bound::new((0, 0)).rand(), 0);
  assert!(I32Bound::new((0, 1)).rand() <= 1);
  assert!(I32Bound::new((0, 2)).rand() <= 2);
  assert!(I32Bound::new((1, 2)).rand() >= 1);
  assert!(I32Bound::new((1, 2)).rand() <= 2);
  assert_eq!(I32Bound::new((2, 2)).rand(), 2);
}

#[test]
fn test_into_u32_seq() {
  assert_eq!(I32Seq::from(I32Bound::new((0, 1))), I32Seq::new((0, 1)));
  assert_eq!(I32Seq::from(I32Bound::new((0, 2))), I32Seq::new((0, 2)));
  assert_eq!(I32Seq::from(I32Bound::new((1, 2))), I32Seq::new((1, 2)));
}

#[test]
fn test_into_f64_seq() {
  assert_eq!(F64Seq::from(I32Bound::new((0, 1))), F64Seq::new((0.0, 1.0)));
  assert_eq!(F64Seq::from(I32Bound::new((0, 2))), F64Seq::new((0.0, 2.0)));
  assert_eq!(F64Seq::from(I32Bound::new((1, 2))), F64Seq::new((1.0, 2.0)));
}

#[test]
fn test_into_f64_bound() {
  assert_eq!(
    F64Bound::from(I32Bound::new((0, 1))),
    F64Bound::new((0.0, 1.0))
  );
  assert_eq!(
    F64Bound::from(I32Bound::new((0, 2))),
    F64Bound::new((0.0, 2.0))
  );
  assert_eq!(
    F64Bound::from(I32Bound::new((1, 2))),
    F64Bound::new((1.0, 2.0))
  );
}
