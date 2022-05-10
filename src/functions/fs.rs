use std::{
  fs::{create_dir, read_dir, remove_dir_all, remove_file},
  io,
};

// It removes all files and subdirectories in the directory where its path is received from its parameter.
//
// `dir_path`: The path where the directory is to be emptied.
pub fn empty_dir(dir_path: &str) -> io::Result<()> {
  if let Err(err) = remove_dir_all(dir_path) {
    if err.kind() != io::ErrorKind::NotFound {
      return Err(err);
    }
  }
  create_dir(dir_path)?;
  Ok(())
}

/// It removes all files in the directory where its path is received from its parameter with matching `ext_name`
/// received.
///
/// `ext_name`: Files where its file name contains the extension name will be removed. The name must not starts with .
///
/// `dir_path`: The path where the directory is to be focused on.
pub fn remove_files(ext_name: &str, dir_path: &str) -> io::Result<()> {
  for entry in read_dir(dir_path)? {
    let entry = entry?;
    if entry.file_type()?.is_dir() {
      remove_files(ext_name, &entry.path().to_string_lossy())?;
    } else if entry
      .file_name()
      .to_string_lossy()
      .ends_with(&format!(".{ext_name}"))
    {
      remove_file(entry.path())?;
    }
  }
  Ok(())
}

/// It checks whether the directory path given has at least one file with an `ext_name` that is received from the
/// parameter.
///
/// `ext_name`: The extension name used during checking. The name must not start with .
///
/// `dir_path`: The path where the directory is to be focused on.
///
/// It returns true if there is at least one file with an `ext_name` that is coming from the parameter, false
///   otherwise.
pub fn has_file_with_ext(ext_name: &str, dir_path: &str) -> io::Result<bool> {
  for entry in read_dir(dir_path)? {
    if entry?
      .file_name()
      .to_string_lossy()
      .ends_with(&format!(".{ext_name}"))
    {
      return Ok(true);
    }
  }
  Ok(false)
}
