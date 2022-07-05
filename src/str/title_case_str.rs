use crate::{
  camel_case_str::ToCamelCase, lower_case_str::ToLowerCase, pascal_case_str::ToPascalCase,
  screaming_case_str::ToScreamingCase, snake_case_str::ToSnakeCase, upper_case_str::ToUpperCase,
  ToWords,
};

use std::borrow::Cow;

#[derive(Copy, Clone, Debug)]
pub struct TitleCaseStr<'a>(&'a str);

impl<'a> TitleCaseStr<'a> {
  pub fn new(value: &'a str) -> Self {
    debug_assert!(
      value.chars().next().unwrap().is_uppercase(),
      "The first character of a title case string must be a capital letter!"
    );
    debug_assert!(
      value.chars().all(|ch| ch.is_alphanumeric() || ch == ' '),
      "Title case string given must only contain letters, digits, and spaces!"
    );
    Self(value)
  }
}

impl ToWords for TitleCaseStr<'_> {
  fn to_words(&self) -> Cow<[String]> {
    self.0.split(' ').map(|word| word.to_owned()).collect()
  }
}

impl ToCamelCase for TitleCaseStr<'_> {
  fn to_camel_case(&self) -> String {
    self
      .to_words()
      .iter()
      .enumerate()
      .map(|(id, word)| {
        if id != 0 {
          word.to_owned()
        } else {
          word[0..1].to_lowercase() + &word[1..]
        }
      })
      .fold("".to_owned(), |result, word| result + &word)
  }
}

impl ToLowerCase for TitleCaseStr<'_> {
  fn to_lower_case(&self) -> String {
    self.0.to_lowercase()
  }
}

impl ToPascalCase for TitleCaseStr<'_> {
  fn to_pascal_case(&self) -> String {
    self
      .to_words()
      .iter()
      .fold("".to_owned(), |result, word| result + word)
  }
}

impl ToScreamingCase for TitleCaseStr<'_> {
  fn to_screaming_case(&self) -> String {
    self
      .to_words()
      .iter()
      .map(|word| word.to_uppercase())
      .fold("".to_owned(), |result, word| result + &word + "_")
      .trim_end_matches('_')
      .to_owned()
  }
}

impl ToSnakeCase for TitleCaseStr<'_> {
  fn to_snake_case(&self) -> String {
    self
      .to_words()
      .iter()
      .map(|word| word.to_lowercase())
      .fold("".to_owned(), |result, word| result + &word + "_")
      .trim_end_matches('_')
      .to_owned()
  }
}

impl ToUpperCase for TitleCaseStr<'_> {
  fn to_upper_case(&self) -> String {
    self.0.to_uppercase()
  }
}

pub trait ToTitleCase {
  fn to_title_case(&self) -> String;
}

impl ToTitleCase for TitleCaseStr<'_> {
  fn to_title_case(&self) -> String {
    self.0.to_owned()
  }
}
