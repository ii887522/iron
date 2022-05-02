extern crate iron;

use iron::Delayed;

#[test]
fn step_delayed_value_when_initial_value_is_0() {
  let mut delayed_value = Delayed::new(0);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 0);
  delayed_value.set_value(1);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 0);
  delayed_value.step(1.0);
  assert_eq!(delayed_value.get_value(), 1);
  delayed_value.step(2.0);
  assert_eq!(delayed_value.get_value(), 1);
  delayed_value.set_value(2);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 1);
  delayed_value.step(1.0);
  assert_eq!(delayed_value.get_value(), 2);
  delayed_value.step(2.0);
  assert_eq!(delayed_value.get_value(), 2);
  delayed_value.set_now(3);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 3);
  delayed_value.step(1.0);
  assert_eq!(delayed_value.get_value(), 3);
  delayed_value.set_now(4);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 4);
  delayed_value.step(1.0);
  assert_eq!(delayed_value.get_value(), 4);
}

#[test]
fn step_delayed_value_when_initial_value_is_5() {
  let mut delayed_value = Delayed::new(5);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 5);
  delayed_value.set_value(1);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 5);
  delayed_value.step(1.0);
  assert_eq!(delayed_value.get_value(), 1);
  delayed_value.step(2.0);
  assert_eq!(delayed_value.get_value(), 1);
  delayed_value.set_value(2);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 1);
  delayed_value.step(1.0);
  assert_eq!(delayed_value.get_value(), 2);
  delayed_value.step(2.0);
  assert_eq!(delayed_value.get_value(), 2);
  delayed_value.set_now(3);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 3);
  delayed_value.step(1.0);
  assert_eq!(delayed_value.get_value(), 3);
  delayed_value.set_now(4);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 4);
  delayed_value.step(1.0);
  assert_eq!(delayed_value.get_value(), 4);
}

#[test]
fn step_delayed_value_when_initial_value_is_6() {
  let mut delayed_value = Delayed::new(6);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 6);
  delayed_value.set_value(1);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 6);
  delayed_value.step(1.0);
  assert_eq!(delayed_value.get_value(), 1);
  delayed_value.step(2.0);
  assert_eq!(delayed_value.get_value(), 1);
  delayed_value.set_value(2);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 1);
  delayed_value.step(1.0);
  assert_eq!(delayed_value.get_value(), 2);
  delayed_value.step(2.0);
  assert_eq!(delayed_value.get_value(), 2);
  delayed_value.set_now(3);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 3);
  delayed_value.step(1.0);
  assert_eq!(delayed_value.get_value(), 3);
  delayed_value.set_now(4);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 4);
  delayed_value.step(1.0);
  assert_eq!(delayed_value.get_value(), 4);
}

#[test]
fn step_delayed_value_when_initial_value_is_6_and_timeout_after_10_time_units() {
  let mut delayed_value = Delayed::new((6, 10.0));
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 6);
  delayed_value.set_value(1);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 6);
  delayed_value.step(2.0);
  assert_eq!(delayed_value.get_value(), 6);
  delayed_value.step(8.0);
  assert_eq!(delayed_value.get_value(), 1);
  delayed_value.step(10.0);
  assert_eq!(delayed_value.get_value(), 1);
  delayed_value.set_value(2);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 1);
  delayed_value.step(3.0);
  assert_eq!(delayed_value.get_value(), 1);
  delayed_value.step(9.0);
  assert_eq!(delayed_value.get_value(), 2);
  delayed_value.step(10.0);
  assert_eq!(delayed_value.get_value(), 2);
  delayed_value.set_now(3);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 3);
  delayed_value.step(1.0);
  assert_eq!(delayed_value.get_value(), 3);
  delayed_value.set_now(4);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 4);
  delayed_value.step(1.0);
  assert_eq!(delayed_value.get_value(), 4);
}

#[test]
fn step_delayed_value_when_initial_value_is_6_and_timeout_after_100_time_units() {
  let mut delayed_value = Delayed::new((6, 100.0));
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 6);
  delayed_value.set_value(1);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 6);
  delayed_value.step(20.0);
  assert_eq!(delayed_value.get_value(), 6);
  delayed_value.step(80.0);
  assert_eq!(delayed_value.get_value(), 1);
  delayed_value.step(100.0);
  assert_eq!(delayed_value.get_value(), 1);
  delayed_value.set_value(2);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 1);
  delayed_value.step(30.0);
  assert_eq!(delayed_value.get_value(), 1);
  delayed_value.step(90.0);
  assert_eq!(delayed_value.get_value(), 2);
  delayed_value.step(100.0);
  assert_eq!(delayed_value.get_value(), 2);
  delayed_value.set_now(3);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 3);
  delayed_value.step(1.0);
  assert_eq!(delayed_value.get_value(), 3);
  delayed_value.set_now(4);
  delayed_value.step(0.0);
  assert_eq!(delayed_value.get_value(), 4);
  delayed_value.step(1.0);
  assert_eq!(delayed_value.get_value(), 4);
}
