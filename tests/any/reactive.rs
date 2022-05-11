use iron_ingot::any::reactive;

#[test]
fn watch_a_reactive_object_with_initial_value_equal_to_0() {
  let value = reactive::Handle::from(0);
  let value_squared = value.borrow_mut().watch(|&value| value * value);
  assert_eq!(value_squared.borrow().get_value(), &0);
  value.borrow_mut().set_value(1);
  assert_eq!(value_squared.borrow().get_value(), &1);
  value.borrow_mut().set_value(2);
  assert_eq!(value_squared.borrow().get_value(), &4);
  let value_str = value.borrow_mut().watch(|&value| format!("{value}"));
  assert_eq!(value_str.borrow().get_value(), &"2".to_owned());
  value.borrow_mut().set_value(3);
  assert_eq!(value_str.borrow().get_value(), &"3".to_owned());
  value.borrow_mut().set_value(4);
  assert_eq!(value_str.borrow().get_value(), &"4".to_owned());
  value.borrow_mut().set_value(4);
  assert_eq!(value_str.borrow().get_value(), &"4".to_owned());
}

#[test]
fn watch_a_reactive_object_with_initial_value_equal_to_a() {
  let string = reactive::Handle::from("a");
  let repeated_str = string
    .borrow_mut()
    .watch(|&value| format!("{value}{value}"));
  assert_eq!(repeated_str.borrow().get_value(), &"aa".to_owned());
  string.borrow_mut().set_value("b");
  assert_eq!(repeated_str.borrow().get_value(), &"bb".to_owned());
  string.borrow_mut().set_value("1");
  assert_eq!(repeated_str.borrow().get_value(), &"11".to_owned());
  let number = string.borrow_mut().watch(|&value| value.parse::<i32>());
  assert_eq!(number.borrow().get_value(), &Ok(1));
  string.borrow_mut().set_value("2");
  assert_eq!(number.borrow().get_value(), &Ok(2));
  string.borrow_mut().set_value("21");
  assert_eq!(number.borrow().get_value(), &Ok(21));
  string.borrow_mut().set_value("21");
  assert_eq!(number.borrow().get_value(), &Ok(21));
}

#[test]
fn test_unwatch() {
  let value = reactive::Handle::from(0);
  let squared_value = value.borrow_mut().watch(|&value| value * value);
  assert_eq!(squared_value.borrow().get_value(), &0);
  value.borrow_mut().set_value(3);
  assert_eq!(squared_value.borrow().get_value(), &9);
  value.borrow_mut().unwatch(squared_value.clone());
  value.borrow_mut().set_value(4);
  assert_eq!(squared_value.borrow().get_value(), &9);
}
