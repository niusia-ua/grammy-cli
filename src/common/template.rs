use anyhow::Result;
use include_dir::DirEntry;
use include_dir::{include_dir, Dir};
use std::{fs, path::Path};

static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/");

pub fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
  fs::create_dir_all(&to)?;
  let dir = TEMPLATES_DIR.get_dir(from).unwrap();
  for entry in dir.entries() {
    match entry {
      DirEntry::Dir(dir) => {
        let dir_name = dir.path().components().last().unwrap();
        copy(dir.path(), to.as_ref().join(dir_name))?
      }
      DirEntry::File(file) => {
        let file_name = file.path().file_name().unwrap().to_str().unwrap();
        fs::write(to.as_ref().join(file_name), file.contents())?
      }
    }
  }
  Ok(())
}

pub fn get_known_templates() -> Vec<String> {
  TEMPLATES_DIR
    .dirs()
    .map(|dir| dir.path().to_str().unwrap().to_string())
    .collect::<Vec<_>>()
}
