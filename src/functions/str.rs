pub trait Substr {
  /// It returns a substring from this object which starts from the `start_str` given and ends with a text
  /// which immediately precedes the `end_str` given.
  ///
  /// `start_str`: The start text included in this object.
  ///
  /// `end_str`: The end text included in this object.
  ///
  /// It returns the substring requested if it exists.
  fn substr(&self, start_str: &str, end_str: &str) -> Option<&str>;
}

impl Substr for str {
  fn substr(&self, start_str: &str, end_str: &str) -> Option<&str> {
    let aux = &self[self.find(start_str)?..];
    Some(&aux[..aux.find(end_str)?])
  }
}
