extern crate iron;

use iron::any::reactive;
use iron::ReactivityManager;

#[test]
pub fn test_watch() {
  let mut reactivity_manager = ReactivityManager::new();
  let value = reactive::Handle::from(0);
  let squared_value = reactivity_manager.watch(value.clone(), |&value| value * value);
  assert_eq!(squared_value.borrow().get_value(), &0);
  value.borrow_mut().set_value(1);
  assert_eq!(squared_value.borrow().get_value(), &1);
  value.borrow_mut().set_value(2);
  assert_eq!(squared_value.borrow().get_value(), &4);
  let value_str = reactivity_manager.watch(value.clone(), |&value| format!("{value}"));
  assert_eq!(value_str.borrow().get_value(), &"2".to_owned());
  value.borrow_mut().set_value(3);
  assert_eq!(value_str.borrow().get_value(), &"3".to_owned());
  value.borrow_mut().set_value(4);
  assert_eq!(value_str.borrow().get_value(), &"4".to_owned());
  let string = reactive::Handle::from("a");
  let repeated_str = reactivity_manager.watch(string.clone(), |&value| format!("{value}{value}"));
  assert_eq!(repeated_str.borrow().get_value(), &"aa".to_owned());
  string.borrow_mut().set_value("b");
  assert_eq!(repeated_str.borrow().get_value(), &"bb".to_owned());
  string.borrow_mut().set_value("1");
  assert_eq!(repeated_str.borrow().get_value(), &"11".to_owned());
  let number = reactivity_manager.watch(string.clone(), |&value| value.parse::<i32>());
  assert_eq!(number.borrow().get_value(), &Ok(1));
  string.borrow_mut().set_value("2");
  assert_eq!(number.borrow().get_value(), &Ok(2));
  string.borrow_mut().set_value("3");
  assert_eq!(number.borrow().get_value(), &Ok(3));
}

#[test]
fn test_watch2() {
  let mut reactivity_manager = ReactivityManager::new();
  let a = reactive::Handle::from(0);
  let b = reactive::Handle::from(0);
  let sum = reactivity_manager.watch2(a.clone(), b.clone(), |&a, &b| a + b);
  assert_eq!(sum.borrow().get_value(), &0);
  a.borrow_mut().set_value(1);
  assert_eq!(sum.borrow().get_value(), &1);
  b.borrow_mut().set_value(2);
  assert_eq!(sum.borrow().get_value(), &3);
  a.borrow_mut().set_value(3);
  assert_eq!(sum.borrow().get_value(), &5);
  b.borrow_mut().set_value(4);
  assert_eq!(sum.borrow().get_value(), &7);
  let diff = reactivity_manager.watch2(a.clone(), b.clone(), |&a, &b| a - b);
  assert_eq!(diff.borrow().get_value(), &-1);
  a.borrow_mut().set_value(5);
  assert_eq!(diff.borrow().get_value(), &1);
  b.borrow_mut().set_value(3);
  assert_eq!(diff.borrow().get_value(), &2);
  a.borrow_mut().set_value(6);
  assert_eq!(diff.borrow().get_value(), &3);
  b.borrow_mut().set_value(2);
  assert_eq!(diff.borrow().get_value(), &4);
  let c = reactive::Handle::from("a");
  let d = reactive::Handle::from("b");
  let cd = reactivity_manager.watch2(c.clone(), d.clone(), |&c, &d| format!("{c}{d}"));
  assert_eq!(cd.borrow().get_value(), &"ab".to_owned());
  c.borrow_mut().set_value("b");
  assert_eq!(cd.borrow().get_value(), &"bb".to_owned());
  d.borrow_mut().set_value("c");
  assert_eq!(cd.borrow().get_value(), &"bc".to_owned());
  c.borrow_mut().set_value("d");
  assert_eq!(cd.borrow().get_value(), &"dc".to_owned());
  d.borrow_mut().set_value("e");
  assert_eq!(cd.borrow().get_value(), &"de".to_owned());
  let min_str = reactivity_manager.watch2(c.clone(), d.clone(), |&c, &d| if c < d { c } else { d });
  assert_eq!(min_str.borrow().get_value(), &"d".to_owned());
  c.borrow_mut().set_value("f");
  assert_eq!(min_str.borrow().get_value(), &"e".to_owned());
  d.borrow_mut().set_value("g");
  assert_eq!(min_str.borrow().get_value(), &"f".to_owned());
  c.borrow_mut().set_value("h");
  assert_eq!(min_str.borrow().get_value(), &"g".to_owned());
  d.borrow_mut().set_value("i");
  assert_eq!(min_str.borrow().get_value(), &"h".to_owned());
}

