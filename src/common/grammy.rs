use crate::common::{config, version};
use anyhow::Result;
use std::path;

// http://patorjk.com/software/taag/#p=display&h=0&f=Speed&t=grammY
pub const ASCII_ART: &str = r#"                                                 __  __
_______ _______________ ________ ___ _______ ___ _ \/ /
__  __ `/__  ___/_  __ `/__  __ `__ \__  __ `__ \__  /
_  /_/ / _  /    / /_/ / _  / / / / /_  / / / / /_  /
_\__, /  /_/     \__,_/  /_/ /_/ /_/ /_/ /_/ /_/ /_/
/____/"#;

pub struct GrammyInfo {
  pub grammy_version: String,
  pub bot_api_version: String,
  pub plugins: Vec<PluginInfo>,
}

pub struct PluginInfo {
  pub name: String,
  pub version: String,
}

pub fn get_grammy_info(root: &path::Path) -> Result<GrammyInfo> {
  let config = config::PackageConfig::new(root)?;
  let deps = config.deps();
  let grammy_version = version::clear_semver(deps.get("grammy").unwrap().as_str().unwrap());
  let plugins = deps
    .iter()
    .filter(|(k, _v)| k.starts_with("@grammyjs/"))
    .map(|(name, version)| PluginInfo {
      name: name.to_string().replace("@grammyjs/", ""),
      version: version::clear_semver(version.as_str().unwrap()),
    })
    .collect();
  let bot_api_version = version::get_bot_api_version(&semver::Version::parse(&grammy_version)?)?;
  Ok(GrammyInfo {
    grammy_version,
    bot_api_version,
    plugins,
  })
}
