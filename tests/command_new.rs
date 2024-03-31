use std::{fs, path};

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
  t.expect("Select the template:")?;
  t.select_line("Deno", 2)?;
  t.expect("Great! Now type the following commands in the terminal and get to work!")?;

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
  t.expect("Select the template:")?;
  t.select_line("Node.js", 2)?;
  t.expect("Great! Now type the following commands in the terminal and get to work!")?;

  assert!(path::Path::new("grammy-bot-nodejs/package.json").exists());

  common::clear_dir("grammy-bot-nodejs")?;

  Ok(())
}

#[test]
fn should_abort_if_overwriting_is_not_allowed() -> Result<()> {
  common::clear_dir("grammy-bot")?;
  fs::create_dir("grammy-bot")?;

  let cmd = create_cmd(Some(vec!["new"]))?;
  let mut t = terminal::Terminal::new(cmd)?;

  t.expect("Enter the project name:")?;
  t.send("grammy-bot")?;
  t.expect("The project already exists. Overwrite?")?;
  t.send("No")?;
  t.expect("Aborting...")?;

  common::clear_dir("grammy-bot")?;

  Ok(())
}

#[test]
fn should_overwrite() -> Result<()> {
  common::clear_dir("grammy-bot-overwrite")?;

  // Deno
  let cmd = create_cmd(Some(vec!["new"]))?;
  let mut tdeno = terminal::Terminal::new(cmd)?;
  tdeno.expect("Enter the project name:")?;
  tdeno.send("grammy-bot-overwrite")?;
  tdeno.expect("Select the template:")?;
  tdeno.select_line("Deno", 2)?;
  tdeno.expect("Great! Now type the following commands in the terminal and get to work!")?;

  // Node.js
  let cmd = create_cmd(Some(vec!["new"]))?;
  let mut tnode = terminal::Terminal::new(cmd)?;
  tnode.expect("Enter the project name:")?;
  tnode.send("grammy-bot-overwrite")?;
  tnode.expect("The project already exists. Overwrite?")?;
  tnode.send("Yes")?;
  tnode.expect("Select the template:")?;
  tnode.select_line("Node.js", 2)?;
  tnode.expect("Great! Now type the following commands in the terminal and get to work!")?;

  // Only conflicting files are overwritten, not the entire directory.
  // That is, the directory should now contain the configuration files of both runtimes.
  assert!(path::Path::new("grammy-bot-overwrite/deno.json").exists());
  assert!(path::Path::new("grammy-bot-overwrite/package.json").exists());

  common::clear_dir("grammy-bot-overwrite")?;

  Ok(())
}
