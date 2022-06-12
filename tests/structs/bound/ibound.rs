use iron_ingot::{DBound, DSeq, FBound, FSeq, IBound, UBound};

#[test]
fn test_get_middle() {
  assert_eq!(IBound::new((0, 0)).get_middle(), 0);
  assert_eq!(IBound::new((0, 1)).get_middle(), 0);
  assert_eq!(IBound::new((0, 2)).get_middle(), 1);
  assert_eq!(IBound::new((1, 2)).get_middle(), 1);
  assert_eq!(IBound::new((2, 2)).get_middle(), 2);
}

#[test]
fn test_is_intersect() {
  assert!(IBound::new((0, 0)).is_intersect(IBound::new((0, 0))));
  assert!(IBound::new((0, 1)).is_intersect(IBound::new((0, 0))));
  assert!(IBound::new((0, 2)).is_intersect(IBound::new((0, 0))));
  assert!(!IBound::new((1, 2)).is_intersect(IBound::new((0, 0))));
  assert!(!IBound::new((2, 2)).is_intersect(IBound::new((0, 0))));
  assert!(!IBound::new((2, 2)).is_intersect(IBound::new((0, 1))));
  assert!(IBound::new((2, 2)).is_intersect(IBound::new((0, 2))));
  assert!(IBound::new((2, 2)).is_intersect(IBound::new((1, 2))));
  assert!(IBound::new((2, 2)).is_intersect(IBound::new((2, 2))));
}

#[test]
fn test_intersect() {
  assert_eq!(
    IBound::new((0, 0)).intersect(IBound::new((0, 0))),
    Some(IBound::new((0, 0)))
  );
  assert_eq!(
    IBound::new((0, 1)).intersect(IBound::new((0, 0))),
    Some(IBound::new((0, 0)))
  );
  assert_eq!(
    IBound::new((0, 2)).intersect(IBound::new((0, 0))),
    Some(IBound::new((0, 0)))
  );
  assert_eq!(IBound::new((1, 2)).intersect(IBound::new((0, 0))), None);
  assert_eq!(IBound::new((2, 2)).intersect(IBound::new((0, 0))), None);
  assert_eq!(IBound::new((2, 2)).intersect(IBound::new((0, 1))), None);
  assert_eq!(
    IBound::new((2, 2)).intersect(IBound::new((0, 2))),
    Some(IBound::new((2, 2)))
  );
  assert_eq!(
    IBound::new((2, 2)).intersect(IBound::new((1, 2))),
    Some(IBound::new((2, 2)))
  );
  assert_eq!(
    IBound::new((2, 2)).intersect(IBound::new((2, 2))),
    Some(IBound::new((2, 2)))
  );
}

#[test]
fn test_has() {
  assert!(IBound::new((0, 0)).has(0));
  assert!(IBound::new((0, 1)).has(0));
  assert!(IBound::new((0, 2)).has(0));
  assert!(!IBound::new((1, 2)).has(0));
  assert!(!IBound::new((2, 2)).has(0));
  assert!(!IBound::new((2, 2)).has(1));
  assert!(IBound::new((2, 2)).has(2));
}

#[test]
fn test_rand() {
  assert_eq!(IBound::new((0, 0)).rand(), 0);
  assert!(IBound::new((0, 1)).rand() <= 1);
  assert!(IBound::new((0, 2)).rand() <= 2);
  assert!(IBound::new((1, 2)).rand() >= 1);
  assert!(IBound::new((1, 2)).rand() <= 2);
  assert_eq!(IBound::new((2, 2)).rand(), 2);
}

#[test]
fn test_clamp() {
  assert_eq!(IBound::new((0, 0)).clamp(0), 0);
  assert_eq!(IBound::new((0, 1)).clamp(0), 0);
  assert_eq!(IBound::new((0, 2)).clamp(0), 0);
  assert_eq!(IBound::new((1, 2)).clamp(0), 1);
  assert_eq!(IBound::new((2, 2)).clamp(0), 2);
  assert_eq!(IBound::new((2, 3)).clamp(0), 2);
  assert_eq!(IBound::new((2, 4)).clamp(0), 2);
  assert_eq!(IBound::new((2, 4)).clamp(1), 2);
  assert_eq!(IBound::new((2, 4)).clamp(2), 2);
  assert_eq!(IBound::new((2, 4)).clamp(3), 3);
  assert_eq!(IBound::new((2, 4)).clamp(4), 4);
  assert_eq!(IBound::new((2, 4)).clamp(5), 4);
  assert_eq!(IBound::new((2, 4)).clamp(6), 4);
}

#[test]
fn test_into_dbound() {
  assert_eq!(DBound::from(IBound::new((0, 1))), DBound::new((0.0, 1.0)));
  assert_eq!(DBound::from(IBound::new((0, 2))), DBound::new((0.0, 2.0)));
  assert_eq!(DBound::from(IBound::new((1, 2))), DBound::new((1.0, 2.0)));
}

#[test]
fn test_into_fbound() {
  assert_eq!(FBound::from(IBound::new((0, 1))), FBound::new((0.0, 1.0)));
  assert_eq!(FBound::from(IBound::new((0, 2))), FBound::new((0.0, 2.0)));
  assert_eq!(FBound::from(IBound::new((1, 2))), FBound::new((1.0, 2.0)));
}

#[test]
fn test_into_ubound() {
  assert_eq!(UBound::from(IBound::new((0, 1))), UBound::new((0, 1)));
  assert_eq!(UBound::from(IBound::new((0, 2))), UBound::new((0, 2)));
  assert_eq!(UBound::from(IBound::new((1, 2))), UBound::new((1, 2)));
}

#[test]
fn test_into_dseq() {
  assert_eq!(DSeq::from(IBound::new((0, 1))), DSeq::new(0.0, 1.0));
  assert_eq!(DSeq::from(IBound::new((0, 2))), DSeq::new(0.0, 2.0));
  assert_eq!(DSeq::from(IBound::new((1, 2))), DSeq::new(1.0, 2.0));
}

#[test]
fn test_into_fseq() {
  assert_eq!(FSeq::from(IBound::new((0, 1))), FSeq::new(0.0, 1.0));
  assert_eq!(FSeq::from(IBound::new((0, 2))), FSeq::new(0.0, 2.0));
  assert_eq!(FSeq::from(IBound::new((1, 2))), FSeq::new(1.0, 2.0));
}
