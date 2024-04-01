use anyhow::{bail, Result};
use include_dir::{include_dir, Dir, DirEntry};
use inquire::{Select, Text};
use std::{env, fs, path};

static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/");

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

  let target_dir = get_target_dir(&project_name)?;
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

  let known_templates = TEMPLATES_DIR
    .dirs()
    .map(|dir| dir.path().to_str().unwrap())
    .collect::<Vec<_>>();
  let template = Select::new("Select the template:", known_templates).prompt()?;

  copy(template, &target_dir)?;

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

fn get_target_dir(project_name: &str) -> Result<path::PathBuf> {
  let cwd = env::current_dir()?;
  Ok(cwd.join(project_name))
}

fn copy(from: impl AsRef<path::Path>, to: impl AsRef<path::Path>) -> Result<()> {
  fs::create_dir_all(&to)?;
  let dir = TEMPLATES_DIR.get_dir(from).unwrap();
  for entry in dir.entries() {
    match entry {
      DirEntry::Dir(dir) => {
        let dir_name = dir.path().components().last().unwrap();
        copy(dir.path(), to.as_ref().join(dir_name))?
      }
      DirEntry::File(file) => {
        let file_name = file.path().file_name().unwrap().to_str().unwrap();
        fs::write(to.as_ref().join(file_name), file.contents())?
      }
    }
  }
  Ok(())
}
