use anyhow::Result;
use assert_cmd::prelude::*;
use std::process::Command;

pub mod common;
pub mod terminal;

pub fn create_cmd(args: Option<Vec<&str>>) -> Result<Command> {
  let mut cmd = Command::cargo_bin("grammy-cli")?;
  if let Some(args) = args {
    cmd.args(args);
  }
  Ok(cmd)
}
