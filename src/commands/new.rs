use std::{fs, path};

use crate::{
  common::{existence::ExistenceProcessing, runtime::Runtime, template},
  utils,
};
use anyhow::Result;
use clap::Args;
use inquire::{Select, Text};

#[derive(Debug, Args)]
pub struct NewOptions {
  #[arg(short, long)]
  /// Go through the process of creating a project, but do not actually perform any operations with the file system
  dry_run: bool,
}

pub fn handler(opts: NewOptions) -> Result<()> {
  let project_name = Text::new("Enter the project name:")
    .with_default("grammy-bot")
    .prompt()?;

  let path = utils::build_path(Some(project_name.clone()))?;
  let existence_processing = match path.exists() {
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

  if existence_processing == ExistenceProcessing::Cancel {
    println!("Operation cancelled.");
    return Ok(());
  }

  let template = Select::new("Select a template:", template::get_known_templates()).prompt()?;

  action(ActionNewOptions {
    path,
    template,
    existence_processing,
    dry_run: opts.dry_run,
  })?;

  Ok(())
}

struct ActionNewOptions {
  path: path::PathBuf,
  template: String,
  existence_processing: ExistenceProcessing,
  dry_run: bool,
}

fn action(opts: ActionNewOptions) -> Result<()> {
  println!("Scaffolding project in {}...", opts.path.display());

  if !opts.dry_run {
    if opts.existence_processing == ExistenceProcessing::Clear {
      fs::remove_dir_all(&opts.path)?;
    }

    template::copy(&opts.template, &opts.path)?;
  }

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
