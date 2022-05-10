use iron::{Bound, Seq};

#[test]
fn test_normalize() {
  assert_eq!(Seq::new((0.0, 1.0)).normalize(0.0), 0.0);
  assert_eq!(Seq::new((0.0, 2.0)).normalize(0.0), 0.0);
  assert_eq!(Seq::new((0.0, 5.0)).normalize(0.0), 0.0);
  assert_eq!(Seq::new((0.0, 5.0)).normalize(1.0), 0.2);
  assert_eq!(Seq::new((0.0, 5.0)).normalize(2.0), 0.4);
  assert_eq!(Seq::new((1.0, 5.0)).normalize(2.0), 0.25);
  assert_eq!(Seq::new((2.0, 5.0)).normalize(2.0), 0.0);
}

#[test]
fn test_unnormalize() {
  assert_eq!(Seq::new((0.0, 1.0)).unnormalize(0.0), 0.0);
  assert_eq!(Seq::new((0.0, 2.0)).unnormalize(0.0), 0.0);
  assert_eq!(Seq::new((0.0, 5.0)).unnormalize(0.0), 0.0);
  assert_eq!(Seq::new((0.0, 5.0)).unnormalize(0.2), 1.0);
  assert_eq!(Seq::new((0.0, 5.0)).unnormalize(0.4), 2.0);
  assert_eq!(Seq::new((1.0, 5.0)).unnormalize(0.25), 2.0);
  assert_eq!(Seq::new((2.0, 5.0)).unnormalize(0.0), 2.0);
}

#[test]
fn test_into_bound() {
  assert_eq!(Bound::from(Seq::new((0.0, 1.0))), Bound::new((0.0, 1.0)));
  assert_eq!(Bound::from(Seq::new((0.0, 2.0))), Bound::new((0.0, 2.0)));
  assert_eq!(Bound::from(Seq::new((0.0, 3.0))), Bound::new((0.0, 3.0)));
  assert_eq!(Bound::from(Seq::new((1.0, 3.0))), Bound::new((1.0, 3.0)));
  assert_eq!(Bound::from(Seq::new((2.0, 3.0))), Bound::new((2.0, 3.0)));
}
