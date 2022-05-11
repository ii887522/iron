use iron_ingot::Dynamic;

#[test]
fn step_dynamic_value_when_value_keeps_increasing() {
  let mut value = 0;
  let mut dynamic_value = Dynamic::new(|| {
    value += 1;
    value
  });
  dynamic_value.step(0.0);
  assert_eq!(dynamic_value.get_value(), &1);
  dynamic_value.step(1.0);
  assert_eq!(dynamic_value.get_value(), &2);
  dynamic_value.step(2.0);
  assert_eq!(dynamic_value.get_value(), &3);
  dynamic_value.step(3.0);
  assert_eq!(dynamic_value.get_value(), &4);
  dynamic_value.step(4.0);
  assert_eq!(dynamic_value.get_value(), &5);
}

#[test]
fn step_dynamic_value_when_value_keeps_decreasing() {
  let mut value = 0;
  let mut dynamic_value = Dynamic::new(|| {
    value -= 1;
    value
  });
  dynamic_value.step(0.0);
  assert_eq!(dynamic_value.get_value(), &-1);
  dynamic_value.step(1.0);
  assert_eq!(dynamic_value.get_value(), &-2);
  dynamic_value.step(2.0);
  assert_eq!(dynamic_value.get_value(), &-3);
  dynamic_value.step(3.0);
  assert_eq!(dynamic_value.get_value(), &-4);
  dynamic_value.step(4.0);
  assert_eq!(dynamic_value.get_value(), &-5);
}

#[test]
fn step_dynamic_value_when_value_keeps_decreasing_and_interval_is_10() {
  let mut value = 0;
  let mut dynamic_value = Dynamic::new((
    || {
      value -= 1;
      value
    },
    10.0,
  ));
  dynamic_value.step(0.0);
  assert_eq!(dynamic_value.get_value(), &-1);
  dynamic_value.step(3.0);
  assert_eq!(dynamic_value.get_value(), &-1);
  dynamic_value.step(7.0);
  assert_eq!(dynamic_value.get_value(), &-2);
  dynamic_value.step(10.0);
  assert_eq!(dynamic_value.get_value(), &-3);
  dynamic_value.step(20.0);
  assert_eq!(dynamic_value.get_value(), &-4);
}
