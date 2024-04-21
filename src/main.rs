mod commands;
mod constants;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::info::InfoOptions;

#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help = true, before_help = constants::GRAMMY_ASCII_ART)]
struct Cli {
  #[command(subcommand)]
  command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
  /// Generate grammY project
  New,

  /// Display project information, installed plugins, and other useful system information
  Info(InfoOptions),
}

fn main() -> Result<()> {
  let cli = Cli::parse();

  match cli.command {
    Command::New => commands::new::handler()?,
    Command::Info(opts) => commands::info::handler(opts)?,
  };

  Ok(())
}
