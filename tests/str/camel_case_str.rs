use iron_ingot::{
  camel_case_str::ToCamelCase, lower_case_str::ToLowerCase, pascal_case_str::ToPascalCase,
  screaming_case_str::ToScreamingCase, snake_case_str::ToSnakeCase, title_case_str::ToTitleCase,
  upper_case_str::ToUpperCase, CamelCaseStr, ToWords,
};

use std::borrow::Cow;

#[test]
fn test_to_words() {
  assert_eq!(CamelCaseStr::new("a").to_words(), Cow::Borrowed(&["a"]));
  assert_eq!(CamelCaseStr::new("b").to_words(), Cow::Borrowed(&["b"]));
  assert_eq!(CamelCaseStr::new("c").to_words(), Cow::Borrowed(&["c"]));
  assert_eq!(CamelCaseStr::new("ca").to_words(), Cow::Borrowed(&["ca"]));
  assert_eq!(CamelCaseStr::new("cb").to_words(), Cow::Borrowed(&["cb"]));
  assert_eq!(CamelCaseStr::new("db").to_words(), Cow::Borrowed(&["db"]));
  assert_eq!(CamelCaseStr::new("dba").to_words(), Cow::Borrowed(&["dba"]));
  assert_eq!(
    CamelCaseStr::new("dbaA").to_words(),
    Cow::Borrowed(&["dba", "A"])
  );
  assert_eq!(
    CamelCaseStr::new("dbaAb").to_words(),
    Cow::Borrowed(&["dba", "Ab"])
  );
  assert_eq!(
    CamelCaseStr::new("dbaAbC").to_words(),
    Cow::Borrowed(&["dba", "Ab", "C"])
  );
}

#[test]
fn test_to_lower_case() {
  assert_eq!(CamelCaseStr::new("a").to_lower_case(), "a");
  assert_eq!(CamelCaseStr::new("b").to_lower_case(), "b");
  assert_eq!(CamelCaseStr::new("c").to_lower_case(), "c");
  assert_eq!(CamelCaseStr::new("ca").to_lower_case(), "ca");
  assert_eq!(CamelCaseStr::new("cb").to_lower_case(), "cb");
  assert_eq!(CamelCaseStr::new("db").to_lower_case(), "db");
  assert_eq!(CamelCaseStr::new("dba").to_lower_case(), "dba");
  assert_eq!(CamelCaseStr::new("dbaA").to_lower_case(), "dba a");
  assert_eq!(CamelCaseStr::new("dbaAb").to_lower_case(), "dba ab");
  assert_eq!(CamelCaseStr::new("dbaAbC").to_lower_case(), "dba ab c");
}

#[test]
fn test_to_pascal_case() {
  assert_eq!(CamelCaseStr::new("a").to_pascal_case(), "A");
  assert_eq!(CamelCaseStr::new("b").to_pascal_case(), "B");
  assert_eq!(CamelCaseStr::new("c").to_pascal_case(), "C");
  assert_eq!(CamelCaseStr::new("ca").to_pascal_case(), "Ca");
  assert_eq!(CamelCaseStr::new("cb").to_pascal_case(), "Cb");
  assert_eq!(CamelCaseStr::new("db").to_pascal_case(), "Db");
  assert_eq!(CamelCaseStr::new("dba").to_pascal_case(), "Dba");
  assert_eq!(CamelCaseStr::new("dbaA").to_pascal_case(), "DbaA");
  assert_eq!(CamelCaseStr::new("dbaAb").to_pascal_case(), "DbaAb");
  assert_eq!(CamelCaseStr::new("dbaAbC").to_pascal_case(), "DbaAbC");
}

#[test]
fn test_to_screaming_case() {
  assert_eq!(CamelCaseStr::new("a").to_screaming_case(), "A");
  assert_eq!(CamelCaseStr::new("b").to_screaming_case(), "B");
  assert_eq!(CamelCaseStr::new("c").to_screaming_case(), "C");
  assert_eq!(CamelCaseStr::new("ca").to_screaming_case(), "CA");
  assert_eq!(CamelCaseStr::new("cb").to_screaming_case(), "CB");
  assert_eq!(CamelCaseStr::new("db").to_screaming_case(), "DB");
  assert_eq!(CamelCaseStr::new("dba").to_screaming_case(), "DBA");
  assert_eq!(CamelCaseStr::new("dbaA").to_screaming_case(), "DBA_A");
  assert_eq!(CamelCaseStr::new("dbaAb").to_screaming_case(), "DBA_AB");
  assert_eq!(CamelCaseStr::new("dbaAbC").to_screaming_case(), "DBA_AB_C");
}

