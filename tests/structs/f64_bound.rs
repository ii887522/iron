use iron_ingot::{F64Bound, F64Seq, I32Bound, I32Seq};

#[test]
fn test_get_middle() {
  assert_eq!(F64Bound::new((0.0, 0.0)).get_middle(), 0.0);
  assert_eq!(F64Bound::new((0.0, 1.0)).get_middle(), 0.5);
  assert_eq!(F64Bound::new((0.0, 2.0)).get_middle(), 1.0);
  assert_eq!(F64Bound::new((1.0, 2.0)).get_middle(), 1.5);
  assert_eq!(F64Bound::new((2.0, 2.0)).get_middle(), 2.0);
}

#[test]
fn test_is_intersect() {
  assert!(F64Bound::new((0.0, 0.0)).is_intersect(F64Bound::new((0.0, 0.0))));
  assert!(F64Bound::new((0.0, 1.0)).is_intersect(F64Bound::new((0.0, 0.0))));
  assert!(F64Bound::new((0.0, 2.0)).is_intersect(F64Bound::new((0.0, 0.0))));
  assert!(!F64Bound::new((1.0, 2.0)).is_intersect(F64Bound::new((0.0, 0.0))));
  assert!(!F64Bound::new((2.0, 2.0)).is_intersect(F64Bound::new((0.0, 0.0))));
  assert!(!F64Bound::new((2.0, 2.0)).is_intersect(F64Bound::new((0.0, 1.0))));
  assert!(F64Bound::new((2.0, 2.0)).is_intersect(F64Bound::new((0.0, 2.0))));
  assert!(F64Bound::new((2.0, 2.0)).is_intersect(F64Bound::new((1.0, 2.0))));
  assert!(F64Bound::new((2.0, 2.0)).is_intersect(F64Bound::new((2.0, 2.0))));
}

#[test]
fn test_intersect() {
  assert_eq!(
    F64Bound::new((0.0, 0.0)).intersect(F64Bound::new((0.0, 0.0))),
    Some(F64Bound::new((0.0, 0.0)))
  );
  assert_eq!(
    F64Bound::new((0.0, 1.0)).intersect(F64Bound::new((0.0, 0.0))),
    Some(F64Bound::new((0.0, 0.0)))
  );
  assert_eq!(
    F64Bound::new((0.0, 2.0)).intersect(F64Bound::new((0.0, 0.0))),
    Some(F64Bound::new((0.0, 0.0)))
  );
  assert_eq!(
    F64Bound::new((1.0, 2.0)).intersect(F64Bound::new((0.0, 0.0))),
    None
  );
  assert_eq!(
    F64Bound::new((2.0, 2.0)).intersect(F64Bound::new((0.0, 0.0))),
    None
  );
  assert_eq!(
    F64Bound::new((2.0, 2.0)).intersect(F64Bound::new((0.0, 1.0))),
    None
  );
  assert_eq!(
    F64Bound::new((2.0, 2.0)).intersect(F64Bound::new((0.0, 2.0))),
    Some(F64Bound::new((2.0, 2.0)))
  );
  assert_eq!(
    F64Bound::new((2.0, 2.0)).intersect(F64Bound::new((1.0, 2.0))),
    Some(F64Bound::new((2.0, 2.0)))
  );
  assert_eq!(
    F64Bound::new((2.0, 2.0)).intersect(F64Bound::new((2.0, 2.0))),
    Some(F64Bound::new((2.0, 2.0)))
  );
}

#[test]
fn test_has() {
  assert!(F64Bound::new((0.0, 0.0)).has(0.0));
  assert!(F64Bound::new((0.0, 1.0)).has(0.0));
  assert!(F64Bound::new((0.0, 2.0)).has(0.0));
  assert!(!F64Bound::new((1.0, 2.0)).has(0.0));
  assert!(!F64Bound::new((2.0, 2.0)).has(0.0));
  assert!(!F64Bound::new((2.0, 2.0)).has(1.0));
  assert!(F64Bound::new((2.0, 2.0)).has(2.0));
}

#[test]
fn test_rand() {
  assert_eq!(F64Bound::new((0.0, 0.0)).rand(), 0.0);
  assert!(F64Bound::new((0.0, 1.0)).rand() >= 0.0);
  assert!(F64Bound::new((0.0, 1.0)).rand() <= 1.0);
  assert!(F64Bound::new((0.0, 2.0)).rand() >= 0.0);
  assert!(F64Bound::new((0.0, 2.0)).rand() <= 2.0);
  assert!(F64Bound::new((1.0, 2.0)).rand() >= 1.0);
  assert!(F64Bound::new((1.0, 2.0)).rand() <= 2.0);
  assert_eq!(F64Bound::new((2.0, 2.0)).rand(), 2.0);
}

#[test]
fn test_into_f64_seq() {
  assert_eq!(
    F64Seq::from(F64Bound::new((0.0, 1.0))),
    F64Seq::new((0.0, 1.0))
  );
  assert_eq!(
    F64Seq::from(F64Bound::new((0.0, 2.0))),
    F64Seq::new((0.0, 2.0))
  );
  assert_eq!(
    F64Seq::from(F64Bound::new((1.0, 2.0))),
    F64Seq::new((1.0, 2.0))
  );
}

#[test]
fn test_into_u32_seq() {
  assert_eq!(I32Seq::from(F64Bound::new((0.0, 1.0))), I32Seq::new((0, 1)));
  assert_eq!(I32Seq::from(F64Bound::new((0.0, 2.0))), I32Seq::new((0, 2)));
  assert_eq!(I32Seq::from(F64Bound::new((1.0, 2.0))), I32Seq::new((1, 2)));
}

#[test]
fn test_into_u32_bound() {
  assert_eq!(
    I32Bound::from(F64Bound::new((0.0, 1.0))),
    I32Bound::new((0, 1))
  );
  assert_eq!(
    I32Bound::from(F64Bound::new((0.0, 2.0))),
    I32Bound::new((0, 2))
  );
  assert_eq!(
    I32Bound::from(F64Bound::new((1.0, 2.0))),
    I32Bound::new((1, 2))
  );
}
