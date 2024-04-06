use anyhow::Result;

mod utils;
use utils::{common, create_cmd, terminal};

#[test]
fn should_display_grammy_info() -> Result<()> {
  common::clear_dir("grammy-bot-info")?;

  // Creating a dummy project
  let cmd = create_cmd(Some(vec!["new"]))?;
  let mut t = terminal::Terminal::new(cmd)?;

  t.expect("Enter the project name:")?;
  t.send("grammy-bot-info")?;
  t.expect("Select a template:")?;
  t.select_line("Node.js", 2)?;
  t.expect("Done. Now run:")?;

  // Getting information about the project
  let cmd = create_cmd(Some(vec!["info", "--path", "grammy-bot-info"]))?;
  let mut t = terminal::Terminal::new(cmd)?;

  t.expect("[grammY Information]")?;
  t.expect("grammY: 1.21.1")?;
  t.expect("Bot API: 7.1")?;
  t.expect("Installed Plugins: (no plugins installed)")?;

  common::clear_dir("grammy-bot-info")?;

  Ok(())
}
