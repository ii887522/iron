use iron_ingot::{camel_case_str::ToCamelCase, snake_case_str::ToSnakeCase, CamelCaseStr, ToWords};
use std::borrow::Cow;

#[test]
fn test_to_words() {
  assert_eq!(CamelCaseStr::new("A").to_words(), Cow::Borrowed(&["A"]));
  assert_eq!(CamelCaseStr::new("B").to_words(), Cow::Borrowed(&["B"]));
  assert_eq!(CamelCaseStr::new("C").to_words(), Cow::Borrowed(&["C"]));
  assert_eq!(CamelCaseStr::new("Ca").to_words(), Cow::Borrowed(&["Ca"]));
  assert_eq!(CamelCaseStr::new("Cb").to_words(), Cow::Borrowed(&["Cb"]));
  assert_eq!(CamelCaseStr::new("Db").to_words(), Cow::Borrowed(&["Db"]));
  assert_eq!(CamelCaseStr::new("Dba").to_words(), Cow::Borrowed(&["Dba"]));
  assert_eq!(
    CamelCaseStr::new("DbaA").to_words(),
    Cow::Borrowed(&["Dba", "A"])
  );
  assert_eq!(
    CamelCaseStr::new("DbaAb").to_words(),
    Cow::Borrowed(&["Dba", "Ab"])
  );
  assert_eq!(
    CamelCaseStr::new("DbaAbC").to_words(),
    Cow::Borrowed(&["Dba", "Ab", "C"])
  );
}

#[test]
fn test_to_snake_case() {
  assert_eq!(CamelCaseStr::new("A").to_snake_case(), "a");
  assert_eq!(CamelCaseStr::new("B").to_snake_case(), "b");
  assert_eq!(CamelCaseStr::new("C").to_snake_case(), "c");
  assert_eq!(CamelCaseStr::new("Ca").to_snake_case(), "ca");
  assert_eq!(CamelCaseStr::new("Cb").to_snake_case(), "cb");
  assert_eq!(CamelCaseStr::new("Db").to_snake_case(), "db");
  assert_eq!(CamelCaseStr::new("Dba").to_snake_case(), "dba");
  assert_eq!(CamelCaseStr::new("DbaA").to_snake_case(), "dba_a");
  assert_eq!(CamelCaseStr::new("DbaAb").to_snake_case(), "dba_ab");
  assert_eq!(CamelCaseStr::new("DbaAbC").to_snake_case(), "dba_ab_c");
}

#[test]
fn test_to_camel_case() {
  assert_eq!(CamelCaseStr::new("A").to_camel_case(), "A");
  assert_eq!(CamelCaseStr::new("B").to_camel_case(), "B");
  assert_eq!(CamelCaseStr::new("C").to_camel_case(), "C");
  assert_eq!(CamelCaseStr::new("Ca").to_camel_case(), "Ca");
  assert_eq!(CamelCaseStr::new("Cb").to_camel_case(), "Cb");
  assert_eq!(CamelCaseStr::new("Db").to_camel_case(), "Db");
  assert_eq!(CamelCaseStr::new("Dba").to_camel_case(), "Dba");
  assert_eq!(CamelCaseStr::new("DbaA").to_camel_case(), "DbaA");
  assert_eq!(CamelCaseStr::new("DbaAb").to_camel_case(), "DbaAb");
  assert_eq!(CamelCaseStr::new("DbaAbC").to_camel_case(), "DbaAbC");
}
