use anyhow::Result;
use std::{fs, path};

pub fn clear_dir(dir: &str) -> Result<()> {
  if path::Path::new(dir).exists() {
    fs::remove_dir_all(dir)?;
  }
  Ok(())
}
