use iron::fs::has_file_with_ext;

#[test]
fn test_has_file_with_ext() {
  assert!(has_file_with_ext("txt", "tests/res/c").unwrap_or(false));
  assert!(!has_file_with_ext("txt", "tests/res/d").unwrap_or(true));
  assert!(has_file_with_ext("txt", "tests/res/e").unwrap_or(false));
  assert!(has_file_with_ext("txt", "tests/res/f").unwrap_or(false));
  assert!(!has_file_with_ext("txt", "tests/res/g").unwrap_or(true));
  assert!(!has_file_with_ext("txt", "tests/res/h").unwrap_or(true));
  assert!(has_file_with_ext("txt", "tests/res/i").unwrap_or(false));
  assert!(!has_file_with_ext("png", "tests/res/c").unwrap_or(true));
  assert!(has_file_with_ext("png", "tests/res/d").unwrap_or(false));
  assert!(has_file_with_ext("png", "tests/res/e").unwrap_or(false));
  assert!(!has_file_with_ext("png", "tests/res/f").unwrap_or(true));
  assert!(has_file_with_ext("png", "tests/res/g").unwrap_or(false));
  assert!(has_file_with_ext("png", "tests/res/h").unwrap_or(false));
  assert!(!has_file_with_ext("png", "tests/res/i").unwrap_or(true));
}
