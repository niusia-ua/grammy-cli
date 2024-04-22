mod commands;
mod common;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help = true, before_help = common::grammy::ASCII_ART)]
struct Cli {
  #[command(subcommand)]
  command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
  /// Generate grammY project
  New(commands::new::NewOptions),

  /// Display project information, installed plugins, and other useful system information
  Info(commands::info::InfoOptions),
}

fn main() -> Result<()> {
  let cli = Cli::parse();

  match cli.command {
    Command::New(opts) => commands::new::handler(opts)?,
    Command::Info(opts) => commands::info::handler(opts)?,
  };

  Ok(())
}
