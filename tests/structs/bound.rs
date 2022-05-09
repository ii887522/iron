extern crate iron;

use iron::Bound;
use iron::Seq;

#[test]
fn test_get_middle() {
  assert_eq!(Bound::new((0.0, 0.0)).get_middle(), 0.0);
  assert_eq!(Bound::new((0.0, 1.0)).get_middle(), 0.5);
  assert_eq!(Bound::new((0.0, 2.0)).get_middle(), 1.0);
  assert_eq!(Bound::new((1.0, 2.0)).get_middle(), 1.5);
  assert_eq!(Bound::new((2.0, 2.0)).get_middle(), 2.0);
}

#[test]
fn test_is_intersect() {
  assert!(Bound::new((0.0, 0.0)).is_intersect(Bound::new((0.0, 0.0))));
  assert!(Bound::new((0.0, 1.0)).is_intersect(Bound::new((0.0, 0.0))));
  assert!(Bound::new((0.0, 2.0)).is_intersect(Bound::new((0.0, 0.0))));
  assert!(!Bound::new((1.0, 2.0)).is_intersect(Bound::new((0.0, 0.0))));
  assert!(!Bound::new((2.0, 2.0)).is_intersect(Bound::new((0.0, 0.0))));
  assert!(!Bound::new((2.0, 2.0)).is_intersect(Bound::new((0.0, 1.0))));
  assert!(Bound::new((2.0, 2.0)).is_intersect(Bound::new((0.0, 2.0))));
  assert!(Bound::new((2.0, 2.0)).is_intersect(Bound::new((1.0, 2.0))));
  assert!(Bound::new((2.0, 2.0)).is_intersect(Bound::new((2.0, 2.0))));
}

#[test]
fn test_intersect() {
  assert_eq!(
    Bound::new((0.0, 0.0)).intersect(Bound::new((0.0, 0.0))),
    Some(Bound::new((0.0, 0.0)))
  );
  assert_eq!(
    Bound::new((0.0, 1.0)).intersect(Bound::new((0.0, 0.0))),
    Some(Bound::new((0.0, 0.0)))
  );
  assert_eq!(
    Bound::new((0.0, 2.0)).intersect(Bound::new((0.0, 0.0))),
    Some(Bound::new((0.0, 0.0)))
  );
  assert_eq!(
    Bound::new((1.0, 2.0)).intersect(Bound::new((0.0, 0.0))),
    None
  );
  assert_eq!(
    Bound::new((2.0, 2.0)).intersect(Bound::new((0.0, 0.0))),
    None
  );
  assert_eq!(
    Bound::new((2.0, 2.0)).intersect(Bound::new((0.0, 1.0))),
    None
  );
  assert_eq!(
    Bound::new((2.0, 2.0)).intersect(Bound::new((0.0, 2.0))),
    Some(Bound::new((2.0, 2.0)))
  );
  assert_eq!(
    Bound::new((2.0, 2.0)).intersect(Bound::new((1.0, 2.0))),
    Some(Bound::new((2.0, 2.0)))
  );
  assert_eq!(
    Bound::new((2.0, 2.0)).intersect(Bound::new((2.0, 2.0))),
    Some(Bound::new((2.0, 2.0)))
  );
}

#[test]
fn test_clamp() {
  assert_eq!(Bound::new((0.0, 0.0)).clamp(0.0), 0.0);
  assert_eq!(Bound::new((0.0, 1.0)).clamp(0.0), 0.0);
  assert_eq!(Bound::new((0.0, 2.0)).clamp(0.0), 0.0);
  assert_eq!(Bound::new((1.0, 2.0)).clamp(0.0), 1.0);
  assert_eq!(Bound::new((2.0, 2.0)).clamp(0.0), 2.0);
  assert_eq!(Bound::new((2.0, 4.0)).clamp(1.0), 2.0);
  assert_eq!(Bound::new((2.0, 4.0)).clamp(2.0), 2.0);
  assert_eq!(Bound::new((2.0, 4.0)).clamp(3.0), 3.0);
  assert_eq!(Bound::new((2.0, 4.0)).clamp(4.0), 4.0);
  assert_eq!(Bound::new((2.0, 4.0)).clamp(5.0), 4.0);
}

#[test]
fn test_rand() {
  assert_eq!(Bound::new((0.0, 0.0)).rand(), 0.0);
  assert!(Bound::new((0.0, 1.0)).rand() >= 0.0);
  assert!(Bound::new((0.0, 1.0)).rand() <= 1.0);
  assert!(Bound::new((0.0, 2.0)).rand() >= 0.0);
  assert!(Bound::new((0.0, 2.0)).rand() <= 2.0);
  assert!(Bound::new((1.0, 2.0)).rand() >= 1.0);
  assert!(Bound::new((1.0, 2.0)).rand() <= 2.0);
  assert_eq!(Bound::new((2.0, 2.0)).rand(), 2.0);
}

#[test]
fn test_into_seq() {
  assert_eq!(Seq::from(Bound::new((0.0, 0.0))), Seq::new((0.0, 0.0)));
  assert_eq!(Seq::from(Bound::new((0.0, 1.0))), Seq::new((0.0, 1.0)));
  assert_eq!(Seq::from(Bound::new((0.0, 2.0))), Seq::new((0.0, 2.0)));
  assert_eq!(Seq::from(Bound::new((1.0, 2.0))), Seq::new((1.0, 2.0)));
  assert_eq!(Seq::from(Bound::new((2.0, 2.0))), Seq::new((2.0, 2.0)));
}
