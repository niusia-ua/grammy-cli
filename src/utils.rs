use anyhow::Result;
use std::{env, path};

pub fn build_path(path: Option<String>) -> Result<path::PathBuf> {
  let cwd = env::current_dir()?;
  match path {
    Some(path) => Ok(cwd.join(path)),
    None => Ok(cwd),
  }
}
