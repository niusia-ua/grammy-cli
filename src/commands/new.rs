use std::{fs, path};

use crate::utils;
use anyhow::Result;
use inquire::{Select, Text};

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

#[derive(PartialEq)]
enum ExistenceProcessing {
  Clear,
  Overwrite,
  Cancel,
}

impl ExistenceProcessing {
  fn from_choice(choice: &str) -> Self {
    match choice {
      "Clear the directory and continue" => ExistenceProcessing::Clear,
      "Ignore and continue (conflicting files will be overwritten)" => {
        ExistenceProcessing::Overwrite
      }
      "Cancel operation" => ExistenceProcessing::Cancel,
      _ => unreachable!(),
    }
  }
}

pub fn handler() -> Result<()> {
  let project_name = Text::new("Enter the project name:")
    .with_default("grammy-bot")
    .prompt()?;

  let path = utils::build_path(Some(project_name.clone()))?;
  let existence_procesing = match path.exists() {
    true => {
      let choice = Select::new(
        &format!(
          r#"The target directory "{}" already exists. Choose how to proceed:"#,
          project_name
        ),
        vec![
          "Clear the directory and continue",
          "Ignore and continue (conflicting files will be overwritten)",
          "Cancel operation",
        ],
      )
      .prompt()?;
      ExistenceProcessing::from_choice(choice)
    }
    false => ExistenceProcessing::Overwrite,
  };

  if existence_procesing == ExistenceProcessing::Cancel {
    println!("Operation cancelled.");
    return Ok(());
  }

  let template = Select::new("Select a template:", utils::get_known_templates()).prompt()?;

  action(ActionNewOptions {
    path,
    template,
    existence_procesing,
  })?;

  Ok(())
}

struct ActionNewOptions {
  path: path::PathBuf,
  template: String,
  existence_procesing: ExistenceProcessing,
}

fn action(opts: ActionNewOptions) -> Result<()> {
  println!("Scaffolding project in {}...", opts.path.display());

  if opts.existence_procesing == ExistenceProcessing::Clear {
    fs::remove_dir_all(&opts.path)?;
  }

  utils::copy(&opts.template, &opts.path)?;

  let project_name = opts.path.file_name().unwrap().to_str().unwrap();
  println!("Done. Now run:");
  println!("  cd {}", project_name);
  match Runtime::from_template(&opts.template)? {
    Runtime::Deno => println!("  deno task dev"),
    Runtime::NodeJS => {
      println!("  npm install");
      println!("  npm run dev");
    }
  };

  Ok(())
}
