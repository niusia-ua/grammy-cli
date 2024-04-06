use crate::{constants, utils};
use anyhow::Result;
use inquire::{Select, Text};
use std::{fs, path};

enum Runtime {
  Deno,
  NodeJS,
}

impl Runtime {
  fn from_template(template_name: &str) -> Result<Self> {
    match template_name {
      "Deno" => Ok(Runtime::Deno),
      "Node.js" => Ok(Runtime::NodeJS),
      _ => unreachable!("Unknown template."),
    }
  }
}

pub fn command_new_action() -> Result<()> {
  let project_name = Text::new("Enter the project name:")
    .with_default("grammy-bot")
    .prompt()?;

  let target_dir = utils::build_path(Some(project_name.clone()))?;
  if path::Path::new(&target_dir).exists() {
    let handling = Select::new(
      &format!(
        "The target directory \"{}\" already exists. Choose how to proceed:",
        project_name
      ),
      vec![
        "Clear the directory and continue",
        "Ignore and continue (conflicting files will be overwritten)",
        "Cancel operation",
      ],
    )
    .prompt()?;
    match handling {
      "Clear the directory and continue" => fs::remove_dir_all(&target_dir)?,
      "Ignore and continue (conflicting files will be overwritten)" => (),
      "Cancel operation" => {
        println!("Operation canceled");
        return Ok(());
      }
      _ => unreachable!(),
    };
  }

  let known_templates = constants::TEMPLATES_DIR
    .dirs()
    .map(|dir| dir.path().to_str().unwrap())
    .collect::<Vec<_>>();
  let template = Select::new("Select a template:", known_templates).prompt()?;

  println!("Scaffolding project in {}...", target_dir.display());
  utils::copy(template, &target_dir)?;

  println!("Done. Now run:");
  println!("  cd {}", project_name);
  match Runtime::from_template(template)? {
    Runtime::Deno => println!("  deno task dev"),
    Runtime::NodeJS => {
      println!("  npm install");
      println!("  npm run dev");
    }
  };

  Ok(())
}
