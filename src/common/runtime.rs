use anyhow::Result;

pub enum Runtime {
  Deno,
  NodeJS,
}

impl Runtime {
  pub fn from_template(template_name: &str) -> Result<Self> {
    match template_name {
      "Deno" => Ok(Runtime::Deno),
      "Node.js" => Ok(Runtime::NodeJS),
      template => unreachable!(r#"Unknown template "{}"."#, template),
    }
  }

  pub fn from_config_file(config_file: &str) -> Self {
    match config_file {
      "deno.json" | "deno.jsonc" => Runtime::Deno,
      "package.json" => Runtime::NodeJS,
      config_file => unreachable!(r#"Unknown config file "{}"."#, config_file),
    }
  }
}
