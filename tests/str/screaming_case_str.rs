use iron_ingot::{
  camel_case_str::ToCamelCase, lower_case_str::ToLowerCase, pascal_case_str::ToPascalCase,
  screaming_case_str::ToScreamingCase, snake_case_str::ToSnakeCase, title_case_str::ToTitleCase,
  upper_case_str::ToUpperCase, ScreamingCaseStr, ToWords,
};

use std::borrow::Cow;

#[test]
fn test_to_words() {
  assert_eq!(ScreamingCaseStr::new("A").to_words(), Cow::Borrowed(&["A"]));
  assert_eq!(ScreamingCaseStr::new("B").to_words(), Cow::Borrowed(&["B"]));
  assert_eq!(ScreamingCaseStr::new("C").to_words(), Cow::Borrowed(&["C"]));
  assert_eq!(
    ScreamingCaseStr::new("CA").to_words(),
    Cow::Borrowed(&["CA"])
  );
  assert_eq!(
    ScreamingCaseStr::new("CB").to_words(),
    Cow::Borrowed(&["CB"])
  );
  assert_eq!(
    ScreamingCaseStr::new("DB").to_words(),
    Cow::Borrowed(&["DB"])
  );
  assert_eq!(
    ScreamingCaseStr::new("DBA").to_words(),
    Cow::Borrowed(&["DBA"])
  );
  assert_eq!(
    ScreamingCaseStr::new("DBA_A").to_words(),
    Cow::Borrowed(&["DBA", "A"])
  );
  assert_eq!(
    ScreamingCaseStr::new("DBA_AB").to_words(),
    Cow::Borrowed(&["DBA", "AB"])
  );
  assert_eq!(
    ScreamingCaseStr::new("DBA_AB_C").to_words(),
    Cow::Borrowed(&["DBA", "AB", "C"])
  );
}

#[test]
fn test_to_camel_case() {
  assert_eq!(ScreamingCaseStr::new("A").to_camel_case(), "a");
  assert_eq!(ScreamingCaseStr::new("B").to_camel_case(), "b");
  assert_eq!(ScreamingCaseStr::new("C").to_camel_case(), "c");
  assert_eq!(ScreamingCaseStr::new("CA").to_camel_case(), "ca");
  assert_eq!(ScreamingCaseStr::new("CB").to_camel_case(), "cb");
  assert_eq!(ScreamingCaseStr::new("DB").to_camel_case(), "db");
  assert_eq!(ScreamingCaseStr::new("DBA").to_camel_case(), "dba");
  assert_eq!(ScreamingCaseStr::new("DBA_A").to_camel_case(), "dbaA");
  assert_eq!(ScreamingCaseStr::new("DBA_AB").to_camel_case(), "dbaAb");
  assert_eq!(ScreamingCaseStr::new("DBA_AB_C").to_camel_case(), "dbaAbC");
}

#[test]
fn test_to_lower_case() {
  assert_eq!(ScreamingCaseStr::new("A").to_lower_case(), "a");
  assert_eq!(ScreamingCaseStr::new("B").to_lower_case(), "b");
  assert_eq!(ScreamingCaseStr::new("C").to_lower_case(), "c");
  assert_eq!(ScreamingCaseStr::new("CA").to_lower_case(), "ca");
  assert_eq!(ScreamingCaseStr::new("CB").to_lower_case(), "cb");
  assert_eq!(ScreamingCaseStr::new("DB").to_lower_case(), "db");
  assert_eq!(ScreamingCaseStr::new("DBA").to_lower_case(), "dba");
  assert_eq!(ScreamingCaseStr::new("DBA_A").to_lower_case(), "dba a");
  assert_eq!(ScreamingCaseStr::new("DBA_AB").to_lower_case(), "dba ab");
  assert_eq!(
    ScreamingCaseStr::new("DBA_AB_C").to_lower_case(),
    "dba ab c"
  );
}

#[test]
fn test_to_snake_case() {
  assert_eq!(ScreamingCaseStr::new("A").to_snake_case(), "a");
  assert_eq!(ScreamingCaseStr::new("B").to_snake_case(), "b");
  assert_eq!(ScreamingCaseStr::new("C").to_snake_case(), "c");
  assert_eq!(ScreamingCaseStr::new("CA").to_snake_case(), "ca");
  assert_eq!(ScreamingCaseStr::new("CB").to_snake_case(), "cb");
  assert_eq!(ScreamingCaseStr::new("DB").to_snake_case(), "db");
  assert_eq!(ScreamingCaseStr::new("DBA").to_snake_case(), "dba");
  assert_eq!(ScreamingCaseStr::new("DBA_A").to_snake_case(), "dba_a");
  assert_eq!(ScreamingCaseStr::new("DBA_AB").to_snake_case(), "dba_ab");
  assert_eq!(
    ScreamingCaseStr::new("DBA_AB_C").to_snake_case(),
    "dba_ab_c"
  );
}

