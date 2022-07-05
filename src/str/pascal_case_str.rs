use crate::{
  camel_case_str::ToCamelCase, lower_case_str::ToLowerCase, screaming_case_str::ToScreamingCase,
  snake_case_str::ToSnakeCase, title_case_str::ToTitleCase, to_words::ToWords,
  upper_case_str::ToUpperCase,
};

use std::borrow::Cow;

#[derive(Copy, Clone, Debug)]
pub struct PascalCaseStr<'a>(&'a str);

impl<'a> PascalCaseStr<'a> {
  pub fn new(value: &'a str) -> Self {
    debug_assert!(
      value.chars().next().unwrap().is_uppercase(),
      "The first character of a pascal case string must be a capital letter!"
    );
    debug_assert!(
      value.chars().all(|ch| ch.is_alphanumeric()),
      "Pascal case string given must only contain letters and digits!"
    );
    Self(value)
  }
}

impl ToWords for PascalCaseStr<'_> {
  fn to_words(&self) -> Cow<[String]> {
    let mut words = vec![];
    let mut cap_letter = "";
    for substr in (self.0.to_owned() + "A").split_inclusive(char::is_uppercase) {
      let word = cap_letter.to_owned() + &substr[..substr.len() - 1];
      if !word.is_empty() {
        words.push(word);
      }
      cap_letter = &substr[substr.len() - 1..];
    }
    words.into()
  }
}

impl ToCamelCase for PascalCaseStr<'_> {
  fn to_camel_case(&self) -> String {
    self.0[0..1].to_lowercase() + &self.0[1..]
  }
}

impl ToLowerCase for PascalCaseStr<'_> {
  fn to_lower_case(&self) -> String {
    self
      .to_words()
      .iter()
      .map(|word| word.to_lowercase())
      .fold("".to_owned(), |result, word| result + &word + " ")
      .trim_end()
      .to_owned()
  }
}

impl ToScreamingCase for PascalCaseStr<'_> {
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

impl ToSnakeCase for PascalCaseStr<'_> {
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

impl ToTitleCase for PascalCaseStr<'_> {
  fn to_title_case(&self) -> String {
    self
      .to_words()
      .iter()
      .fold("".to_owned(), |result, word| result + word + " ")
      .trim_end()
      .to_owned()
  }
}

impl ToUpperCase for PascalCaseStr<'_> {
  fn to_upper_case(&self) -> String {
    self
      .to_words()
      .iter()
      .map(|word| word.to_uppercase())
      .fold("".to_owned(), |result, word| result + &word + " ")
      .trim_end()
      .to_owned()
  }
}

pub trait ToPascalCase {
  fn to_pascal_case(&self) -> String;
}

impl ToPascalCase for PascalCaseStr<'_> {
  fn to_pascal_case(&self) -> String {
    self.0.to_owned()
  }
}
