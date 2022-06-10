use crate::{snake_case_str::ToSnakeCase, to_words::ToWords};
use std::borrow::Cow;

#[derive(Copy, Clone, Debug)]
pub struct CamelCaseStr<'a>(&'a str);

impl<'a> CamelCaseStr<'a> {
  pub fn new(value: &'a str) -> Self {
    debug_assert!(
      value.starts_with(char::is_uppercase),
      "Camel case string given must start from a capital letter!"
    );
    Self(value)
  }
}

impl<'a> ToWords for CamelCaseStr<'a> {
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

impl<'a> ToSnakeCase for CamelCaseStr<'a> {
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

pub trait ToCamelCase {
  fn to_camel_case(&self) -> String;
}

impl<'a> ToCamelCase for CamelCaseStr<'a> {
  fn to_camel_case(&self) -> String {
    self.0.to_owned()
  }
}
