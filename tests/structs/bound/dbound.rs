use iron_ingot::{DBound, DSeq, FBound, FSeq, IBound, UBound};

#[test]
fn test_get_middle() {
  assert_eq!(DBound::new((0.0, 0.0)).get_middle(), 0.0);
  assert_eq!(DBound::new((0.0, 1.0)).get_middle(), 0.5);
  assert_eq!(DBound::new((0.0, 2.0)).get_middle(), 1.0);
  assert_eq!(DBound::new((1.0, 2.0)).get_middle(), 1.5);
  assert_eq!(DBound::new((2.0, 2.0)).get_middle(), 2.0);
}

#[test]
fn test_is_intersect() {
  assert!(DBound::new((0.0, 0.0)).is_intersect(DBound::new((0.0, 0.0))));
  assert!(DBound::new((0.0, 1.0)).is_intersect(DBound::new((0.0, 0.0))));
  assert!(DBound::new((0.0, 2.0)).is_intersect(DBound::new((0.0, 0.0))));
  assert!(!DBound::new((1.0, 2.0)).is_intersect(DBound::new((0.0, 0.0))));
  assert!(!DBound::new((2.0, 2.0)).is_intersect(DBound::new((0.0, 0.0))));
  assert!(!DBound::new((2.0, 2.0)).is_intersect(DBound::new((0.0, 1.0))));
  assert!(DBound::new((2.0, 2.0)).is_intersect(DBound::new((0.0, 2.0))));
  assert!(DBound::new((2.0, 2.0)).is_intersect(DBound::new((1.0, 2.0))));
  assert!(DBound::new((2.0, 2.0)).is_intersect(DBound::new((2.0, 2.0))));
}

#[test]
fn test_intersect() {
  assert_eq!(
    DBound::new((0.0, 0.0)).intersect(DBound::new((0.0, 0.0))),
    Some(DBound::new((0.0, 0.0)))
  );
  assert_eq!(
    DBound::new((0.0, 1.0)).intersect(DBound::new((0.0, 0.0))),
    Some(DBound::new((0.0, 0.0)))
  );
  assert_eq!(
    DBound::new((0.0, 2.0)).intersect(DBound::new((0.0, 0.0))),
    Some(DBound::new((0.0, 0.0)))
  );
  assert_eq!(
    DBound::new((1.0, 2.0)).intersect(DBound::new((0.0, 0.0))),
    None
  );
  assert_eq!(
    DBound::new((2.0, 2.0)).intersect(DBound::new((0.0, 0.0))),
    None
  );
  assert_eq!(
    DBound::new((2.0, 2.0)).intersect(DBound::new((0.0, 1.0))),
    None
  );
  assert_eq!(
    DBound::new((2.0, 2.0)).intersect(DBound::new((0.0, 2.0))),
    Some(DBound::new((2.0, 2.0)))
  );
  assert_eq!(
    DBound::new((2.0, 2.0)).intersect(DBound::new((1.0, 2.0))),
    Some(DBound::new((2.0, 2.0)))
  );
  assert_eq!(
    DBound::new((2.0, 2.0)).intersect(DBound::new((2.0, 2.0))),
    Some(DBound::new((2.0, 2.0)))
  );
}

#[test]
fn test_has() {
  assert!(DBound::new((0.0, 0.0)).has(0.0));
  assert!(DBound::new((0.0, 1.0)).has(0.0));
  assert!(DBound::new((0.0, 2.0)).has(0.0));
  assert!(!DBound::new((1.0, 2.0)).has(0.0));
  assert!(!DBound::new((2.0, 2.0)).has(0.0));
  assert!(!DBound::new((2.0, 2.0)).has(1.0));
  assert!(DBound::new((2.0, 2.0)).has(2.0));
}

