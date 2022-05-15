use iron_ingot::{camel_case_str::ToCamelCase, snake_case_str::ToSnakeCase, SnakeCaseStr, ToWords};
use std::borrow::Cow;

#[test]
fn test_to_words() {
  assert_eq!(SnakeCaseStr::new("a").to_words(), Cow::Borrowed(&["a"]));
  assert_eq!(SnakeCaseStr::new("b").to_words(), Cow::Borrowed(&["b"]));
  assert_eq!(SnakeCaseStr::new("c").to_words(), Cow::Borrowed(&["c"]));
  assert_eq!(SnakeCaseStr::new("ca").to_words(), Cow::Borrowed(&["ca"]));
  assert_eq!(SnakeCaseStr::new("cb").to_words(), Cow::Borrowed(&["cb"]));
  assert_eq!(SnakeCaseStr::new("db").to_words(), Cow::Borrowed(&["db"]));
  assert_eq!(SnakeCaseStr::new("dba").to_words(), Cow::Borrowed(&["dba"]));
  assert_eq!(
    SnakeCaseStr::new("dba_a").to_words(),
    Cow::Borrowed(&["dba", "a"])
  );
  assert_eq!(
    SnakeCaseStr::new("dba_ab").to_words(),
    Cow::Borrowed(&["dba", "ab"])
  );
  assert_eq!(
    SnakeCaseStr::new("dba_ab_c").to_words(),
    Cow::Borrowed(&["dba", "ab", "c"])
  );
}

#[test]
fn test_to_camel_case() {
  assert_eq!(SnakeCaseStr::new("a").to_camel_case(), "A");
  assert_eq!(SnakeCaseStr::new("b").to_camel_case(), "B");
  assert_eq!(SnakeCaseStr::new("c").to_camel_case(), "C");
  assert_eq!(SnakeCaseStr::new("ca").to_camel_case(), "Ca");
  assert_eq!(SnakeCaseStr::new("cb").to_camel_case(), "Cb");
  assert_eq!(SnakeCaseStr::new("db").to_camel_case(), "Db");
  assert_eq!(SnakeCaseStr::new("dba").to_camel_case(), "Dba");
  assert_eq!(SnakeCaseStr::new("dba_a").to_camel_case(), "DbaA");
  assert_eq!(SnakeCaseStr::new("dba_ab").to_camel_case(), "DbaAb");
  assert_eq!(SnakeCaseStr::new("dba_ab_c").to_camel_case(), "DbaAbC");
}

#[test]
fn test_to_snake_case() {
  assert_eq!(SnakeCaseStr::new("a").to_snake_case(), "a");
  assert_eq!(SnakeCaseStr::new("b").to_snake_case(), "b");
  assert_eq!(SnakeCaseStr::new("c").to_snake_case(), "c");
  assert_eq!(SnakeCaseStr::new("ca").to_snake_case(), "ca");
  assert_eq!(SnakeCaseStr::new("cb").to_snake_case(), "cb");
  assert_eq!(SnakeCaseStr::new("db").to_snake_case(), "db");
  assert_eq!(SnakeCaseStr::new("dba").to_snake_case(), "dba");
  assert_eq!(SnakeCaseStr::new("dba_a").to_snake_case(), "dba_a");
  assert_eq!(SnakeCaseStr::new("dba_ab").to_snake_case(), "dba_ab");
  assert_eq!(SnakeCaseStr::new("dba_ab_c").to_snake_case(), "dba_ab_c");
}
