use iron_ingot::{DBound, DSeq, FBound, FSeq, IBound, UBound};

#[test]
fn test_get_middle() {
  assert_eq!(FBound::new((0.0, 0.0)).get_middle(), 0.0);
  assert_eq!(FBound::new((0.0, 1.0)).get_middle(), 0.5);
  assert_eq!(FBound::new((0.0, 2.0)).get_middle(), 1.0);
  assert_eq!(FBound::new((1.0, 2.0)).get_middle(), 1.5);
  assert_eq!(FBound::new((2.0, 2.0)).get_middle(), 2.0);
}

#[test]
fn test_is_intersect() {
  assert!(FBound::new((0.0, 0.0)).is_intersect(FBound::new((0.0, 0.0))));
  assert!(FBound::new((0.0, 1.0)).is_intersect(FBound::new((0.0, 0.0))));
  assert!(FBound::new((0.0, 2.0)).is_intersect(FBound::new((0.0, 0.0))));
  assert!(!FBound::new((1.0, 2.0)).is_intersect(FBound::new((0.0, 0.0))));
  assert!(!FBound::new((2.0, 2.0)).is_intersect(FBound::new((0.0, 0.0))));
  assert!(!FBound::new((2.0, 2.0)).is_intersect(FBound::new((0.0, 1.0))));
  assert!(FBound::new((2.0, 2.0)).is_intersect(FBound::new((0.0, 2.0))));
  assert!(FBound::new((2.0, 2.0)).is_intersect(FBound::new((1.0, 2.0))));
  assert!(FBound::new((2.0, 2.0)).is_intersect(FBound::new((2.0, 2.0))));
}

#[test]
fn test_intersect() {
  assert_eq!(
    FBound::new((0.0, 0.0)).intersect(FBound::new((0.0, 0.0))),
    Some(FBound::new((0.0, 0.0)))
  );
  assert_eq!(
    FBound::new((0.0, 1.0)).intersect(FBound::new((0.0, 0.0))),
    Some(FBound::new((0.0, 0.0)))
  );
  assert_eq!(
    FBound::new((0.0, 2.0)).intersect(FBound::new((0.0, 0.0))),
    Some(FBound::new((0.0, 0.0)))
  );
  assert_eq!(
    FBound::new((1.0, 2.0)).intersect(FBound::new((0.0, 0.0))),
    None
  );
  assert_eq!(
    FBound::new((2.0, 2.0)).intersect(FBound::new((0.0, 0.0))),
    None
  );
  assert_eq!(
    FBound::new((2.0, 2.0)).intersect(FBound::new((0.0, 1.0))),
    None
  );
  assert_eq!(
    FBound::new((2.0, 2.0)).intersect(FBound::new((0.0, 2.0))),
    Some(FBound::new((2.0, 2.0)))
  );
  assert_eq!(
    FBound::new((2.0, 2.0)).intersect(FBound::new((1.0, 2.0))),
    Some(FBound::new((2.0, 2.0)))
  );
  assert_eq!(
    FBound::new((2.0, 2.0)).intersect(FBound::new((2.0, 2.0))),
    Some(FBound::new((2.0, 2.0)))
  );
}

#[test]
fn test_has() {
  assert!(FBound::new((0.0, 0.0)).has(0.0));
  assert!(FBound::new((0.0, 1.0)).has(0.0));
  assert!(FBound::new((0.0, 2.0)).has(0.0));
  assert!(!FBound::new((1.0, 2.0)).has(0.0));
  assert!(!FBound::new((2.0, 2.0)).has(0.0));
  assert!(!FBound::new((2.0, 2.0)).has(1.0));
  assert!(FBound::new((2.0, 2.0)).has(2.0));
}

#[test]
fn test_rand() {
  assert_eq!(FBound::new((0.0, 0.0)).rand(), 0.0);
  assert!(FBound::new((0.0, 1.0)).rand() >= 0.0);
  assert!(FBound::new((0.0, 1.0)).rand() <= 1.0);
  assert!(FBound::new((0.0, 2.0)).rand() >= 0.0);
  assert!(FBound::new((0.0, 2.0)).rand() <= 2.0);
  assert!(FBound::new((1.0, 2.0)).rand() >= 1.0);
  assert!(FBound::new((1.0, 2.0)).rand() <= 2.0);
  assert_eq!(FBound::new((2.0, 2.0)).rand(), 2.0);
}

#[test]
fn test_into_dbound() {
  assert_eq!(
    DBound::from(FBound::new((0.0, 1.0))),
    DBound::new((0.0, 1.0))
  );
  assert_eq!(
    DBound::from(FBound::new((0.0, 2.0))),
    DBound::new((0.0, 2.0))
  );
  assert_eq!(
    DBound::from(FBound::new((1.0, 2.0))),
    DBound::new((1.0, 2.0))
  );
}

#[test]
fn test_into_ibound() {
  assert_eq!(IBound::from(FBound::new((0.0, 1.0))), IBound::new((0, 1)));
  assert_eq!(IBound::from(FBound::new((0.0, 2.0))), IBound::new((0, 2)));
  assert_eq!(IBound::from(FBound::new((1.0, 2.0))), IBound::new((1, 2)));
}

#[test]
fn test_into_ubound() {
  assert_eq!(UBound::from(FBound::new((0.0, 1.0))), UBound::new((0, 1)));
  assert_eq!(UBound::from(FBound::new((0.0, 2.0))), UBound::new((0, 2)));
  assert_eq!(UBound::from(FBound::new((1.0, 2.0))), UBound::new((1, 2)));
}

#[test]
fn test_into_dseq() {
  assert_eq!(DSeq::from(FBound::new((0.0, 1.0))), DSeq::new(0.0, 1.0));
  assert_eq!(DSeq::from(FBound::new((0.0, 2.0))), DSeq::new(0.0, 2.0));
  assert_eq!(DSeq::from(FBound::new((1.0, 2.0))), DSeq::new(1.0, 2.0));
}

#[test]
fn test_into_fseq() {
  assert_eq!(FSeq::from(FBound::new((0.0, 1.0))), FSeq::new(0.0, 1.0));
  assert_eq!(FSeq::from(FBound::new((0.0, 2.0))), FSeq::new(0.0, 2.0));
  assert_eq!(FSeq::from(FBound::new((1.0, 2.0))), FSeq::new(1.0, 2.0));
}
