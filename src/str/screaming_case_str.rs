use crate::{
  camel_case_str::ToCamelCase, consts::RADIX, lower_case_str::ToLowerCase,
  pascal_case_str::ToPascalCase, snake_case_str::ToSnakeCase, title_case_str::ToTitleCase,
  to_words::ToWords, upper_case_str::ToUpperCase,
};

use std::borrow::Cow;

#[derive(Copy, Clone, Debug)]
pub struct ScreamingCaseStr<'a>(&'a str);

impl<'a> ScreamingCaseStr<'a> {
  pub fn new(value: &'a str) -> Self {
    debug_assert!(
      value.chars().next().unwrap().is_uppercase(),
      "The first character o a screaming case string must be a capital letter!"
    );
    debug_assert!(
      value
        .chars()
        .all(|ch| ch.is_uppercase() || ch.is_digit(RADIX) || ch == '_'),
      "Screaming case string given must only contain capital letters, digits, and underscores!"
    );
    Self(value)
  }
}

impl ToWords for ScreamingCaseStr<'_> {
  fn to_words(&self) -> Cow<[String]> {
    self.0.split('_').map(|word| word.to_owned()).collect()
  }
}

impl ToCamelCase for ScreamingCaseStr<'_> {
  fn to_camel_case(&self) -> String {
    self
      .to_words()
      .iter()
      .enumerate()
      .map(|(id, word)| {
        let word = word.to_lowercase();
        if id != 0 {
          word[0..1].to_uppercase() + &word[1..]
        } else {
          word
        }
      })
      .fold("".to_owned(), |result, word| result + &word)
  }
}

impl ToLowerCase for ScreamingCaseStr<'_> {
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

impl ToPascalCase for ScreamingCaseStr<'_> {
  fn to_pascal_case(&self) -> String {
    self
      .to_words()
      .iter()
      .map(|word| {
        let word = word.to_lowercase();
        word[0..1].to_uppercase() + &word[1..]
      })
      .fold("".to_owned(), |result, word| result + &word)
  }
}

impl ToSnakeCase for ScreamingCaseStr<'_> {
  fn to_snake_case(&self) -> String {
    self.0.to_lowercase()
  }
}

impl ToTitleCase for ScreamingCaseStr<'_> {
  fn to_title_case(&self) -> String {
    self
      .to_words()
      .iter()
      .map(|word| {
        let word = word.to_lowercase();
        word[0..1].to_uppercase() + &word[1..]
      })
      .fold("".to_owned(), |result, word| result + &word + " ")
      .trim_end()
      .to_owned()
  }
}

impl ToUpperCase for ScreamingCaseStr<'_> {
  fn to_upper_case(&self) -> String {
    self
      .to_words()
      .iter()
      .fold("".to_owned(), |result, word| result + word + " ")
      .trim_end()
      .to_owned()
  }
}

pub trait ToScreamingCase {
  fn to_screaming_case(&self) -> String;
}

impl ToScreamingCase for ScreamingCaseStr<'_> {
  fn to_screaming_case(&self) -> String {
    self.0.to_owned()
  }
}
