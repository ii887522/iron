use crate::{
  camel_case_str::ToCamelCase, consts::RADIX, lower_case_str::ToLowerCase,
  pascal_case_str::ToPascalCase, screaming_case_str::ToScreamingCase, title_case_str::ToTitleCase,
  to_words::ToWords, upper_case_str::ToUpperCase,
};

use std::borrow::Cow;

#[derive(Copy, Clone, Debug)]
pub struct SnakeCaseStr<'a>(&'a str);

impl<'a> SnakeCaseStr<'a> {
  pub fn new(value: &'a str) -> Self {
    debug_assert!(
      value.chars().next().unwrap().is_lowercase(),
      "The first character of a snake case string must be a small letter!"
    );
    debug_assert!(
      value
        .chars()
        .all(|ch| ch.is_lowercase() || ch.is_digit(RADIX) || ch == '_'),
      "Snake case string given must only contain small letters, digits, and underscores!"
    );
    Self(value)
  }
}

impl ToWords for SnakeCaseStr<'_> {
  fn to_words(&self) -> Cow<[String]> {
    self.0.split('_').map(|word| word.to_owned()).collect()
  }
}

impl ToCamelCase for SnakeCaseStr<'_> {
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

impl ToLowerCase for SnakeCaseStr<'_> {
  fn to_lower_case(&self) -> String {
    self
      .to_words()
      .iter()
      .fold("".to_owned(), |result, word| result + word + " ")
      .trim_end()
      .to_owned()
  }
}

impl ToPascalCase for SnakeCaseStr<'_> {
  fn to_pascal_case(&self) -> String {
    self
      .to_words()
      .iter()
      .map(|word| word[0..1].to_uppercase() + &word[1..])
      .fold("".to_owned(), |result, word| result + &word)
  }
}

impl ToScreamingCase for SnakeCaseStr<'_> {
  fn to_screaming_case(&self) -> String {
    self.0.to_uppercase()
  }
}

impl ToTitleCase for SnakeCaseStr<'_> {
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

impl ToUpperCase for SnakeCaseStr<'_> {
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

pub trait ToSnakeCase {
  fn to_snake_case(&self) -> String;
}

impl ToSnakeCase for SnakeCaseStr<'_> {
  fn to_snake_case(&self) -> String {
    self.0.to_owned()
  }
}
