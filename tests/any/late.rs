use iron_ingot::Late;

#[test]
fn test_late() {
  let mut late_value = Late::new();
  assert!(late_value.get_value().is_err());
  assert!(late_value.set_value(0).is_ok());
  assert!(late_value.get_value().is_ok());
  assert_eq!(late_value.get_value().unwrap_or(&-1), &0);
  assert!(late_value.set_value(1).is_err());
}
