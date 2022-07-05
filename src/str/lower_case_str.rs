use crate::{
  camel_case_str::ToCamelCase, consts::RADIX, pascal_case_str::ToPascalCase,
  screaming_case_str::ToScreamingCase, snake_case_str::ToSnakeCase, title_case_str::ToTitleCase,
  upper_case_str::ToUpperCase, ToWords,
};

use std::borrow::Cow;

#[derive(Copy, Clone, Debug)]
pub struct LowerCaseStr<'a>(&'a str);

impl<'a> LowerCaseStr<'a> {
  pub fn new(value: &'a str) -> Self {
    debug_assert!(
      value.chars().next().unwrap().is_lowercase(),
      "The first character of a lower case string must be a small letter!"
    );
    debug_assert!(
      value
        .chars()
        .all(|ch| ch.is_lowercase() || ch.is_digit(RADIX) || ch == ' '),
      "Lower case string given must only contain small letters, digits, and spaces!"
    );
    Self(value)
  }
}

impl ToWords for LowerCaseStr<'_> {
  fn to_words(&self) -> Cow<[String]> {
    self.0.split(' ').map(|word| word.to_owned()).collect()
  }
}

impl ToCamelCase for LowerCaseStr<'_> {
  fn to_camel_case(&self) -> String {
    self
      .to_words()
      .iter()
      .enumerate()
      .map(|(id, word)| {
        if id != 0 {
          word[0..1].to_uppercase() + &word[1..]
        } else {
          word.to_owned()
        }
      })
      .fold("".to_owned(), |result, word| result + &word)
  }
}

impl ToPascalCase for LowerCaseStr<'_> {
  fn to_pascal_case(&self) -> String {
    self
      .to_words()
      .iter()
      .map(|word| word[0..1].to_uppercase() + &word[1..])
      .fold("".to_owned(), |result, word| result + &word)
  }
}

impl ToScreamingCase for LowerCaseStr<'_> {
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

impl ToSnakeCase for LowerCaseStr<'_> {
  fn to_snake_case(&self) -> String {
    self
      .to_words()
      .iter()
      .fold("".to_owned(), |result, word| result + word + "_")
      .trim_end_matches('_')
      .to_owned()
  }
}

impl ToTitleCase for LowerCaseStr<'_> {
  fn to_title_case(&self) -> String {
    self
      .to_words()
      .iter()
      .map(|word| word[0..1].to_uppercase() + &word[1..])
      .fold("".to_owned(), |result, word| result + &word + " ")
      .trim_end()
      .to_owned()
  }
}

impl ToUpperCase for LowerCaseStr<'_> {
  fn to_upper_case(&self) -> String {
    self.0.to_uppercase()
  }
}

pub trait ToLowerCase {
  fn to_lower_case(&self) -> String;
}

impl ToLowerCase for LowerCaseStr<'_> {
  fn to_lower_case(&self) -> String {
    self.0.to_owned()
  }
}
