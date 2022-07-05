use iron_ingot::{
  camel_case_str::ToCamelCase, lower_case_str::ToLowerCase, pascal_case_str::ToPascalCase,
  screaming_case_str::ToScreamingCase, snake_case_str::ToSnakeCase, title_case_str::ToTitleCase,
  upper_case_str::ToUpperCase, TitleCaseStr, ToWords,
};

use std::borrow::Cow;

#[test]
fn test_to_words() {
  assert_eq!(TitleCaseStr::new("A").to_words(), Cow::Borrowed(&["A"]));
  assert_eq!(TitleCaseStr::new("B").to_words(), Cow::Borrowed(&["B"]));
  assert_eq!(TitleCaseStr::new("C").to_words(), Cow::Borrowed(&["C"]));
  assert_eq!(TitleCaseStr::new("Ca").to_words(), Cow::Borrowed(&["Ca"]));
  assert_eq!(TitleCaseStr::new("Cb").to_words(), Cow::Borrowed(&["Cb"]));
  assert_eq!(TitleCaseStr::new("Db").to_words(), Cow::Borrowed(&["Db"]));
  assert_eq!(TitleCaseStr::new("Dba").to_words(), Cow::Borrowed(&["Dba"]));
  assert_eq!(
    TitleCaseStr::new("Dba A").to_words(),
    Cow::Borrowed(&["Dba", "A"])
  );
  assert_eq!(
    TitleCaseStr::new("Dba Ab").to_words(),
    Cow::Borrowed(&["Dba", "Ab"])
  );
  assert_eq!(
    TitleCaseStr::new("Dba Ab C").to_words(),
    Cow::Borrowed(&["Dba", "Ab", "C"])
  );
}

#[test]
fn test_to_camel_case() {
  assert_eq!(TitleCaseStr::new("A").to_camel_case(), "a");
  assert_eq!(TitleCaseStr::new("B").to_camel_case(), "b");
  assert_eq!(TitleCaseStr::new("C").to_camel_case(), "c");
  assert_eq!(TitleCaseStr::new("Ca").to_camel_case(), "ca");
  assert_eq!(TitleCaseStr::new("Cb").to_camel_case(), "cb");
  assert_eq!(TitleCaseStr::new("Db").to_camel_case(), "db");
  assert_eq!(TitleCaseStr::new("Dba").to_camel_case(), "dba");
  assert_eq!(TitleCaseStr::new("Dba A").to_camel_case(), "dbaA");
  assert_eq!(TitleCaseStr::new("Dba Ab").to_camel_case(), "dbaAb");
  assert_eq!(TitleCaseStr::new("Dba Ab C").to_camel_case(), "dbaAbC");
}

#[test]
fn test_to_lower_case() {
  assert_eq!(TitleCaseStr::new("A").to_lower_case(), "a");
  assert_eq!(TitleCaseStr::new("B").to_lower_case(), "b");
  assert_eq!(TitleCaseStr::new("C").to_lower_case(), "c");
  assert_eq!(TitleCaseStr::new("Ca").to_lower_case(), "ca");
  assert_eq!(TitleCaseStr::new("Cb").to_lower_case(), "cb");
  assert_eq!(TitleCaseStr::new("Db").to_lower_case(), "db");
  assert_eq!(TitleCaseStr::new("Dba").to_lower_case(), "dba");
  assert_eq!(TitleCaseStr::new("Dba A").to_lower_case(), "dba a");
  assert_eq!(TitleCaseStr::new("Dba Ab").to_lower_case(), "dba ab");
  assert_eq!(TitleCaseStr::new("Dba Ab C").to_lower_case(), "dba ab c");
}

#[test]
fn test_to_pascal_case() {
  assert_eq!(TitleCaseStr::new("A").to_pascal_case(), "A");
  assert_eq!(TitleCaseStr::new("B").to_pascal_case(), "B");
  assert_eq!(TitleCaseStr::new("C").to_pascal_case(), "C");
  assert_eq!(TitleCaseStr::new("Ca").to_pascal_case(), "Ca");
  assert_eq!(TitleCaseStr::new("Cb").to_pascal_case(), "Cb");
  assert_eq!(TitleCaseStr::new("Db").to_pascal_case(), "Db");
  assert_eq!(TitleCaseStr::new("Dba").to_pascal_case(), "Dba");
  assert_eq!(TitleCaseStr::new("Dba A").to_pascal_case(), "DbaA");
  assert_eq!(TitleCaseStr::new("Dba Ab").to_pascal_case(), "DbaAb");
  assert_eq!(TitleCaseStr::new("Dba Ab C").to_pascal_case(), "DbaAbC");
}

