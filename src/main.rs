mod commands;
mod constants;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{
  info::{command_info_action, InfoOptions},
  new::command_new_action,
};

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
    Command::New => command_new_action()?,
    Command::Info(args) => command_info_action(args)?,
  };

  Ok(())
}
