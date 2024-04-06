use std::path;

use anyhow::Result;

mod utils;
use utils::{common, create_cmd, terminal};

#[test]
fn should_setup_deno_project() -> Result<()> {
  common::clear_dir("grammy-bot-deno")?;

  let cmd = create_cmd(Some(vec!["new"]))?;
  let mut t = terminal::Terminal::new(cmd)?;

  t.expect("Enter the project name:")?;
  t.send("grammy-bot-deno")?;
  t.expect("Select a template:")?;
  t.select_line("Deno", 2)?;
  t.expect("Done. Now run:")?;

  assert!(path::Path::new("grammy-bot-deno/deno.json").exists());

  common::clear_dir("grammy-bot-deno")?;

  Ok(())
}

#[test]
fn should_setup_nodejs_project() -> Result<()> {
  common::clear_dir("grammy-bot-nodejs")?;

  let cmd = create_cmd(Some(vec!["new"]))?;
  let mut t = terminal::Terminal::new(cmd)?;

  t.expect("Enter the project name:")?;
  t.send("grammy-bot-nodejs")?;
  t.expect("Select a template:")?;
  t.select_line("Node.js", 2)?;
  t.expect("Done. Now run:")?;

  assert!(path::Path::new("grammy-bot-nodejs/package.json").exists());

  common::clear_dir("grammy-bot-nodejs")?;

  Ok(())
}