#[test]
fn test_to_title_case() {
  assert_eq!(ScreamingCaseStr::new("A").to_title_case(), "A");
  assert_eq!(ScreamingCaseStr::new("B").to_title_case(), "B");
  assert_eq!(ScreamingCaseStr::new("C").to_title_case(), "C");
  assert_eq!(ScreamingCaseStr::new("CA").to_title_case(), "Ca");
  assert_eq!(ScreamingCaseStr::new("CB").to_title_case(), "Cb");
  assert_eq!(ScreamingCaseStr::new("DB").to_title_case(), "Db");
  assert_eq!(ScreamingCaseStr::new("DBA").to_title_case(), "Dba");
  assert_eq!(ScreamingCaseStr::new("DBA_A").to_title_case(), "Dba A");
  assert_eq!(ScreamingCaseStr::new("DBA_AB").to_title_case(), "Dba Ab");
  assert_eq!(
    ScreamingCaseStr::new("DBA_AB_C").to_title_case(),
    "Dba Ab C"
  );
}

#[test]
fn test_to_upper_case() {
  assert_eq!(ScreamingCaseStr::new("A").to_upper_case(), "A");
  assert_eq!(ScreamingCaseStr::new("B").to_upper_case(), "B");
  assert_eq!(ScreamingCaseStr::new("C").to_upper_case(), "C");
  assert_eq!(ScreamingCaseStr::new("CA").to_upper_case(), "CA");
  assert_eq!(ScreamingCaseStr::new("CB").to_upper_case(), "CB");
  assert_eq!(ScreamingCaseStr::new("DB").to_upper_case(), "DB");
  assert_eq!(ScreamingCaseStr::new("DBA").to_upper_case(), "DBA");
  assert_eq!(ScreamingCaseStr::new("DBA_A").to_upper_case(), "DBA A");
  assert_eq!(ScreamingCaseStr::new("DBA_AB").to_upper_case(), "DBA AB");
  assert_eq!(
    ScreamingCaseStr::new("DBA_AB_C").to_upper_case(),
    "DBA AB C"
  );
}

#[test]
fn test_to_pascal_case() {
  assert_eq!(ScreamingCaseStr::new("A").to_pascal_case(), "A");
  assert_eq!(ScreamingCaseStr::new("B").to_pascal_case(), "B");
  assert_eq!(ScreamingCaseStr::new("C").to_pascal_case(), "C");
  assert_eq!(ScreamingCaseStr::new("CA").to_pascal_case(), "Ca");
  assert_eq!(ScreamingCaseStr::new("CB").to_pascal_case(), "Cb");
  assert_eq!(ScreamingCaseStr::new("DB").to_pascal_case(), "Db");
  assert_eq!(ScreamingCaseStr::new("DBA").to_pascal_case(), "Dba");
  assert_eq!(ScreamingCaseStr::new("DBA_A").to_pascal_case(), "DbaA");
  assert_eq!(ScreamingCaseStr::new("DBA_AB").to_pascal_case(), "DbaAb");
  assert_eq!(ScreamingCaseStr::new("DBA_AB_C").to_pascal_case(), "DbaAbC");
}

#[test]
fn test_to_screaming_case() {
  assert_eq!(ScreamingCaseStr::new("A").to_screaming_case(), "A");
  assert_eq!(ScreamingCaseStr::new("B").to_screaming_case(), "B");
  assert_eq!(ScreamingCaseStr::new("C").to_screaming_case(), "C");
  assert_eq!(ScreamingCaseStr::new("CA").to_screaming_case(), "CA");
  assert_eq!(ScreamingCaseStr::new("CB").to_screaming_case(), "CB");
  assert_eq!(ScreamingCaseStr::new("DB").to_screaming_case(), "DB");
  assert_eq!(ScreamingCaseStr::new("DBA").to_screaming_case(), "DBA");
  assert_eq!(ScreamingCaseStr::new("DBA_A").to_screaming_case(), "DBA_A");
  assert_eq!(
    ScreamingCaseStr::new("DBA_AB").to_screaming_case(),
    "DBA_AB"
  );
  assert_eq!(
    ScreamingCaseStr::new("DBA_AB_C").to_screaming_case(),
    "DBA_AB_C"
  );
}
