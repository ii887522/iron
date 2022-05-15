use std::borrow::Cow;

pub trait ToWords {
  fn to_words(&self) -> Cow<[String]>;
}
