extern crate iron;

use iron::Reactive;

#[test]
fn watch_a_reactive_object_with_initial_value_equal_to_0() {
  let mut value = Reactive::from(Box::new(0));
  let value_squared = value.watch(move |value| Box::new(value * value));
  assert_eq!(value_squared.borrow().get_value(), &0);
  value.set_value(Box::new(1));
  assert_eq!(value_squared.borrow().get_value(), &1);
  value.set_value(Box::new(2));
  assert_eq!(value_squared.borrow().get_value(), &4);
  let value_str = value.watch(move |value| Box::new(format!("{value}")));
  assert_eq!(value_str.borrow().get_value(), &"2".to_owned());
  value.set_value(Box::new(3));
  assert_eq!(value_str.borrow().get_value(), &"3".to_owned());
  value.set_value(Box::new(4));
  assert_eq!(value_str.borrow().get_value(), &"4".to_owned());
  value.set_value(Box::new(4));
  assert_eq!(value_str.borrow().get_value(), &"4".to_owned());
}

#[test]
fn watch_a_reactive_object_with_initial_value_equal_to_a() {
  let mut string = Reactive::from(Box::new("a"));
  let repeated_string = string.watch(move |value| Box::new(format!("{value}{value}")));
  assert_eq!(repeated_string.borrow().get_value(), &"aa".to_owned());
  string.set_value(Box::new("b"));
  assert_eq!(repeated_string.borrow().get_value(), &"bb".to_owned());
  string.set_value(Box::new("1"));
  assert_eq!(repeated_string.borrow().get_value(), &"11".to_owned());
  let number = string.watch(move |&value| Box::new(value.parse::<i32>()));
  assert_eq!(number.borrow().get_value(), &Ok(1));
  string.set_value(Box::new("2"));
  assert_eq!(number.borrow().get_value(), &Ok(2));
  string.set_value(Box::new("21"));
  assert_eq!(number.borrow().get_value(), &Ok(21));
  string.set_value(Box::new("21"));
  assert_eq!(number.borrow().get_value(), &Ok(21));
}

#[test]
fn test_unwatch() {
  let mut value = Reactive::from(Box::new(0));
  let squared_value = value.watch(move |value| Box::new(value * value));
  assert_eq!(squared_value.borrow().get_value(), &0);
  value.set_value(Box::new(3));
  assert_eq!(squared_value.borrow().get_value(), &9);
  value.unwatch(squared_value);
}
