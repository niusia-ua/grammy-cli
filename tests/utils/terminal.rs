use anyhow::Result;
use rexpect::session::{spawn_command, PtySession};
use std::process::Command;

// Adapted with some changes from https://github.com/Kobzol/cargo-wizard/blob/main/tests/integration/utils/terminal.rs
pub struct Terminal {
  session: PtySession,
}

impl Terminal {
  pub fn new(cmd: Command) -> Result<Terminal> {
    let session = spawn_command(cmd, Some(1000))?;
    Ok(Terminal { session })
  }

  pub fn expect(&mut self, text: &str) -> Result<()> {
    self.session.exp_string(text)?;
    Ok(())
  }

  pub fn send(&mut self, text: &str) -> Result<()> {
    self.session.send_line(text)?;
    Ok(())
  }

  pub fn select_line(&mut self, prefix: &str, list_length: u8) -> Result<()> {
    let regex = format!(">.*{prefix}");
    for _ in 0..list_length {
      if self.session.exp_regex(&regex).is_ok() {
        return self.enter();
      }
      self.down()?;
    }
    eprintln!("Could not find line beginning with {prefix}.");
    // Print terminal output
    let msg = format!("<missing {prefix} in list>");
    self.session.exp_string(&msg)?;
    unreachable!();
  }

  pub fn enter(&mut self) -> Result<()> {
    self.session.send_line("")?;
    Ok(())
  }

  fn down(&mut self) -> Result<()> {
    // Arrow down, detected through `showkey -a`
    self.session.send("\x1b\x5b\x42")?;
    self.session.flush()?;
    Ok(())
  }
}