#[test]
fn test_to_upper_case() {
  assert_eq!(TitleCaseStr::new("A").to_upper_case(), "A");
  assert_eq!(TitleCaseStr::new("B").to_upper_case(), "B");
  assert_eq!(TitleCaseStr::new("C").to_upper_case(), "C");
  assert_eq!(TitleCaseStr::new("Ca").to_upper_case(), "CA");
  assert_eq!(TitleCaseStr::new("Cb").to_upper_case(), "CB");
  assert_eq!(TitleCaseStr::new("Db").to_upper_case(), "DB");
  assert_eq!(TitleCaseStr::new("Dba").to_upper_case(), "DBA");
  assert_eq!(TitleCaseStr::new("Dba A").to_upper_case(), "DBA A");
  assert_eq!(TitleCaseStr::new("Dba Ab").to_upper_case(), "DBA AB");
  assert_eq!(TitleCaseStr::new("Dba Ab C").to_upper_case(), "DBA AB C");
}

#[test]
fn test_to_screaming_case() {
  assert_eq!(TitleCaseStr::new("A").to_screaming_case(), "A");
  assert_eq!(TitleCaseStr::new("B").to_screaming_case(), "B");
  assert_eq!(TitleCaseStr::new("C").to_screaming_case(), "C");
  assert_eq!(TitleCaseStr::new("Ca").to_screaming_case(), "CA");
  assert_eq!(TitleCaseStr::new("Cb").to_screaming_case(), "CB");
  assert_eq!(TitleCaseStr::new("Db").to_screaming_case(), "DB");
  assert_eq!(TitleCaseStr::new("Dba").to_screaming_case(), "DBA");
  assert_eq!(TitleCaseStr::new("Dba A").to_screaming_case(), "DBA_A");
  assert_eq!(TitleCaseStr::new("Dba Ab").to_screaming_case(), "DBA_AB");
  assert_eq!(
    TitleCaseStr::new("Dba Ab C").to_screaming_case(),
    "DBA_AB_C"
  );
}

#[test]
fn test_to_snake_case() {
  assert_eq!(TitleCaseStr::new("A").to_snake_case(), "a");
  assert_eq!(TitleCaseStr::new("B").to_snake_case(), "b");
  assert_eq!(TitleCaseStr::new("C").to_snake_case(), "c");
  assert_eq!(TitleCaseStr::new("Ca").to_snake_case(), "ca");
  assert_eq!(TitleCaseStr::new("Cb").to_snake_case(), "cb");
  assert_eq!(TitleCaseStr::new("Db").to_snake_case(), "db");
  assert_eq!(TitleCaseStr::new("Dba").to_snake_case(), "dba");
  assert_eq!(TitleCaseStr::new("Dba A").to_snake_case(), "dba_a");
  assert_eq!(TitleCaseStr::new("Dba Ab").to_snake_case(), "dba_ab");
  assert_eq!(TitleCaseStr::new("Dba Ab C").to_snake_case(), "dba_ab_c");
}

#[test]
fn test_to_title_case() {
  assert_eq!(TitleCaseStr::new("A").to_title_case(), "A");
  assert_eq!(TitleCaseStr::new("B").to_title_case(), "B");
  assert_eq!(TitleCaseStr::new("C").to_title_case(), "C");
  assert_eq!(TitleCaseStr::new("Ca").to_title_case(), "Ca");
  assert_eq!(TitleCaseStr::new("Cb").to_title_case(), "Cb");
  assert_eq!(TitleCaseStr::new("Db").to_title_case(), "Db");
  assert_eq!(TitleCaseStr::new("Dba").to_title_case(), "Dba");
  assert_eq!(TitleCaseStr::new("Dba A").to_title_case(), "Dba A");
  assert_eq!(TitleCaseStr::new("Dba Ab").to_title_case(), "Dba Ab");
  assert_eq!(TitleCaseStr::new("Dba Ab C").to_title_case(), "Dba Ab C");
}
