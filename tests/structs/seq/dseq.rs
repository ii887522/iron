use iron_ingot::{DBound, DSeq, FBound, FSeq, IBound, UBound};

#[test]
fn test_normalize() {
  assert_eq!(DSeq::new(0.0, 1.0).normalize(0.0), 0.0);
  assert_eq!(DSeq::new(0.0, 2.0).normalize(0.0), 0.0);
  assert_eq!(DSeq::new(0.0, 5.0).normalize(0.0), 0.0);
  assert_eq!(DSeq::new(0.0, 5.0).normalize(1.0), 0.2);
  assert_eq!(DSeq::new(0.0, 5.0).normalize(2.0), 0.4);
  assert_eq!(DSeq::new(1.0, 5.0).normalize(2.0), 0.25);
  assert_eq!(DSeq::new(2.0, 5.0).normalize(2.0), 0.0);
}

#[test]
fn test_unnormalize() {
  assert_eq!(DSeq::new(0.0, 1.0).unnormalize(0.0), 0.0);
  assert_eq!(DSeq::new(0.0, 2.0).unnormalize(0.0), 0.0);
  assert_eq!(DSeq::new(0.0, 5.0).unnormalize(0.0), 0.0);
  assert_eq!(DSeq::new(0.0, 5.0).unnormalize(0.2), 1.0);
  assert_eq!(DSeq::new(0.0, 5.0).unnormalize(0.4), 2.0);
  assert_eq!(DSeq::new(1.0, 5.0).unnormalize(0.25), 2.0);
  assert_eq!(DSeq::new(2.0, 5.0).unnormalize(0.0), 2.0);
}

#[test]
fn test_into_fseq() {
  assert_eq!(FSeq::from(DSeq::new(0.0, 1.0)), FSeq::new(0.0, 1.0));
  assert_eq!(FSeq::from(DSeq::new(0.0, 2.0)), FSeq::new(0.0, 2.0));
  assert_eq!(FSeq::from(DSeq::new(0.0, 3.0)), FSeq::new(0.0, 3.0));
  assert_eq!(FSeq::from(DSeq::new(1.0, 3.0)), FSeq::new(1.0, 3.0));
  assert_eq!(FSeq::from(DSeq::new(2.0, 3.0)), FSeq::new(2.0, 3.0));
}

#[test]
fn test_into_dbound() {
  assert_eq!(DBound::from(DSeq::new(0.0, 1.0)), DBound::new((0.0, 1.0)));
  assert_eq!(DBound::from(DSeq::new(0.0, 2.0)), DBound::new((0.0, 2.0)));
  assert_eq!(DBound::from(DSeq::new(0.0, 3.0)), DBound::new((0.0, 3.0)));
  assert_eq!(DBound::from(DSeq::new(1.0, 3.0)), DBound::new((1.0, 3.0)));
  assert_eq!(DBound::from(DSeq::new(2.0, 3.0)), DBound::new((2.0, 3.0)));
}

#[test]
fn test_into_fbound() {
  assert_eq!(FBound::from(DSeq::new(0.0, 1.0)), FBound::new((0.0, 1.0)));
  assert_eq!(FBound::from(DSeq::new(0.0, 2.0)), FBound::new((0.0, 2.0)));
  assert_eq!(FBound::from(DSeq::new(0.0, 3.0)), FBound::new((0.0, 3.0)));
  assert_eq!(FBound::from(DSeq::new(1.0, 3.0)), FBound::new((1.0, 3.0)));
  assert_eq!(FBound::from(DSeq::new(2.0, 3.0)), FBound::new((2.0, 3.0)));
}

#[test]
fn test_into_ibound() {
  assert_eq!(IBound::from(DSeq::new(0.0, 1.0)), IBound::new((0, 1)));
  assert_eq!(IBound::from(DSeq::new(0.0, 2.0)), IBound::new((0, 2)));
  assert_eq!(IBound::from(DSeq::new(0.0, 3.0)), IBound::new((0, 3)));
  assert_eq!(IBound::from(DSeq::new(1.0, 3.0)), IBound::new((1, 3)));
  assert_eq!(IBound::from(DSeq::new(2.0, 3.0)), IBound::new((2, 3)));
}

#[test]
fn test_into_ubound() {
  assert_eq!(UBound::from(DSeq::new(0.0, 1.0)), UBound::new((0, 1)));
  assert_eq!(UBound::from(DSeq::new(0.0, 2.0)), UBound::new((0, 2)));
  assert_eq!(UBound::from(DSeq::new(0.0, 3.0)), UBound::new((0, 3)));
  assert_eq!(UBound::from(DSeq::new(1.0, 3.0)), UBound::new((1, 3)));
  assert_eq!(UBound::from(DSeq::new(2.0, 3.0)), UBound::new((2, 3)));
}
