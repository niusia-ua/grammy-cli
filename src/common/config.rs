use super::runtime::Runtime;
use anyhow::{bail, Result};
use std::{fs, path};

const KNOWN_CONFIG_FILES: [&str; 3] = ["deno.json", "deno.jsonc", "package.json"];

pub type Object = serde_json::Map<String, serde_json::Value>;

pub struct PackageConfig {
  runtime: Runtime,
  content: Object,
}

impl PackageConfig {
  pub fn new(root: &path::Path) -> Result<Self> {
    let config_file = KNOWN_CONFIG_FILES
      .iter()
      .find(|file| root.join(file).exists());
    if let Some(config_file) = config_file {
      let config_path = root.join(config_file);
      let content = fs::read_to_string(config_path)?;
      let config: serde_json::Value = serde_json::from_str(&content)?;
      return Ok(PackageConfig {
        runtime: Runtime::from_config_file(config_file),
        content: config.as_object().unwrap().to_owned(),
      });
    }

    bail!(
      r#"Could not find any known configuration file ({}) in "{}"."#,
      KNOWN_CONFIG_FILES.join(", "),
      root.display()
    );
  }

  pub fn deps(&self) -> &Object {
    let key = match self.runtime {
      Runtime::Deno => "imports",
      Runtime::NodeJS => "dependencies",
    };
    self.content.get(key).unwrap().as_object().unwrap()
  }
}