#[test]
fn test_watch3() {
  let mut reactivity_manager = ReactivityManager::new();
  let a = reactive::Handle::from(0);
  let b = reactive::Handle::from(0);
  let c = reactive::Handle::from(0);
  let sum = reactivity_manager.watch3(a.clone(), b.clone(), c.clone(), |&a, &b, &c| a + b + c);
  assert_eq!(sum.borrow().get_value(), &0);
  a.borrow_mut().set_value(1);
  assert_eq!(sum.borrow().get_value(), &1);
  b.borrow_mut().set_value(2);
  assert_eq!(sum.borrow().get_value(), &3);
  c.borrow_mut().set_value(3);
  assert_eq!(sum.borrow().get_value(), &6);
  a.borrow_mut().set_value(4);
  assert_eq!(sum.borrow().get_value(), &9);
  b.borrow_mut().set_value(5);
  assert_eq!(sum.borrow().get_value(), &12);
  c.borrow_mut().set_value(6);
  assert_eq!(sum.borrow().get_value(), &15);
  let diff = reactivity_manager.watch3(a.clone(), b.clone(), c.clone(), |&a, &b, &c| a - b - c);
  assert_eq!(diff.borrow().get_value(), &-7);
  a.borrow_mut().set_value(7);
  assert_eq!(diff.borrow().get_value(), &-4);
  b.borrow_mut().set_value(8);
  assert_eq!(diff.borrow().get_value(), &-7);
  c.borrow_mut().set_value(9);
  assert_eq!(diff.borrow().get_value(), &-10);
  a.borrow_mut().set_value(10);
  assert_eq!(diff.borrow().get_value(), &-7);
  b.borrow_mut().set_value(11);
  assert_eq!(diff.borrow().get_value(), &-10);
  c.borrow_mut().set_value(12);
  assert_eq!(diff.borrow().get_value(), &-13);
  let d = reactive::Handle::from("a");
  let e = reactive::Handle::from("b");
  let f = reactive::Handle::from("c");
  let def = reactivity_manager.watch3(d.clone(), e.clone(), f.clone(), |&d, &e, &f| {
    format!("{d}{e}{f}")
  });
  assert_eq!(def.borrow().get_value(), &"abc".to_owned());
  d.borrow_mut().set_value("d");
  assert_eq!(def.borrow().get_value(), &"dbc".to_owned());
  e.borrow_mut().set_value("e");
  assert_eq!(def.borrow().get_value(), &"dec".to_owned());
  f.borrow_mut().set_value("f");
  assert_eq!(def.borrow().get_value(), &"def".to_owned());
  d.borrow_mut().set_value("g");
  assert_eq!(def.borrow().get_value(), &"gef".to_owned());
  e.borrow_mut().set_value("h");
  assert_eq!(def.borrow().get_value(), &"ghf".to_owned());
  f.borrow_mut().set_value("i");
  assert_eq!(def.borrow().get_value(), &"ghi".to_owned());
  let min_str = reactivity_manager.watch3(d.clone(), e.clone(), f.clone(), |&d, &e, &f| {
    if d < e {
      if d < f {
        d
      } else {
        f
      }
    } else if e < f {
      e
    } else {
      f
    }
  });
  assert_eq!(min_str.borrow().get_value(), &"g".to_owned());
  d.borrow_mut().set_value("j");
  assert_eq!(min_str.borrow().get_value(), &"h".to_owned());
  e.borrow_mut().set_value("k");
  assert_eq!(min_str.borrow().get_value(), &"i".to_owned());
  f.borrow_mut().set_value("l");
  assert_eq!(min_str.borrow().get_value(), &"j".to_owned());
  d.borrow_mut().set_value("m");
  assert_eq!(min_str.borrow().get_value(), &"k".to_owned());
  e.borrow_mut().set_value("n");
  assert_eq!(min_str.borrow().get_value(), &"l".to_owned());
  f.borrow_mut().set_value("o");
  assert_eq!(min_str.borrow().get_value(), &"m".to_owned());
}
