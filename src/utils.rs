use crate::constants;
use anyhow::Result;
use include_dir::DirEntry;
use std::{env, fs, path};

pub fn build_path(path: Option<String>) -> Result<path::PathBuf> {
  let cwd = env::current_dir()?;
  match path {
    Some(path) => Ok(cwd.join(path)),
    None => Ok(cwd),
  }
}

pub fn clear_semver(version: &str) -> String {
  if version.starts_with("https://deno.land/x/") {
    extract_semver_from_deno_x_url(version)
  } else {
    version.replace(['=', '>', '<', '^', '~'], "")
  }
}

pub fn extract_semver_from_deno_x_url(url: &str) -> String {
  let package = url.split('/').find(|s| s.starts_with("grammy")).unwrap();
  let version = package.split('@').last().unwrap();
  version.replace('v', "")
}

pub fn copy(from: impl AsRef<path::Path>, to: impl AsRef<path::Path>) -> Result<()> {
  fs::create_dir_all(&to)?;
  let dir = constants::TEMPLATES_DIR.get_dir(from).unwrap();
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
