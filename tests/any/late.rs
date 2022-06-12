use iron_ingot::Late;

#[test]
fn test_late() {
  let mut late = Late::<Box<i32>>::new();
  late.set(Box::new(0));
  assert_eq!(*late, Box::new(0));
  **late = 1;
  assert_eq!(*late, Box::new(1));
}