#[test]
fn test_to_snake_case() {
  assert_eq!(CamelCaseStr::new("a").to_snake_case(), "a");
  assert_eq!(CamelCaseStr::new("b").to_snake_case(), "b");
  assert_eq!(CamelCaseStr::new("c").to_snake_case(), "c");
  assert_eq!(CamelCaseStr::new("ca").to_snake_case(), "ca");
  assert_eq!(CamelCaseStr::new("cb").to_snake_case(), "cb");
  assert_eq!(CamelCaseStr::new("db").to_snake_case(), "db");
  assert_eq!(CamelCaseStr::new("dba").to_snake_case(), "dba");
  assert_eq!(CamelCaseStr::new("dbaA").to_snake_case(), "dba_a");
  assert_eq!(CamelCaseStr::new("dbaAb").to_snake_case(), "dba_ab");
  assert_eq!(CamelCaseStr::new("dbaAbC").to_snake_case(), "dba_ab_c");
}

#[test]
fn test_to_title_case() {
  assert_eq!(CamelCaseStr::new("a").to_title_case(), "A");
  assert_eq!(CamelCaseStr::new("b").to_title_case(), "B");
  assert_eq!(CamelCaseStr::new("c").to_title_case(), "C");
  assert_eq!(CamelCaseStr::new("ca").to_title_case(), "Ca");
  assert_eq!(CamelCaseStr::new("cb").to_title_case(), "Cb");
  assert_eq!(CamelCaseStr::new("db").to_title_case(), "Db");
  assert_eq!(CamelCaseStr::new("dba").to_title_case(), "Dba");
  assert_eq!(CamelCaseStr::new("dbaA").to_title_case(), "Dba A");
  assert_eq!(CamelCaseStr::new("dbaAb").to_title_case(), "Dba Ab");
  assert_eq!(CamelCaseStr::new("dbaAbC").to_title_case(), "Dba Ab C");
}

#[test]
fn test_to_upper_case() {
  assert_eq!(CamelCaseStr::new("a").to_upper_case(), "A");
  assert_eq!(CamelCaseStr::new("b").to_upper_case(), "B");
  assert_eq!(CamelCaseStr::new("c").to_upper_case(), "C");
  assert_eq!(CamelCaseStr::new("ca").to_upper_case(), "CA");
  assert_eq!(CamelCaseStr::new("cb").to_upper_case(), "CB");
  assert_eq!(CamelCaseStr::new("db").to_upper_case(), "DB");
  assert_eq!(CamelCaseStr::new("dba").to_upper_case(), "DBA");
  assert_eq!(CamelCaseStr::new("dbaA").to_upper_case(), "DBA A");
  assert_eq!(CamelCaseStr::new("dbaAb").to_upper_case(), "DBA AB");
  assert_eq!(CamelCaseStr::new("dbaAbC").to_upper_case(), "DBA AB C");
}

#[test]
fn test_to_camel_case() {
  assert_eq!(CamelCaseStr::new("a").to_camel_case(), "a");
  assert_eq!(CamelCaseStr::new("b").to_camel_case(), "b");
  assert_eq!(CamelCaseStr::new("c").to_camel_case(), "c");
  assert_eq!(CamelCaseStr::new("ca").to_camel_case(), "ca");
  assert_eq!(CamelCaseStr::new("cb").to_camel_case(), "cb");
  assert_eq!(CamelCaseStr::new("db").to_camel_case(), "db");
  assert_eq!(CamelCaseStr::new("dba").to_camel_case(), "dba");
  assert_eq!(CamelCaseStr::new("dbaA").to_camel_case(), "dbaA");
  assert_eq!(CamelCaseStr::new("dbaAb").to_camel_case(), "dbaAb");
  assert_eq!(CamelCaseStr::new("dbaAbC").to_camel_case(), "dbaAbC");
}
