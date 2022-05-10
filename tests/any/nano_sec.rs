use iron::{NanoSec, Sec};

#[test]
fn test_from() {
  assert_eq!(NanoSec::from(Sec(0.0)), NanoSec(0.0));
  assert_eq!(NanoSec::from(Sec(1.0)), NanoSec(1e+9));
  assert_eq!(NanoSec::from(Sec(2.0)), NanoSec(2e+9));
}
