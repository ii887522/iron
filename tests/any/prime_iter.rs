use iron::PrimeIter;
use std::borrow::Cow;

#[test]
fn test_next_when_starts_from_0() {
  assert_eq!(
    &*PrimeIter::new(()).take(8).collect::<Cow<_>>(),
    &[2, 3, 5, 7, 11, 13, 17, 19]
  );
  assert_eq!(
    &*PrimeIter::new(2).take(8).collect::<Cow<_>>(),
    &[3, 5, 7, 11, 13, 17, 19, 23]
  );
  assert_eq!(
    &*PrimeIter::new(3).take(8).collect::<Cow<_>>(),
    &[5, 7, 11, 13, 17, 19, 23, 29]
  );
  assert_eq!(
    &*PrimeIter::new(4).take(8).collect::<Cow<_>>(),
    &[5, 7, 11, 13, 17, 19, 23, 29]
  );
  assert_eq!(
    &*PrimeIter::new(5).take(8).collect::<Cow<_>>(),
    &[7, 11, 13, 17, 19, 23, 29, 31]
  );
}