#[test]
fn test_rand() {
  assert_eq!(DBound::new((0.0, 0.0)).rand(), 0.0);
  assert!(DBound::new((0.0, 1.0)).rand() >= 0.0);
  assert!(DBound::new((0.0, 1.0)).rand() <= 1.0);
  assert!(DBound::new((0.0, 2.0)).rand() >= 0.0);
  assert!(DBound::new((0.0, 2.0)).rand() <= 2.0);
  assert!(DBound::new((1.0, 2.0)).rand() >= 1.0);
  assert!(DBound::new((1.0, 2.0)).rand() <= 2.0);
  assert_eq!(DBound::new((2.0, 2.0)).rand(), 2.0);
}

#[test]
fn test_clamp() {
  assert_eq!(DBound::new((0.0, 0.0)).clamp(0.0), 0.0);
  assert_eq!(DBound::new((0.0, 1.0)).clamp(0.0), 0.0);
  assert_eq!(DBound::new((0.0, 2.0)).clamp(0.0), 0.0);
  assert_eq!(DBound::new((1.0, 2.0)).clamp(0.0), 1.0);
  assert_eq!(DBound::new((2.0, 2.0)).clamp(0.0), 2.0);
  assert_eq!(DBound::new((2.0, 3.0)).clamp(0.0), 2.0);
  assert_eq!(DBound::new((2.0, 4.0)).clamp(0.0), 2.0);
  assert_eq!(DBound::new((2.0, 4.0)).clamp(1.0), 2.0);
  assert_eq!(DBound::new((2.0, 4.0)).clamp(2.0), 2.0);
  assert_eq!(DBound::new((2.0, 4.0)).clamp(3.0), 3.0);
  assert_eq!(DBound::new((2.0, 4.0)).clamp(4.0), 4.0);
  assert_eq!(DBound::new((2.0, 4.0)).clamp(5.0), 4.0);
  assert_eq!(DBound::new((2.0, 4.0)).clamp(6.0), 4.0);
}

#[test]
fn test_into_fbound() {
  assert_eq!(
    FBound::from(DBound::new((0.0, 1.0))),
    FBound::new((0.0, 1.0))
  );
  assert_eq!(
    FBound::from(DBound::new((0.0, 2.0))),
    FBound::new((0.0, 2.0))
  );
  assert_eq!(
    FBound::from(DBound::new((1.0, 2.0))),
    FBound::new((1.0, 2.0))
  );
}

#[test]
fn test_into_ibound() {
  assert_eq!(IBound::from(DBound::new((0.0, 1.0))), IBound::new((0, 1)));
  assert_eq!(IBound::from(DBound::new((0.0, 2.0))), IBound::new((0, 2)));
  assert_eq!(IBound::from(DBound::new((1.0, 2.0))), IBound::new((1, 2)));
}

#[test]
fn test_into_ubound() {
  assert_eq!(UBound::from(DBound::new((0.0, 1.0))), UBound::new((0, 1)));
  assert_eq!(UBound::from(DBound::new((0.0, 2.0))), UBound::new((0, 2)));
  assert_eq!(UBound::from(DBound::new((1.0, 2.0))), UBound::new((1, 2)));
}

#[test]
fn test_into_dseq() {
  assert_eq!(DSeq::from(DBound::new((0.0, 1.0))), DSeq::new(0.0, 1.0));
  assert_eq!(DSeq::from(DBound::new((0.0, 2.0))), DSeq::new(0.0, 2.0));
  assert_eq!(DSeq::from(DBound::new((1.0, 2.0))), DSeq::new(1.0, 2.0));
}

#[test]
fn test_into_fseq() {
  assert_eq!(FSeq::from(DBound::new((0.0, 1.0))), FSeq::new(0.0, 1.0));
  assert_eq!(FSeq::from(DBound::new((0.0, 2.0))), FSeq::new(0.0, 2.0));
  assert_eq!(FSeq::from(DBound::new((1.0, 2.0))), FSeq::new(1.0, 2.0));
}
