use crate::{common::grammy, utils};
use anyhow::Result;
use clap::Args;
use std::env;

#[derive(Debug, Args)]
pub struct InfoOptions {
  #[arg(short, long)]
  /// Relative path to the project directory.
  /// If not specified, the current working directory is used.
  path: Option<String>,
}

pub fn handler(opts: InfoOptions) -> Result<()> {
  let project_path = utils::build_path(opts.path)?;
  let grammy = grammy::get_grammy_info(&project_path)?;

  action(ActionInfoOptions { grammy });

  Ok(())
}

struct ActionInfoOptions {
  grammy: grammy::GrammyInfo,
}

fn action(opts: ActionInfoOptions) {
  println!("{}\n", grammy::ASCII_ART);
  println!("[System Information]");
  println!("  OS: {}, {}", env::consts::OS, env::consts::ARCH);
  println!("[grammY Information]");
  println!("  grammY: {}", opts.grammy.grammy_version);
  println!("  Bot API: {}", opts.grammy.bot_api_version);
  if !opts.grammy.plugins.is_empty() {
    println!("  Installed Plugins:");
    for plugin in opts.grammy.plugins {
      println!("    {}: {}", plugin.name, plugin.version);
    }
  } else {
    println!("  Installed Plugins: (no plugins installed)")
  }
}
