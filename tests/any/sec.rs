use iron::{NanoSec, Sec};

#[test]
fn test_from() {
  assert_eq!(Sec::from(NanoSec(0.0)), Sec(0.0));
  assert_eq!(Sec::from(NanoSec(1e+9)), Sec(1.0));
  assert_eq!(Sec::from(NanoSec(2e+9)), Sec(2.0));
}
