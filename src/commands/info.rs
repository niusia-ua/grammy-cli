use anyhow::{bail, Result};
use clap::Args;
use std::{env, fs, path};

type Object = serde_json::Map<String, serde_json::Value>;

// http://patorjk.com/software/taag/#p=display&h=0&f=Speed&t=grammY
const GRAMMY_ASCII_ART: &str = r#"                                                 __  __
_______ _______________ ________ ___ _______ ___ _ \/ /
__  __ `/__  ___/_  __ `/__  __ `__ \__  __ `__ \__  /
_  /_/ / _  /    / /_/ / _  / / / / /_  / / / / /_  /
_\__, /  /_/     \__,_/  /_/ /_/ /_/ /_/ /_/ /_/ /_/
/____/
"#;

#[derive(Debug, Args)]
pub struct InfoOptions {
  #[arg(short, long)]
  /// Relative path to the project directory.
  /// If not specified, the current working directory is used.
  path: Option<String>,
}

pub fn command_info_action(args: InfoOptions) -> Result<()> {
  let project_path = get_project_path(args.path)?;
  let project_deps = get_project_deps(&project_path)?;
  let grammy_info = get_grammy_info(&project_deps)?;

  println!("{}", GRAMMY_ASCII_ART);
  println!("[System Information]");
  println!("  OS: {}, {}", env::consts::OS, env::consts::ARCH);
  println!("[grammY Informaion]");
  println!("  grammY Version: {}", grammy_info.version);
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
  version: String,
  // bot_api_version: String,
  plugins: Vec<PluginInfo>,
}

struct PluginInfo {
  name: String,
  version: String,
}

const KNOWN_CONFIG_FILES: [&str; 3] = ["deno.json", "deno.jsonc", "package.json"];

fn get_grammy_info(deps: &Object) -> Result<GrammyInfo> {
  let grammy_version = deps.get("grammy").unwrap().to_string();
  // TODO: Add semver and URL parsing
  let plugins = deps
    .iter()
    .filter(|(k, _v)| k.starts_with("@grammyjs/"))
    .map(|(name, version)| PluginInfo {
      name: name.to_string(),
      version: version.to_string(),
    })
    .collect();
  Ok(GrammyInfo {
    version: grammy_version,
    plugins,
  })
}

fn get_project_path(path: Option<String>) -> Result<path::PathBuf> {
  let cwd = env::current_dir()?;
  match path {
    Some(path) => Ok(cwd.join(path)),
    None => Ok(cwd),
  }
}

fn get_project_deps(project_path: &path::Path) -> Result<Object> {
  let config_path = KNOWN_CONFIG_FILES
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
    KNOWN_CONFIG_FILES.join(", "),
    project_path.display()
  );
}
