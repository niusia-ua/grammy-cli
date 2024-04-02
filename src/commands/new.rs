use crate::{constants, utils};
use anyhow::{bail, Result};
use inquire::{Select, Text};
use std::path;

enum Runtime {
  Deno,
  NodeJS,
}

impl Runtime {
  fn from_template(template_name: &str) -> Result<Self> {
    match template_name {
      "Deno" => Ok(Runtime::Deno),
      "Node.js" => Ok(Runtime::NodeJS),
      _ => bail!("Unknown template."),
    }
  }
}

pub fn command_new_action() -> Result<()> {
  let project_name = Text::new("Enter the project name:")
    .with_default("grammy-bot")
    .prompt()?;

  let target_dir = utils::build_path(Some(project_name.clone()))?;
  if path::Path::new(&target_dir).exists() {
    let overwrite_str =
      Select::new("The project already exists. Overwrite?", vec!["Yes", "No"]).prompt()?;
    match overwrite_str == "Yes" {
      true => (),
      false => {
        println!("Aborting...");
        return Ok(());
      }
    };
  }

  let known_templates = constants::TEMPLATES_DIR
    .dirs()
    .map(|dir| dir.path().to_str().unwrap())
    .collect::<Vec<_>>();
  let template = Select::new("Select the template:", known_templates).prompt()?;

  utils::copy(template, &target_dir)?;

  println!("Great! Now type the following commands in the terminal and get to work!");
  println!("cd {}/", project_name);
  match Runtime::from_template(template)? {
    Runtime::Deno => println!("deno task dev"),
    Runtime::NodeJS => {
      println!("npm install");
      println!("npm run dev");
    }
  };

  Ok(())
}
