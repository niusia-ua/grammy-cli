use crate::{constants, utils};
use anyhow::{bail, Result};
use clap::Args;
use semver::{Version, VersionReq};
use std::{env, fs, path};

type Object = serde_json::Map<String, serde_json::Value>;

#[derive(Debug, Args)]
pub struct InfoOptions {
  #[arg(short, long)]
  /// Relative path to the project directory.
  /// If not specified, the current working directory is used.
  path: Option<String>,
}

pub fn command_info_action(args: InfoOptions) -> Result<()> {
  let project_path = utils::build_path(args.path)?;
  let project_deps = get_project_deps(&project_path)?;
  let grammy_info = get_grammy_info(&project_deps)?;

  println!("{}\n", constants::GRAMMY_ASCII_ART);
  println!("[System Information]");
  println!("  OS: {}, {}", env::consts::OS, env::consts::ARCH);
  println!("[grammY Information]");
  println!("  grammY: {}", grammy_info.grammy_version);
  println!("  Bot API: {}", grammy_info.bot_api_version);
  if !grammy_info.plugins.is_empty() {
    println!("  Installed Plugins:");
    for plugin in grammy_info.plugins {
      println!("    {}: {}", plugin.name, plugin.version);
    }
  } else {
    println!("  Installed Plugins: (no plugins installed)")
  }

  Ok(())
}

struct GrammyInfo {
  grammy_version: String,
  bot_api_version: String,
  plugins: Vec<PluginInfo>,
}

struct PluginInfo {
  name: String,
  version: String,
}

fn get_grammy_info(deps: &Object) -> Result<GrammyInfo> {
  let grammy_version = utils::clear_semver(deps.get("grammy").unwrap().as_str().unwrap());
  let plugins = deps
    .iter()
    .filter(|(k, _v)| k.starts_with("@grammyjs/"))
    .map(|(name, version)| PluginInfo {
      name: name.to_string().replace("@grammyjs/", ""),
      version: utils::clear_semver(version.as_str().unwrap()),
    })
    .collect();
  let bot_api_version = get_bot_api_version(&Version::parse(&grammy_version)?)?;
  Ok(GrammyInfo {
    grammy_version,
    bot_api_version,
    plugins,
  })
}

fn get_project_deps(project_path: &path::Path) -> Result<Object> {
  let config_path = constants::KNOWN_CONFIG_FILES
    .iter()
    .find(|file| path::Path::new(&project_path.join(file)).exists());
  if let Some(config_path) = config_path {
    let content = fs::read_to_string(project_path.join(config_path))?;
    let config: serde_json::Value = serde_json::from_str(&content)?;
    let deps_key = match path::Path::new(config_path)
      .file_stem()
      .unwrap()
      .to_str()
      .unwrap()
    {
      "deno" => "imports",
      "package" => "dependencies",
      _ => unreachable!("Expected deno.json[c] or package.json, but got something else"),
    };
    return Ok(config[deps_key].as_object().unwrap().to_owned());
  }

  bail!(
    "Could not find any known configuration files ({}) in {}.",
    constants::KNOWN_CONFIG_FILES.join(", "),
    project_path.display()
  );
}

fn get_bot_api_version(grammy_version: &Version) -> Result<String> {
  let result = constants::KNOWN_GRAMMY_VERSION_MATCHES_WITH_BOT_API
    .iter()
    .find(|(req, _)| VersionReq::parse(req).unwrap().matches(grammy_version));
  match result {
    Some((_, bot_api_version)) => Ok(bot_api_version.to_string()),
    None => Ok(String::from("Unknown")),
  }
}
