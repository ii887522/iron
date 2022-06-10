use crate::{camel_case_str::ToCamelCase, to_words::ToWords};
use std::borrow::Cow;

#[derive(Copy, Clone, Debug)]
pub struct SnakeCaseStr<'a>(&'a str);

impl<'a> SnakeCaseStr<'a> {
  pub fn new(value: &'a str) -> Self {
    debug_assert!(
      value.starts_with(char::is_lowercase),
      "Snake case string given must start from a small letter!"
    );
    Self(value)
  }
}

impl<'a> ToWords for SnakeCaseStr<'a> {
  fn to_words(&self) -> Cow<[String]> {
    self
      .0
      .split('_')
      .map(|word| word.to_owned())
      .collect::<Vec<String>>()
      .into()
  }
}

impl<'a> ToCamelCase for SnakeCaseStr<'a> {
  fn to_camel_case(&self) -> String {
    self
      .to_words()
      .iter()
      .map(|word| word[0..1].to_uppercase() + &word[1..])
      .fold("".to_owned(), |result, word| result + &word)
  }
}

pub trait ToSnakeCase {
  fn to_snake_case(&self) -> String;
}

impl<'a> ToSnakeCase for SnakeCaseStr<'a> {
  fn to_snake_case(&self) -> String {
    self.0.to_owned()
  }
}
